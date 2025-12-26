// Mathematical constants (const keyword)
pub const PI: f64 = 3.14159265359;
pub const GOLDEN_RATIO: f64 = 1.618033988749;

// Enum demonstrating sum types/variants
#[derive(Debug)]
pub enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
    Square { side: f64 },
}

// Trait demonstrating interfaces/shared behavior
pub trait Calculable {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    fn describe(&self) -> String;
}

// Implementing the trait for the enum
impl Calculable for Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius } => PI * radius * radius,
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle { base, height } => 0.5 * base * height,
            Shape::Square { side } => side * side,
        }
    }

    fn perimeter(&self) -> f64 {
        match self {
            Shape::Circle { radius } => 2.0 * PI * radius,
            Shape::Rectangle { width, height } => 2.0 * (width + height),
            Shape::Triangle { base, height } => {
                // Assuming right triangle for simplicity
                let hypotenuse = (base * base + height * height).sqrt();
                base + height + hypotenuse
            }
            Shape::Square { side } => 4.0 * side,
        }
    }

    fn describe(&self) -> String {
        match self {
            Shape::Circle { radius } => format!("Circle with radius {:.2}", radius),
            Shape::Rectangle { width, height } => {
                format!("Rectangle {}x{}", width, height)
            }
            Shape::Triangle { base, height } => {
                format!("Triangle with base {} and height {}", base, height)
            }
            Shape::Square { side } => format!("Square with side {}", side),
        }
    }
}
