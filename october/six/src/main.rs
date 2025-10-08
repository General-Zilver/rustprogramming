


use std::arch::asm;

fn main() {
    let message = b"Hello, my name is John Payes\n";
 
    unsafe {
        // write syscall
        asm!(
            "mov rax, 1",  // syscall number for write
            "mov rdi, 1",  // file descriptor: 1 is stdout
            "syscall",
            in("rsi") message.as_ptr(),
            in("rdx") message.len(),
            out("rax") _,
            out("rcx") _,
            out("r11") _,
            clobber_abi("system")
        );

        // exit syscall
        asm!(
            "mov rax, 60", // syscall number for exit
            "xor rdi, rdi", // status code 0
            "syscall",
            options(noreturn)
        );
    }
}