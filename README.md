# Rust Keywords Demonstration

## Project Background

This project demonstrates five fundamental Rust keywords through a practical shape calculator application:

1. **`enum`** - Sum types with variants
2. **`trait`** - Interfaces and shared behavior
3. **`return`** - Early returns from functions
4. **`mod`** - Module system for code organization
5. **`const`** - Compile-time constants

The application calculates geometric properties (area and perimeter) for different shapes while validating input using early return patterns.

## App Structure/Architecture

```
rust_keywords_02/
├── Cargo.toml                 # Project configuration
├── README.md                  # This file
├── TESTING.md                 # Testing documentation
└── src/
    ├── main.rs                # Entry point, demonstrates mod keyword
    ├── shapes.rs              # Module with enum, trait, and const
    └── validators.rs          # Module with return keyword examples
```

### Module Breakdown

#### `shapes.rs` - Core Shape Definitions
- **Constants (`const`)**: Mathematical constants (PI, GOLDEN_RATIO)
- **Enum (`enum`)**: `Shape` enum with four variants (Circle, Rectangle, Triangle, Square)
- **Trait (`trait`)**: `Calculable` trait defining shared behavior for all shapes
- **Implementation**: Trait implementation for the Shape enum

#### `validators.rs` - Validation Logic
- **Early Returns (`return`)**: Input validation with early exit patterns
- **Error Handling**: Demonstrates Result types with early returns

#### `main.rs` - Application Entry
- **Module Imports (`mod`)**: Declares and imports the shape and validator modules
- **Orchestration**: Brings all concepts together in working examples

## Keyword Demonstrations

### 1. `const` - Constants
```rust
pub const PI: f64 = 3.14159265359;
pub const GOLDEN_RATIO: f64 = 1.618033988749;
```
Compile-time constants that cannot be changed. Used for mathematical values.

### 2. `enum` - Sum Types/Variants
```rust
pub enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
    Square { side: f64 },
}
```
Defines a type that can be one of several variants, each potentially carrying different data.

### 3. `trait` - Interfaces/Shared Behavior
```rust
pub trait Calculable {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    fn describe(&self) -> String;
}
```
Defines a contract that types can implement, ensuring consistent behavior across different shapes.

### 4. `mod` - Modules
```rust
mod shapes;
mod validators;
```
Organizes code into separate namespaces, promoting separation of concerns and reusability.

### 5. `return` - Early Returns
```rust
if *radius <= 0.0 {
    return Err("Circle radius must be positive".to_string());
}
```
Exits a function early when conditions are met, avoiding deep nesting and improving code clarity.

## Deployment and Usage Instructions

### Prerequisites
- Rust toolchain (rustc, cargo)
- Install from: https://rustup.rs/

### Build the Project
```bash
cargo build
```

### Run the Application
```bash
cargo run
```

### Expected Output
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

Rectangle 10x6
  Area:      60.00
  Perimeter: 32.00

Triangle with base 8 and height 12
  Area:      48.00
  Perimeter: 34.42

Square with side 7
  Area:      49.00
  Perimeter: 28.00

------------------------------------------------------------
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

### Run Tests (when added)
```bash
cargo test
```

### Build for Release
```bash
cargo build --release
```
The optimized binary will be located at `target/release/rust_keywords_demo`

## Key Concepts Illustrated

### Pattern Matching
The application extensively uses pattern matching with `match` to handle different enum variants, demonstrating Rust's powerful type system.

### Error Handling
Proper use of `Result<T, E>` types with early returns shows idiomatic Rust error handling.

### Type Safety
The trait system ensures compile-time guarantees that all shapes implement required methods.

### Module Organization
Clear separation between shape definitions, validation logic, and application orchestration demonstrates good architectural practices.

## Extending the Application

To add a new shape:
1. Add variant to `Shape` enum in `shapes.rs`
2. Implement area/perimeter logic in the `Calculable` trait
3. Add validation rules in `validators.rs`
4. Create test case in `main.rs`

Example:
```rust
// In shapes.rs
pub enum Shape {
    // ... existing variants
    Pentagon { side: f64 },
}

// In Calculable implementation
Shape::Pentagon { side } => {
    // Area calculation for regular pentagon
    1.720477 * side * side
}
```

## Learning Resources

- Rust Book: https://doc.rust-lang.org/book/
- Rust by Example: https://doc.rust-lang.org/rust-by-example/
- Pattern Matching: https://doc.rust-lang.org/book/ch18-00-patterns.html
- Traits: https://doc.rust-lang.org/book/ch10-02-traits.html
- Modules: https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html
