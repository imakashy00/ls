# ls

A simple implementation of the Unix `ls` command written in Rust.

## Features

You can use different CLI flags depending on what you need.

The following CLI flags are supported:

* `-a` — show hidden files
* `-l` — show detailed file information
* `-t` — sort by modification time
* `-S` — sort by file size
* `-d` — show the directory itself

Flags can be combined, for example:

```bash
cargo run -- -la
```

## Usage

List the current directory:

```bash
cargo run
```

List a specific directory:

```bash
cargo run -- ./src
```

Show hidden files:

```bash
cargo run -- -a
```

Show detailed information:

```bash
cargo run -- -l
```

Sort by modification time:

```bash
cargo run -- -t
```

Sort by size:

```bash
cargo run -- -S
```

## Installation

Build the project with Cargo:

```bash
cargo build
```

Or run it directly:

```bash
cargo run
```
