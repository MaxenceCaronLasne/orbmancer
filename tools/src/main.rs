use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
use std::env;

fn main() {
    let project_root = env::current_dir().expect("Failed to get current directory");
    
    println!("🔨 Building GBA ROM with debug symbols...");
    
    // Build the project with debug symbols
    let build_result = Command::new("cargo")
        .arg("build")
        .arg("--target")
        .arg("thumbv4t-none-eabi")
        .current_dir(&project_root)
        .status()
        .expect("Failed to execute cargo build");

    if !build_result.success() {
        eprintln!("❌ Build failed!");
        std::process::exit(1);
    }

    println!("✅ Build successful!");

    // Find the built files
    let target_dir = project_root.join("target/thumbv4t-none-eabi/debug");
    let elf_file = target_dir.join("peg-gle");

    if !elf_file.exists() {
        eprintln!("❌ ELF file not found: {}", elf_file.display());
        std::process::exit(1);
    }

    println!("✅ Found ELF file: {}", elf_file.display());

    println!("🎮 Starting mGBA with GDB server...");
    
    // Start mGBA with GDB server enabled
    let mut mgba_process = Command::new("mGBA")
        .arg("-g")  // Enable GDB server
        .arg("-C")
        .arg("logToStdout=1")
        .arg("-C")
        .arg("logLevel.gba.debug=127")
        .arg(&elf_file)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to start mGBA. Make sure mGBA is installed and in PATH.");

    // Wait a moment for mGBA to start up
    thread::sleep(Duration::from_secs(2));

    println!("🐛 Starting GDB session...");
    
    // Start GDB with the ELF file
    let gdb_result = Command::new("arm-none-eabi-gdb")
        .arg(&elf_file)
        .arg("-ex")
        .arg("target remote localhost:2345")
        .arg("-ex")
        .arg("set architecture armv4t")
        .arg("-ex")
        .arg("monitor reset")
        .current_dir(&project_root)
        .status();

    match gdb_result {
        Ok(status) => {
            if !status.success() {
                println!("⚠️  GDB exited with status: {}", status);
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to start GDB: {}", e);
            eprintln!("Make sure arm-none-eabi-gdb is installed and in PATH.");
        }
    }

    // Clean up mGBA process
    println!("🧹 Cleaning up...");
    let _ = mgba_process.kill();
    let _ = mgba_process.wait();
    
    println!("✅ Debug session ended.");
}