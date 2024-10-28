# MDX File Combiner

A command-line utility written in Rust that combines multiple `.mdx` files from a directory (and its subdirectories) into a single output file.

## Features

- Recursively scans directories for `.mdx` files
- Preserves original file names as comments in the combined output
- Follows symbolic links
- Handles errors gracefully with detailed messages

## Prerequisites

- Rust and Cargo installed on your system

## Installation

Clone the repository and build the project:

```bash
git clone [repository-url]
cd md-generator
cargo build --release
```

## Usage

Run the program with a directory path as an argument:

```bash
cargo run [directory-path]
```

Or after building:

```bash
./target/release/md-generator [directory-path]
```

The program will:
1. Scan the specified directory and its subdirectories for `.mdx` files
2. Create a new file named `combined.mdx` in the current directory
3. Combine all found `.mdx` files into this single output file
4. Add file path comments to indicate the source of each section

## Example

If you have a directory structure like:

```
docs/
  ├── intro.mdx
  ├── guide/
  │   ├── getting-started.mdx
  │   └── advanced.mdx
  └── api/
      └── reference.mdx
```

Running:

```bash
cargo run docs
```

Will create a `combined.mdx` file containing all the `.mdx` content with file markers like:

```mdx
<!-- File: docs/intro.mdx -->
[content of intro.mdx]

<!-- File: docs/guide/getting-started.mdx -->
[content of getting-started.mdx]

...
```

## Dependencies

- `walkdir`: For recursive directory traversal
- `anyhow`: For error handling
