use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::io::Read;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct DogImage {
    message: String, // image URL
    status: String,
}

#[derive(Debug)]
enum ApiResult {
    Success(DogImage),
    ApiError(String),
    NetworkError(String),
}

fn fetch_random_dog_image() -> ApiResult {
    let url = "https://dog.ceo/api/breeds/image/random";

    match ureq::get(url).call() {
        Ok(response) => {
            if response.status() == 200 {
                match response.into_json::<DogImage>() {
                    Ok(dog_image) => ApiResult::Success(dog_image),
                    Err(e) => ApiResult::ApiError(format!("Failed to parse JSON: {}", e)),
                }
            } else {
                ApiResult::ApiError(format!("HTTP error: {}", response.status()))
            }
        }
        Err(e) => ApiResult::NetworkError(format!("Request failed: {}", e)),
    }
}

fn download_and_save(url: &str, dir: &str, index: usize) -> Result<String, Box<dyn Error>> {
    fs::create_dir_all(dir)?;

    let clean = url.split('?').next().unwrap_or(url);
    let clean = clean.split('#').next().unwrap_or(clean);
    let ext = clean
        .rsplit('.')
        .next()
        .filter(|e| e.len() <= 5 && e.chars().all(|c| c.is_ascii_alphanumeric()))
        .unwrap_or("jpg");

    let filename = format!("dog_{:02}.{}", index, ext);
    let path = Path::new(dir).join(filename);

    let resp = ureq::get(url).call()?;
    if resp.status() != 200 {
        return Err(format!("Image HTTP error: {}", resp.status()).into());
    }

    let mut bytes = Vec::new();
    let mut reader = resp.into_reader();
    reader.read_to_end(&mut bytes)?;
    fs::write(&path, &bytes)?;

    Ok(path.display().to_string())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Dog Image Fetcher");
    println!("=================\n");

    for i in 1..=5 {
        println!("Fetching random dog image #{}", i);
        match fetch_random_dog_image() {
            ApiResult::Success(dog_image) => {
                println!("✅ Success!");
                println!("🖼️ Image URL: {}", dog_image.message);

                match download_and_save(&dog_image.message, "dogs", i) {
                    Ok(saved_path) => println!("💾 Saved to: {}", saved_path),
                    Err(e) => println!("❌ Save error: {}", e),
                }

                println!("📊 API Status: {}", dog_image.status);
            }
            ApiResult::ApiError(e) => println!("❌ API Error: {}", e),
            ApiResult::NetworkError(e) => println!("❌ Network Error: {}", e),
        }
        println!();
    }

    println!("Done. Files are in the ./dogs folder.");
    Ok(())
}
