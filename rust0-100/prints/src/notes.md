# Rust Printing Macros Notes

## Variants of `println!`

### `print!`
- Similar to `println!` but **doesn't add a newline** at the end
- Useful for printing multiple items on the same line
- Example: 
  ```rust
  print!("Hello, ");
  print!("world!"); 
  // Output: Hello, world! (no newline)
  ```

### `eprintln!`
- Prints to the **standard error stream** (stderr) instead of standard output (stdout)
- Useful for error messages
- Example:
  ```rust
  eprintln!("Error: File not found");
  ```

### `format!`
- Creates a **new String** with the given format
- Useful for creating strings with multiple values or advanced formatting
- Example:
  ```rust
  let msg = format!("Hello, {}!", "Rust");
  ```

### `write!`
- Writes formatted text to a **buffer** rather than returning a string
- Works with any type that implements the `Write` trait
- Example:
  ```rust
  use std::fmt::Write;
  let mut output = String::new();
  write!(output, "The answer is {}", 42).unwrap();
  ```

## Key Characteristics
1. **Macro Indicator**: The `!` at the end indicates these are macros, not functions
2. **Placeholder Syntax**: All use `{}` as placeholders for values
3. **Standard Library**: These macros are part of Rust's standard library

## Usage Pattern
All follow the same basic pattern:
```rust
macro_name!(format_string, arguments...);
```

## When to Use Which
- `print!`: When you need to control newlines manually
- `eprintln!`: For error messages and diagnostic output
- `format!`: When you need the output as a String value
- `write!`: When writing to buffers, files, or other Write-implementing types