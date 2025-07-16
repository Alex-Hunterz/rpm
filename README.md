# rpm - A Simple Process Manager

A lightweight, command-line process management tool written in Rust for Unix-like systems. Made this for the mini-project in TILDE 4.0 to gain understanding of process management and to get some familiarity with Rust.

  
## Features

- **Create:** Spawn new background processes.
- **List:** Display all processes spawned and tracked by `rpm`.
- **PS:** View all currently running system processes.
- **Kill:** Terminate a process by its PID.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)

### Build

```sh
cargo build --release
```

### Run

```sh
./target/release/rpm
```

## Usage

Once running, you can use the following commands:

```
> help
Commands: help, create, list, ps, kill <pid>, exit

> create
Created process with PID 12345

> list
Tracked processes:
- PID 12345 [alive]

> kill 12345
Sent SIGKILL to process 12345
```
