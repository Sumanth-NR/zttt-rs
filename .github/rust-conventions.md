# Rust Conventions

## Code Style
- Follow standard Rust naming: `PascalCase` for types, `snake_case` for functions
- Use `cargo fmt` (configured in `rustfmt.toml`)
- Address `cargo clippy` warnings
- Maximum line width: 100 characters

## Performance
- Use `Copy` types where possible (`Player`, `Cell`)
- Prefer arrays over `Vec` for fixed sizes (3x3 board)
- Avoid allocations in performance-critical paths
- No heap allocations during gameplay

## Error Handling
- Use `Result<T, &str>` for fallible operations
- Provide clear error messages
- Validate bounds: `if row >= 3 || col >= 3`

## Documentation
- Document all public APIs with `///` doc comments
- Include examples in documentation:
  ```rust
  /// # Example
  /// ```
  /// use zttt_rs::{Board, Player};
  /// let mut board = Board::new();
  /// board.make_move(0, 0, Player::X).unwrap();
  /// ```
  ```
- Use `#[cfg(test)]` for test modules

## Common Idioms
- Pattern matching over if-else for enums
- Early returns for validation
- Method chaining where appropriate
- `impl Default` for types with sensible defaults
