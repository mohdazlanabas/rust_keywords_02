// Module declarations (mod keyword)
mod shapes;
mod validators;

// Importing items from modules
use shapes::{Calculable, Shape, PI, GOLDEN_RATIO};
use validators::{validate_shape, calculate_efficiency_ratio};

fn main() {
    println!("=== Rust Keywords Demo ===\n");
    
    // Demonstrating const usage
    println!("Mathematical Constants:");
    println!("  PI = {}", PI);
    println!("  Golden Ratio = {}\n", GOLDEN_RATIO);
    
    // Creating shapes using enum variants
    let shapes = vec![
        Shape::Circle { radius: 5.0 },
        Shape::Rectangle { width: 10.0, height: 6.0 },
        Shape::Triangle { base: 8.0, height: 12.0 },
        Shape::Square { side: 7.0 },
    ];
    
    println!("Shape Calculations (using trait methods):");
    println!("{:-<60}", "");
    
    for shape in &shapes {
        // Validate using early returns
        match validate_shape(shape) {
            Ok(_) => {
                // Using trait methods
                println!("{}", shape.describe());
                println!("  Area:      {:.2}", shape.area());
                println!("  Perimeter: {:.2}\n", shape.perimeter());
            }
            Err(e) => {
                println!("Invalid shape: {}\n", e);
            }
        }
    }
    
    // Demonstrating early return in efficiency calculation
    println!("{:-<60}", "");
    println!("Efficiency Ratio Calculations:");
    
    let test_dimensions = vec![
        (16.0, 9.0),   // Valid
        (4.0, 3.0),    // Valid
        (-5.0, 10.0),  // Invalid - will trigger early return
        (10.0, 0.0),   // Invalid - will trigger early return
    ];
    
    for (w, h) in test_dimensions {
        match calculate_efficiency_ratio(w, h) {
            Ok(ratio) => println!("  {:.1} / {:.1} = {:.3}", w, h, ratio),
            Err(e) => println!("  {:.1} / {:.1} = Error: {}", w, h, e),
        }
    }
    
    println!("\n=== Demo Complete ===");
    
    // Summary of demonstrated keywords
    print_summary();
}

fn print_summary() {
    println!("\nKeywords Demonstrated:");
    println!("  ✓ const     - PI and GOLDEN_RATIO constants");
    println!("  ✓ enum      - Shape with Circle, Rectangle, Triangle, Square variants");
    println!("  ✓ trait     - Calculable trait with area(), perimeter(), describe()");
    println!("  ✓ mod       - shapes and validators modules");
    println!("  ✓ return    - Early returns in validate_shape() and calculate_efficiency_ratio()");
}
