# Named Pipe Profiler & Fuzzer for Windows (Rust)

A multithreaded Rust tool that enumerates, connects to, and fuzzes **Windows named pipes** to profile their behavior and detect potential IPC vulnerabilities or undocumented endpoints.

## Features

- Enumerates all named pipes on the local system
- Profiles read/write behavior for each pipe
- Sends a test handshake message to check for responsiveness
- Fuzzes pipes with randomized payloads
- Multithreaded execution for high performance
- Built with stable Rust and raw WinAPI for native access

## Use Cases

- Malware & EDR reverse engineering
- Offensive research (IPC privilege escalation, weak ACLs)
- Security audits of custom Windows services
- Discovery of undocumented inter-process communication channels
- Fuzz testing IPC servers for stability and bugs

## Getting Started

### Requirements

- Windows (x64)
- Rust (latest stable)
- Admin privileges (recommended for full pipe access)
