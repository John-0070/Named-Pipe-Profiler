use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::io::{Read, Write};
use std::fs::OpenOptions;
use std::os::windows::ffi::OsStrExt;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};
use rand::Rng;
use winapi::um::fileapi::{FindFirstFileW, FindNextFileW, FindClose};
use winapi::um::minwinbase::WIN32_FIND_DATAW;
use winapi::um::winnt::HANDLE;

/// Main entry point
fn main() {
    let pipes = enumerate_named_pipes();
    if pipes.is_empty() {
        println!("[!] No named pipes found.");
        return;
    }

    println!("[*] Found {} named pipes:", pipes.len());
    for pipe in &pipes {
        println!(" - {}", pipe);
    }

    println!("\n[*] Testing interaction with pipes...");
    let shared_pipes = Arc::new(pipes);

    // Multithreaded pipe interaction
    let threads: Vec<_> = shared_pipes
        .iter()
        .map(|pipe| {
            let pipe_name = pipe.clone();
            thread::spawn(move || {
                if let Some(clean_name) = sanitize_pipe_name(&pipe_name) {
                    profile_pipe_behavior(&clean_name);
                } else {
                    println!("[-] Skipped malformed pipe name: {}", pipe_name);
                }
            })
        })
        .collect();

    // Join all threads
    for thread in threads {
        let _ = thread.join();
    }

    println!("\n[*] Pipe interaction complete.");
}

/// Enumerate all named pipes on the system
fn enumerate_named_pipes() -> Vec<String> {
    let pipes_root = "\\\\.\\pipe\\*";
    let pipes_root_wide: Vec<u16> = OsString::from(pipes_root).encode_wide().chain(Some(0)).collect();
    let mut pipe_list = Vec::new();

    unsafe {
        let mut find_data: WIN32_FIND_DATAW = std::mem::zeroed();
        let find_handle: HANDLE = FindFirstFileW(pipes_root_wide.as_ptr(), &mut find_data);

        if find_handle.is_null() {
            println!("[!] Failed to find any named pipes. Ensure you have administrative privileges.");
            return pipe_list;
        }

        loop {
            let pipe_name = OsString::from_wide(&find_data.cFileName);
            if let Ok(pipe_str) = pipe_name.into_string() {
                pipe_list.push(pipe_str);
            }

            if FindNextFileW(find_handle, &mut find_data) == 0 {
                break;
            }
        }

        FindClose(find_handle);
    }

    pipe_list
}

/// Profile the behavior of a specific pipe
fn profile_pipe_behavior(pipe_name: &str) {
    let full_pipe_path = format!("\\\\.\\pipe\\{}", pipe_name);
    println!("\n[*] Profiling pipe: {}", pipe_name);

    match OpenOptions::new()
        .read(true)
        .write(true)
        .open(&full_pipe_path)
    {
        Ok(mut pipe) => {
            println!("[+] Connected to pipe: {}", pipe_name);

            // Analyze pipe capabilities
            analyze_pipe_capabilities(&mut pipe, pipe_name);

            // Fuzz the pipe with test payloads
            fuzz_pipe(&mut pipe, pipe_name);

        }
        Err(e) => {
            println!("\n[-] Could not connect to pipe: {}", pipe_name);
            println!("    [!] Error: {}", e);
        }
    }
}

/// Analyze the capabilities of the pipe (read/write, timeout behavior, etc.)
fn analyze_pipe_capabilities(pipe: &mut std::fs::File, pipe_name: &str) {
    println!("    [*] Analyzing capabilities for: {}", pipe_name);

    let test_message = b"CAPABILITY_CHECK";
    if let Err(e) = pipe.write_all(test_message) {
        println!("    [-] Write failed: {}", e);
        return;
    }
    println!("    [*] Sent capability check message.");

    let mut response = [0u8; 512];
    match pipe.read(&mut response) {
        Ok(bytes_read) => {
            println!(
                "    [+] Received response: {:?}",
                String::from_utf8_lossy(&response[..bytes_read])
            );
        }
        Err(_) => println!("    [!] No response or read failed."),
    }
}

/// Fuzz the pipe with randomized or structured payloads
fn fuzz_pipe(pipe: &mut std::fs::File, pipe_name: &str) {
    println!("    [*] Fuzzing pipe: {}", pipe_name);

    let mut rng = rand::thread_rng();
    for i in 0..10 {
        let payload_size: usize = rng.gen_range(1..=128);
        let payload: Vec<u8> = (0..payload_size).map(|_| rng.gen::<u8>()).collect();

        println!(
            "    [*] Fuzz iteration {}: Sending {} bytes.",
            i + 1,
            payload.len()
        );

        if let Err(e) = pipe.write_all(&payload) {
            println!("    [-] Write failed during fuzzing: {}", e);
            continue;
        }

        let mut response = [0u8; 512];
        match pipe.read(&mut response) {
            Ok(bytes_read) => {
                println!(
                    "    [+] Fuzz response: {:?}",
                    String::from_utf8_lossy(&response[..bytes_read])
                );
            }
            Err(_) => {
                println!("    [!] No response during fuzzing iteration.");
            }
        }

        // Wait a bit before the next iteration
        thread::sleep(Duration::from_millis(200));
    }
}

/// Sanitize pipe names by trimming whitespace and filtering out invalid characters
fn sanitize_pipe_name(pipe_name: &str) -> Option<String> {
    let clean_name = pipe_name.trim();
    if clean_name.is_empty() {
        println!(
            "[-] Skipped pipe due to empty name. This might indicate a malformed pipe entry."
        );
        return None;
    }

    if clean_name.contains('\0') {
        println!(
            "[-] Skipped pipe: '{}' contains a NULL character, which is not allowed.",
            pipe_name
        );
        return None;
    }

    Some(clean_name.to_string())
}