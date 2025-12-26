# Testing Guide for Mac M4

## Quick Start

### 1. Ensure Rust is Installed
```bash
rustc --version
cargo --version
```

If not installed:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 2. Navigate to Project
```bash
cd ~/rust_keywords_demo
```

### 3. Build and Run
```bash
# Development build
cargo build

# Run the application
cargo run

# Release build (optimized)
cargo build --release
./target/release/rust_keywords_demo
```

### 4. Check Project Structure
```bash
tree
```

Expected structure:
```
.
├── Cargo.toml
├── README.md
└── src
    ├── main.rs
    ├── shapes.rs
    └── validators.rs
```

## Keyword Verification Checklist

- [ ] **const**: Check output shows PI and GOLDEN_RATIO values
- [ ] **enum**: Verify all 4 shape variants (Circle, Rectangle, Triangle, Square) are created
- [ ] **trait**: Confirm area(), perimeter(), describe() methods work for all shapes
- [ ] **mod**: Verify shapes and validators modules are properly imported
- [ ] **return**: Check that invalid dimensions trigger error messages with early returns

## Expected Console Output Preview

```
=== Rust Keywords Demo ===

Mathematical Constants:
  PI = 3.14159265359
  Golden Ratio = 1.618033988749

Shape Calculations (using trait methods):
------------------------------------------------------------
Circle with radius 5.00
  Area:      78.54
  Perimeter: 31.42

[... more shapes ...]

Efficiency Ratio Calculations:
  16.0 / 9.0 = 1.778
  4.0 / 3.0 = 1.333
  -5.0 / 10.0 = Error: Width must be positive
  10.0 / 0.0 = Error: Height must be positive

=== Demo Complete ===

Keywords Demonstrated:
  ✓ const     - PI and GOLDEN_RATIO constants
  ✓ enum      - Shape with Circle, Rectangle, Triangle, Square variants
  ✓ trait     - Calculable trait with area(), perimeter(), describe()
  ✓ mod       - shapes and validators modules
  ✓ return    - Early returns in validate_shape() and calculate_efficiency_ratio()
```

## Code Exploration Commands

### View Individual Modules
```bash
# View the enum and trait definitions
cat src/shapes.rs

# View the early return examples
cat src/validators.rs

# View module imports and main logic
cat src/main.rs
```

### Compile-time Checks
```bash
# Check code without building
cargo check

# Format code to Rust standards
cargo fmt

# Run linter
cargo clippy
```

## Troubleshooting

### If build fails:
1. Check Rust version: `rustc --version` (should be 1.70+)
2. Update Rust: `rustup update`
3. Clean build: `cargo clean && cargo build`

### If modules not found:
- Ensure all .rs files are in `src/` directory
- Verify `mod` declarations in main.rs

### On Mac M4 specific:
- Should compile natively for ARM64
- Check with: `file target/debug/rust_keywords_demo`
- Should show: "Mach-O 64-bit executable arm64"
