use crate::shapes::Shape;

// Validator using return for early exits
pub fn validate_shape(shape: &Shape) -> Result<(), String> {
    match shape {
        Shape::Circle { radius } => {
            if *radius <= 0.0 {
                // Early return on invalid input
                return Err("Circle radius must be positive".to_string());
            }
            if *radius > 1000.0 {
                // Another early return
                return Err("Circle radius too large (max 1000)".to_string());
            }
        }
        Shape::Rectangle { width, height } => {
            if *width <= 0.0 || *height <= 0.0 {
                return Err("Rectangle dimensions must be positive".to_string());
            }
            if *width > 1000.0 || *height > 1000.0 {
                return Err("Rectangle dimensions too large (max 1000)".to_string());
            }
        }
        Shape::Triangle { base, height } => {
            if *base <= 0.0 || *height <= 0.0 {
                return Err("Triangle dimensions must be positive".to_string());
            }
        }
        Shape::Square { side } => {
            if *side <= 0.0 {
                return Err("Square side must be positive".to_string());
            }
            if *side > 1000.0 {
                return Err("Square side too large (max 1000)".to_string());
            }
        }
    }
    
    // If we reach here, validation passed
    Ok(())
}

// Another example of early return in calculation
pub fn calculate_efficiency_ratio(width: f64, height: f64) -> Result<f64, String> {
    // Early return for invalid dimensions
    if width <= 0.0 {
        return Err("Width must be positive".to_string());
    }
    
    if height <= 0.0 {
        return Err("Height must be positive".to_string());
    }
    
    // Normal return path
    Ok(width / height)
}
