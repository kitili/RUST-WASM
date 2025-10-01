use wasm_bindgen::prelude::*;

// Import the `console.log` function from the browser
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Define a macro to make console.log easier to use
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

// This is our main Calculator struct that will be available to JavaScript
#[wasm_bindgen]
pub struct Calculator {
    value: f64,
}

#[wasm_bindgen]
impl Calculator {
    // Create a new calculator instance
    #[wasm_bindgen(constructor)]
    pub fn new() -> Calculator {
        console_log!("Calculator created!");
        Calculator { value: 0.0 }
    }

    // Get the current value
    #[wasm_bindgen(getter)]
    pub fn value(&self) -> f64 {
        self.value
    }

    // Add two numbers
    #[wasm_bindgen]
    pub fn add(&mut self, a: f64, b: f64) -> f64 {
        self.value = a + b;
        console_log!("Adding {} + {} = {}", a, b, self.value);
        self.value
    }

    // Subtract two numbers
    #[wasm_bindgen]
    pub fn subtract(&mut self, a: f64, b: f64) -> f64 {
        self.value = a - b;
        console_log!("Subtracting {} - {} = {}", a, b, self.value);
        self.value
    }

    // Multiply two numbers
    #[wasm_bindgen]
    pub fn multiply(&mut self, a: f64, b: f64) -> f64 {
        self.value = a * b;
        console_log!("Multiplying {} * {} = {}", a, b, self.value);
        self.value
    }

    // Divide two numbers
    #[wasm_bindgen]
    pub fn divide(&mut self, a: f64, b: f64) -> f64 {
        if b == 0.0 {
            console_log!("Error: Division by zero!");
            self.value = f64::NAN; // Not a Number
        } else {
            self.value = a / b;
            console_log!("Dividing {} / {} = {}", a, b, self.value);
        }
        self.value
    }

    // Calculate square root
    #[wasm_bindgen]
    pub fn sqrt(&mut self, a: f64) -> f64 {
        if a < 0.0 {
            console_log!("Error: Cannot calculate square root of negative number!");
            self.value = f64::NAN;
        } else {
            self.value = a.sqrt();
            console_log!("Square root of {} = {}", a, self.value);
        }
        self.value
    }

    // Calculate power
    #[wasm_bindgen]
    pub fn power(&mut self, base: f64, exponent: f64) -> f64 {
        self.value = base.powf(exponent);
        console_log!("{} to the power of {} = {}", base, exponent, self.value);
        self.value
    }

    // Clear the calculator
    #[wasm_bindgen]
    pub fn clear(&mut self) {
        self.value = 0.0;
        console_log!("Calculator cleared!");
    }
}

// Simple standalone functions that can be called directly from JavaScript
#[wasm_bindgen]
pub fn add_numbers(a: f64, b: f64) -> f64 {
    a + b
}

#[wasm_bindgen]
pub fn multiply_numbers(a: f64, b: f64) -> f64 {
    a * b
}

// This function will be called when the WASM module is loaded
#[wasm_bindgen(start)]
pub fn main() {
    console_log!("Rust + WebAssembly Calculator loaded successfully!");
}
