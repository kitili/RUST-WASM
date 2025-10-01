# Getting Started with Rust WebAssembly – A Beginner's Guide

## 1. Title & Objective

**Technology Chosen:** Rust WebAssembly (WASM)

**Why I chose it:** Rust WebAssembly represents the cutting edge of web performance, allowing developers to write high-performance code in Rust and run it directly in browsers. It's perfect for computationally intensive applications like calculators, games, image processing, and AI inference.

**End Goal:** Create a fully functional calculator that runs in the browser with near-native performance, demonstrating the power of Rust compiled to WebAssembly.

## 2. Quick Summary of the Technology

**What is Rust WebAssembly?**
Rust WebAssembly is a technology that allows you to compile Rust code to WebAssembly (WASM), a binary instruction format that runs in web browsers at near-native speed. It bridges the gap between high-level web development and low-level system programming.

**Where is it used?**
- High-performance web applications (Figma, AutoCAD Web)
- Game engines (Unity, Unreal Engine)
- Image/video processing tools
- AI/ML inference in browsers
- Cryptocurrency mining in browsers
- Scientific computing applications

**Real-world example:** Figma uses WebAssembly to power their collaborative design tool, enabling complex vector graphics operations that would be impossible with traditional JavaScript.

## 3. System Requirements

**OS:** Linux, macOS, or Windows
**Tools Required:**
- Rust (latest stable version)
- wasm-pack (WebAssembly build tool)
- Python 3 (for local development server)
- Modern web browser (Chrome 57+, Firefox 52+, Safari 11+)
- Git (for version control)

**Packages:**
- `wasm-bindgen` crate for Rust-WebAssembly bindings
- `wasm-pack` for building and packaging

## 4. Installation & Setup Instructions

### Step 1: Install Rust
```bash
# Install Rust using rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### Step 2: Install wasm-pack
```bash
# Install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

### Step 3: Add WebAssembly target
```bash
# Add the wasm32-unknown-unknown target
rustup target add wasm32-unknown-unknown
```

### Step 4: Create a new Rust project
```bash
# Create new project
cargo new --lib rust-wasm-calculator
cd rust-wasm-calculator
```

### Step 5: Configure Cargo.toml
```toml
[package]
name = "calculator"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2"

[package.metadata.wasm-pack.profile.release]
wasm-opt = false
```

## 5. Minimal Working Example

### What the example does:
Creates a simple calculator with basic arithmetic operations that runs entirely in the browser using Rust compiled to WebAssembly.

### Rust Code (src/lib.rs):
```rust
use wasm_bindgen::prelude::*;

// Import console.log for debugging
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Macro to make console.log easier to use
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

// Main Calculator struct
#[wasm_bindgen]
pub struct Calculator {
    value: f64,
}

#[wasm_bindgen]
impl Calculator {
    // Create new calculator instance
    #[wasm_bindgen(constructor)]
    pub fn new() -> Calculator {
        console_log!("Calculator created!");
        Calculator { value: 0.0 }
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
            self.value = f64::NAN;
        } else {
            self.value = a / b;
            console_log!("Dividing {} / {} = {}", a, b, self.value);
        }
        self.value
    }
}

// Standalone functions
#[wasm_bindgen]
pub fn add_numbers(a: f64, b: f64) -> f64 {
    a + b
}

#[wasm_bindgen]
pub fn multiply_numbers(a: f64, b: f64) -> f64 {
    a * b
}

// Initialize when WASM module loads
#[wasm_bindgen(start)]
pub fn main() {
    console_log!("Rust + WebAssembly Calculator loaded!");
}
```

### Build and Run:
```bash
# Build the WebAssembly module
wasm-pack build --target web --out-dir pkg

# Start local server
python3 -m http.server 8000

# Open http://localhost:8000 in browser
```

### Expected Output:
A fully functional calculator interface in the browser with:
- Number input buttons (0-9)
- Arithmetic operations (+, -, ×, ÷)
- Real-time calculation display
- Console logging of operations

## 6. AI Prompt Journal

### Prompt 1: Initial Learning
**Prompt used:** "I want to learn Rust WebAssembly. Give me a step-by-step guide to create a simple calculator that runs in the browser using Rust and WebAssembly."

**AI's response summary:** The AI provided a comprehensive overview of Rust WebAssembly, explained the wasm-bindgen crate, and gave step-by-step instructions for setting up a basic project structure.

**Evaluation:** Extremely helpful for understanding the core concepts and getting started quickly. The AI explained complex topics like memory management and JavaScript bindings in an accessible way.

### Prompt 2: Error Resolution
**Prompt used:** "I'm getting a 'wasm-pack not found' error when trying to build my Rust WebAssembly project. How do I install and configure wasm-pack properly?"

**AI's response summary:** The AI provided multiple installation methods for wasm-pack, explained the PATH configuration, and offered troubleshooting steps for common installation issues.

**Evaluation:** Very helpful for resolving the build tool installation. The AI provided both the solution and context about why the error occurred.

### Prompt 3: Advanced Features
**Prompt used:** "How can I add more advanced mathematical functions like square root and power operations to my Rust WebAssembly calculator?"

**AI's response summary:** The AI showed how to implement additional mathematical functions in Rust, explained error handling for edge cases (like negative square roots), and demonstrated proper wasm-bindgen annotations.

**Evaluation:** Excellent for extending functionality. The AI provided both the code and best practices for mathematical operations in WebAssembly.

### Prompt 4: Performance Optimization
**Prompt used:** "What are the best practices for optimizing Rust WebAssembly performance and reducing bundle size?"

**AI's response summary:** The AI explained various optimization techniques including release builds, wasm-opt, dead code elimination, and memory management strategies.

**Evaluation:** Very valuable for understanding production considerations. The AI provided both theoretical knowledge and practical implementation tips.

## 7. Common Issues & Fixes

### Issue 1: wasm-pack not found
**Error:** `wasm-pack: command not found`
**Solution:**
```bash
# Add cargo bin to PATH
export PATH="$HOME/.cargo/bin:$PATH"
# Or reinstall wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

### Issue 2: Target not found
**Error:** `error: target 'wasm32-unknown-unknown' not found`
**Solution:**
```bash
rustup target add wasm32-unknown-unknown
```

### Issue 3: CORS errors in browser
**Error:** `CORS policy: Cross origin requests are only supported for protocol schemes`
**Solution:** Always serve files through HTTP server, not file:// protocol
```bash
python3 -m http.server 8000
# Use http://localhost:8000, not file://
```

### Issue 4: Module loading errors
**Error:** `Failed to fetch dynamically imported module`
**Solution:** Ensure pkg/ directory exists and contains generated files
```bash
wasm-pack build --target web --out-dir pkg
```

### Issue 5: Division by zero handling
**Error:** JavaScript receives NaN values
**Solution:** Implement proper error handling in Rust
```rust
if b == 0.0 {
    self.value = f64::NAN;
} else {
    self.value = a / b;
}
```

## 8. References

### Official Documentation
- [Rust WebAssembly Book](https://rustwasm.github.io/docs/book/)
- [wasm-bindgen Guide](https://rustwasm.github.io/wasm-bindgen/)
- [WebAssembly MDN Docs](https://developer.mozilla.org/en-US/docs/WebAssembly)
- [Rust Official Documentation](https://doc.rust-lang.org/)

### Tutorials and Guides
- [Rust and WebAssembly Tutorial](https://rustwasm.github.io/book/)
- [WebAssembly by Example](https://webassemblybyexample.com/)
- [Rust WebAssembly Examples](https://github.com/rustwasm/wasm-bindgen/tree/main/examples)

### Community Resources
- [Rust WebAssembly Working Group](https://rustwasm.github.io/)
- [WebAssembly Community Group](https://webassembly.org/)
- [Rust Users Forum](https://users.rust-lang.org/)

### Video Tutorials
- [Rust WebAssembly Crash Course](https://www.youtube.com/watch?v=wVhQJ3VQZgI)
- [Building Web Apps with Rust and WebAssembly](https://www.youtube.com/watch?v=uI1J8o1rY10)

---

**Created by [kitili](https://github.com/kitili)** 🚀

*Built with ❤️ using Rust and WebAssembly* 🦀