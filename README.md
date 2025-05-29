# Named Pipe Profiler and Fuzzer (Rust, Windows)

A Rust-based tool for enumerating, analyzing, and fuzzing Windows named pipes. Designed for offensive security research, IPC auditing, and behavior profiling of inter-process communication endpoints on Windows systems.

## Features

- Enumerates all named pipes using raw WinAPI
- Connects to accessible pipes and performs I/O interaction
- Sends a structured handshake (`CAPABILITY_CHECK`)
- Fuzzes pipes with randomized binary payloads
- Multithreaded execution using standard Rust threading
- Pipe name sanitization to handle malformed entries

## Use Cases

- Reverse engineering malware or EDR communication channels
- Discovery of undocumented named pipes
- Behavioral testing of custom Windows services
- Fuzzing IPC interfaces for crash or exception conditions
- Security assessment of inter-process boundaries

## Building

Requirements:
- Windows OS
- Rust (latest stable)
- WinAPI support via `winapi` crate

# Limitations
- Does not inspect or audit access control lists (ACLs) of pipes

- Uses blocking I/O; unresponsive pipes may delay or hang scans

- Random payloads only; lacks protocol-aware or grammar-based fuzzing

- No support for timeouts, retries, or fallback handling

- No crash detection or exception monitoring for the pipe server process

- Output is console-only; no JSON, CSV, or log file output

- Assumes Unicode-compatible pipe names; encoding errors may cause skips

- Static configuration; no CLI options or runtime configuration

# To Be Added
- ACL auditing using GetNamedSecurityInfoW or NtQueryObject

- Timeout-safe I/O operations using asynchronous Rust or timeouts

- CLI interface using clap for:

- Target pipe filters

- Max fuzz iterations

- Output format selection

- Output logging to structured formats (JSON, CSV)

- Grammar-based and protocol-aware fuzzing (e.g., RPC, SMB, NTLM pipes)

- Pipe server process correlation and PID/owner inspection

- Integration with process monitors or crash detectors

- TUI mode with real-time feedback using ratatui or crossterm
