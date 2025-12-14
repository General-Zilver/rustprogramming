use std::thread;
use std::sync::{Arc, Mutex, mpsc};
use std::sync::atomic::{AtomicBool, Ordering};
use std::collections::HashMap;
use std::time::Duration;
use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::time::Instant;

type Job = Box<dyn FnOnce() + Send + 'static>;

// FIX: Added Status tracking for "Per-file processing status" requirement
#[derive(Debug, Clone, PartialEq)]
enum FileStatus {
    Queued,
    Processing,
    Completed,
    Failed,
}

#[derive(Debug, Clone)]
enum ProcessingError {
    IoError(String), // Changed to String to make it Clone-able easily
    DirectoryError(String),
}

// FIX: Return BOTH files and errors so we can report directory failures in the final output
fn get_all_files(dir: &Path) -> (Vec<PathBuf>, Vec<ProcessingError>) {
    let mut files = Vec::new();
    let mut errors = Vec::new();
    
    if dir.is_dir() {
        match fs::read_dir(dir) {
            Ok(entries) => {
                for entry in entries {
                    match entry {
                        Ok(entry) => {
                            let path = entry.path();
                            if path.is_dir() {
                                let (sub_files, sub_errors) = get_all_files(&path);
                                files.extend(sub_files);
                                errors.extend(sub_errors);
                            } else {
                                files.push(path);
                            }
                        }
                        Err(e) => errors.push(ProcessingError::DirectoryError(format!("Entry error in {:?}: {}", dir, e))),
                    }
                }
            }
            Err(e) => errors.push(ProcessingError::DirectoryError(format!("Cannot read dir {:?}: {}", dir, e))),
        }
    }
    (files, errors)
}

struct FileAnalysis {
    filename: String,
    stats: FileStats,
    errors: Vec<ProcessingError>,
    processing_time: Duration,
}

#[derive(Debug, Clone)]
struct FileStats {
    word_count: usize,
    line_count: usize,
    char_frequencies: HashMap<char, usize>,
    size_bytes: u64,
}

// FIX: Added Status Tracker
struct ThreadPool {
    workers: Vec<Worker>,
    sender: Mutex<Option<mpsc::Sender<Job>>>, 
    cancel_flag: Arc<AtomicBool>,
    status_tracker: Arc<Mutex<HashMap<String, FileStatus>>>,
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        let cancel_flag = Arc::new(AtomicBool::new(false));
        let status_tracker = Arc::new(Mutex::new(HashMap::new()));

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver), Arc::clone(&cancel_flag)));
        }

        ThreadPool {
            workers,
            sender: Mutex::new(Some(sender)), // CHANGED: Wrapped in Mutex/Some
            cancel_flag,
            status_tracker,
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        // CHANGED: Lock mutex, check if sender exists, then send
        if let Ok(guard) = self.sender.lock() {
            if let Some(sender) = &*guard {
                let _ = sender.send(job);
            }
        }
    }

    pub fn cancellation_token(&self) -> Arc<AtomicBool> {
        Arc::clone(&self.cancel_flag)
    }

    pub fn update_status_internal(tracker: &Arc<Mutex<HashMap<String, FileStatus>>>, filename: String, status: FileStatus) {
        if let Ok(mut map) = tracker.lock() {
            map.insert(filename, status);
        }
    }

    // CHANGED: This now sets the flag AND closes the channel
    pub fn cancel(&self) {
        self.cancel_flag.store(true, Ordering::Relaxed);
        
        // Drop the sender. This stops new jobs and disconnects workers.
        if let Ok(mut guard) = self.sender.lock() {
            *guard = None;
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // CHANGED: Explicitly drop the sender to close channel
        if let Ok(mut guard) = self.sender.lock() {
            *guard = None;
        }

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                let _ = thread.join();
            }
        }
    }
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>, cancel_flag: Arc<AtomicBool>) -> Worker {
        let thread = thread::spawn(move || loop {
            
            // CHANGED: If cancelled, drain the queue to prevent deadlocks
            if cancel_flag.load(Ordering::Relaxed) {
                if let Ok(lock) = receiver.lock() {
                   // Keep pulling items off the belt and dropping them until empty
                   while let Ok(_) = lock.try_recv() {}
                }
                break;
            }

            // Normal operation
            let job_option = {
                if let Ok(lock) = receiver.lock() {
                    match lock.try_recv() {
                        Ok(job) => Some(job),
                        Err(mpsc::TryRecvError::Empty) => None,
                        Err(mpsc::TryRecvError::Disconnected) => return,
                    }
                } else {
                    return; 
                }
            };

            match job_option {
                Some(job) => {
                    if cancel_flag.load(Ordering::Relaxed) { continue; }
                    job();
                }
                None => {
                    thread::sleep(Duration::from_millis(10));
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

fn process_file(path: PathBuf) -> FileAnalysis {
    let start_time = Instant::now();
    let mut stats = FileStats {
        word_count: 0,
        line_count: 0,
        char_frequencies: HashMap::new(),
        size_bytes: 0,
    };
    let mut errors = Vec::new();

    // FIX: Use metadata for size (handles non-UTF8 correctness)
    if let Ok(metadata) = fs::metadata(&path) {
        stats.size_bytes = metadata.len();
    }

    // FIX: Read as bytes first, then Lossy Convert.
    // This allows "Bonus: different file encodings" (it won't crash on binary/ISO-8859-1)
    let content_result = File::open(&path).and_then(|mut f| {
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).map(|_| buffer)
    });

    match content_result {
        Ok(bytes) => {
            // "lossy" means if it finds invalid UTF8, it inserts  instead of crashing
            let content = String::from_utf8_lossy(&bytes);
            
            stats.line_count = content.lines().count();
            stats.word_count = content.split_whitespace().count();

            for c in content.chars() {
                if !c.is_whitespace() {
                    *stats.char_frequencies.entry(c).or_insert(0) += 1;
                }
            }
        }
        Err(e) => {
            errors.push(ProcessingError::IoError(e.to_string()));
        }
    }

    FileAnalysis {
        filename: path.to_string_lossy().to_string(),
        stats,
        errors,
        processing_time: start_time.elapsed(),
    }
}

fn main() {
    // Wrap pool in Arc so we can share it with the listener
    let pool = Arc::new(ThreadPool::new(8));
    let (tx, rx) = mpsc::channel();

    // --- INTERACTIVE DIRECTORY INPUT ---
    let mut directories = Vec::new();
    println!("--- Parallel File Processor ---");
    println!("Enter directory paths to scan.");
    println!("Type 'done' or just press Enter when you are finished adding folders.");

    loop {
        print!("Please add a Directory (Just press enter or type 'done' when finished): ");
        // Flush stdout so the ">" prompt appears immediately
        let _ = io::stdout().flush(); 
        
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let path_str = input.trim();
                if path_str.is_empty() || path_str.eq_ignore_ascii_case("done") {
                    break;
                }
                
                // Optional: Check if path exists before adding
                let path = PathBuf::from(path_str);
                if path.exists() && path.is_dir() {
                    directories.push(path);
                    println!("Added: {:?}", path_str);
                } else {
                    println!("Warning: '{}' is not a valid directory. Ignored.", path_str);
                }
            }
            Err(error) => println!("Error reading input: {}", error),
        }
    }

    if directories.is_empty() {
        println!("No directories selected. Exiting.");
        return;
    }
    // -----------------------------------

    let mut files = Vec::new();
    let mut scan_errors = Vec::new();

    // Iterate over the user-provided directories
    for root_path in directories {
        println!("Scanning directory: {:?}", root_path);
        let (dir_files, dir_errors) = get_all_files(&root_path);
        files.extend(dir_files);
        scan_errors.extend(dir_errors);
    }

    let total_found = files.len();
    
    println!("Found {} files to process.", total_found);

    let tracker = Arc::clone(&pool.status_tracker);

    for path in files {
        let tx = tx.clone();
        let tracker = Arc::clone(&tracker);
        let p_str = path.to_string_lossy().to_string();
        
        ThreadPool::update_status_internal(&pool.status_tracker, p_str.clone(), FileStatus::Queued);

        let p_clone = p_str.clone();
        pool.execute(move || {
            ThreadPool::update_status_internal(&tracker, p_clone.clone(), FileStatus::Processing);
            let result = process_file(path);
            
            let final_status = if result.errors.is_empty() {
                FileStatus::Completed
            } else {
                FileStatus::Failed
            };
            ThreadPool::update_status_internal(&tracker, p_clone, final_status);

            let _ = tx.send(result);
        });
    }

    // CHANGED: Listener now has access to the full pool to call cancel()
    let pool_for_cancel = Arc::clone(&pool);
    thread::spawn(move || {
        println!("Press ENTER at any time to cancel processing...");
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            println!("Cancelling...");
            pool_for_cancel.cancel(); // This now properly closes the channel
        }
    });

    drop(tx); 

    let mut global_stats = FileStats {
        word_count: 0,
        line_count: 0,
        char_frequencies: HashMap::new(),
        size_bytes: 0,
    };

    let mut processed_files_count = 0;
    let mut total_duration = Duration::new(0, 0);

    for analysis in rx {
        processed_files_count += 1;
        print!("\rProgress: {}/{} files processed...", processed_files_count, total_found);
        let _ = io::stdout().flush();

        if !analysis.errors.is_empty() {
             println!("\nError processing {}: {:?}", analysis.filename, analysis.errors);
        }

        global_stats.word_count += analysis.stats.word_count;
        global_stats.line_count += analysis.stats.line_count;
        global_stats.size_bytes += analysis.stats.size_bytes;
        
        for (char, count) in analysis.stats.char_frequencies {
            *global_stats.char_frequencies.entry(char).or_insert(0) += count;
        }
        total_duration += analysis.processing_time;
    }
    
    for err in scan_errors {
        println!("\nDirectory Error: {:?}", err);
    }

    println!("\n------------------------------------------------");
    if processed_files_count < total_found {
        println!("Process Cancelled Early!");
    }
    println!("Final Report for {} files:", processed_files_count);
    println!("Total Size: {} bytes", global_stats.size_bytes);
    println!("Total Processing Time: {:.2?}", total_duration);
    if processed_files_count > 0 {
        println!("Average Time per File: {:.2?}", total_duration / processed_files_count as u32);
    }
    println!("Total Lines: {}", global_stats.line_count);
    println!("Total Words: {}", global_stats.word_count);
    println!("Top 10 Characters:");
    let mut chars: Vec<_> = global_stats.char_frequencies.iter().collect();
    // Sort by count (descending)
    chars.sort_by(|a, b| b.1.cmp(a.1)); 
    
    for (char, count) in chars.into_iter().take(10) {
        println!("  '{}': {}", char, count);
    }
}

// FIX: Full Testing Suite
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;

    // Unit Test: Thread Pool
    #[test]
    fn test_thread_pool_execution() {
        let pool = ThreadPool::new(4);
        let (tx, rx) = mpsc::channel();

        for i in 0..10 {
            let tx = tx.clone();
            pool.execute(move || {
                let _ = tx.send(i);
            });
        }
        drop(tx);

        let mut sum = 0;
        for num in rx {
            sum += num;
        }
        assert_eq!(sum, 45); 
    }

    // Integration Test: File Processing (Creates real files)
    #[test]
    fn test_integration_file_processing() {
        let dir = Path::new("test_output");
        let _ = fs::create_dir(dir);
        let file1 = dir.join("t1.txt");
        let file2 = dir.join("t2.txt");
        {
            let mut f1 = File::create(&file1).unwrap();
            writeln!(f1, "hello world").unwrap(); 
            let mut f2 = File::create(&file2).unwrap();
            writeln!(f2, "rust is great").unwrap(); 
        }
        
        let (mut files, _) = get_all_files(dir);
        // CHANGED: Sort files to ensure deterministic order (t1 then t2)
        files.sort(); 

        let analysis1 = process_file(files[0].clone());
        let analysis2 = process_file(files[1].clone());

        // Cleanup
        let _ = fs::remove_file(file1);
        let _ = fs::remove_file(file2);
        let _ = fs::remove_dir(dir);

        assert_eq!(analysis1.stats.word_count + analysis2.stats.word_count, 5);
    }

    // Benchmark (Simple)
    #[test]
    fn benchmark_large_processing() {
        let start = Instant::now();
        // Just simulating math load, not file IO for stability
        let pool = ThreadPool::new(4);
        let (tx, rx) = mpsc::channel();
        
        for _ in 0..1000 {
            let tx = tx.clone();
            pool.execute(move || {
                let mut x = 0;
                for i in 0..1000 { x += i; }
                let _ = tx.send(x);
            });
        }
        drop(tx);
        
        let _count = rx.iter().count();
        let duration = start.elapsed();
        
        println!("Benchmark: Processed 1000 tasks in {:?}", duration);
        // Assert it's reasonably fast (under 1 second)
        assert!(duration < Duration::from_secs(1));
    }
}