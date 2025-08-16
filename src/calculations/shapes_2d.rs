use crate::calculations::errors::*;
use std::f64::consts::PI;

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

pub struct Circle {
    pub radius: f64,
}

pub trait Geometric2D {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

impl Rectangle {
    /// Creates a new rectangle with the given width and height.
    pub fn new(width: f64, height: f64) -> Result<Self, CalculationError> {
        if width <= 0.0 || height <= 0.0 {
            return Err(CalculationError {
                message: "Width and height must be positive.".to_string(),
            });
        }
        Ok(Rectangle { width, height })
    }

    /// Creates a rectangle from its area and width.
    pub fn from_area_and_width(area: f64, width: f64) -> Result<Self, CalculationError> {
        if area <= 0.0 || width <= 0.0 {
            return Err(CalculationError {
                message: "Area and width must be positive.".to_string(),
            });
        }
        Ok(Rectangle {
            width,
            height: area / width,
        })
    }

    /// Creates a rectangle from its area and height.
    pub fn from_area_and_height(area: f64, height: f64) -> Result<Self, CalculationError> {
        if area <= 0.0 || height <= 0.0 {
            return Err(CalculationError {
                message: "Area and height must be positive.".to_string(),
            });
        }
        Ok(Rectangle {
            width: area / height,
            height,
        })
    }
}

impl Circle {
    /// Creates a new circle with the given radius.
    pub fn new(radius: f64) -> Result<Self, CalculationError> {
        if radius <= 0.0 {
            return Err(CalculationError {
                message: "Radius must be positive.".to_string(),
            });
        }
        Ok(Circle { radius })
    }

    /// Creates a circle from its diameter.
    pub fn from_diameter(diameter: f64) -> Result<Self, CalculationError> {
        if diameter <= 0.0 {
            return Err(CalculationError {
                message: "Diameter must be positive.".to_string(),
            });
        }
        Ok(Self {
            radius: diameter / 2.0,
        })
    }

    /// Creates a circle from its circumference.
    pub fn from_circumference(circumference: f64) -> Result<Self, CalculationError> {
        if circumference <= 0.0 {
            return Err(CalculationError {
                message: "Circumference must be positive.".to_string(),
            });
        }
        Ok(Self {
            radius: circumference / (2.0 * PI),
        })
    }

    /// Creates a circle from its area.
    pub fn from_area(area: f64) -> Result<Self, CalculationError> {
        if area <= 0.0 {
            return Err(CalculationError {
                message: "Area must be positive.".to_string(),
            });
        }
        Ok(Self {
            radius: (area / PI).sqrt(),
        })
    }
}

impl Default for Rectangle {
    fn default() -> Self {
        Rectangle {
            width: 1.0,
            height: 1.0,
        }
    }
}

impl Geometric2D for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

impl Default for Circle {
    fn default() -> Self {
        Circle { radius: 1.0 }
    }
}

impl Geometric2D for Circle {
    fn area(&self) -> f64 {
        PI * self.radius.powi(2)
    }

    fn perimeter(&self) -> f64 {
        2.0 * PI * self.radius
    }
}
