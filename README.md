# 🦀 Rust WebAssembly Calculator

A high-performance calculator built with Rust and WebAssembly, featuring a beautiful web interface and near-native performance in the browser.

![Calculator Demo](https://img.shields.io/badge/Status-Live-brightgreen)
![Rust](https://img.shields.io/badge/Rust-1.70+-orange)
![WebAssembly](https://img.shields.io/badge/WebAssembly-Enabled-purple)
![License](https://img.shields.io/badge/License-MIT-blue)

## ✨ Features

- **🚀 High Performance**: Powered by Rust compiled to WebAssembly
- **🎨 Beautiful UI**: Modern, responsive design with gradient backgrounds
- **🧮 Full Calculator**: Basic arithmetic operations (+, -, ×, ÷)
- **📐 Advanced Functions**: Square root, power operations
- **🎯 Interactive Demo**: Pre-programmed calculations showcasing Rust functions
- **🌐 Browser Native**: Runs entirely in the browser with no server required
- **📱 Responsive**: Works on desktop and mobile devices

## 🛠️ Tech Stack

- **Backend**: Rust with `wasm-bindgen`
- **Frontend**: Vanilla HTML, CSS, JavaScript
- **Build Tool**: `wasm-pack`
- **Web Server**: Python HTTP server (for local development)

## 🚀 Quick Start

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable version)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- Python 3 (for local development server)

### Installation & Running

1. **Clone the repository**
   ```bash
   git clone https://github.com/kitili/RUST-WASM.git
   cd RUST-WASM
   ```

2. **Build the WebAssembly module**
   ```bash
   wasm-pack build --target web --out-dir pkg
   ```

3. **Start a local web server**
   ```bash
   python3 -m http.server 8000
   ```

4. **Open your browser**
   Navigate to `http://localhost:8000`

## 🎮 How to Use

### Basic Operations
- **Numbers**: Click number buttons (0-9) to input values
- **Operators**: Use +, -, ×, ÷ for arithmetic operations
- **Equals**: Press = to calculate the result
- **Clear**: Press C to reset the calculator

### Advanced Functions
- **Square Root**: Click √ to calculate square root of current number
- **Power**: Click x² to square the current number
- **Decimal**: Use . for decimal numbers

### Demo Functions
Try the pre-programmed calculations:
- **5 + 3 = ?** - Demonstrates addition
- **7 × 4 = ?** - Demonstrates multiplication  
- **√16 = ?** - Demonstrates square root
- **2³ = ?** - Demonstrates power operations

## 🏗️ Project Structure

```
RUST-WASM/
├── src/
│   └── lib.rs              # Rust calculator implementation
├── pkg/                    # Generated WebAssembly files
│   ├── calculator.js       # JavaScript bindings
│   ├── calculator_bg.wasm  # WebAssembly binary
│   └── ...
├── index.html              # Web interface
├── Cargo.toml              # Rust project configuration
└── README.md               # This file
```

## 🔧 Development

### Building for Production

```bash
# Build optimized WebAssembly module
wasm-pack build --target web --out-dir pkg --release
```

### Adding New Functions

1. **Add Rust function** in `src/lib.rs`:
   ```rust
   #[wasm_bindgen]
   pub fn your_function(a: f64, b: f64) -> f64 {
       // Your implementation
   }
   ```

2. **Update JavaScript** in `index.html` to call the new function

3. **Rebuild** with `wasm-pack build --target web --out-dir pkg`

## 🧪 Available Rust Functions

### Calculator Class Methods
- `add(a: f64, b: f64) -> f64` - Addition
- `subtract(a: f64, b: f64) -> f64` - Subtraction
- `multiply(a: f64, b: f64) -> f64` - Multiplication
- `divide(a: f64, b: f64) -> f64` - Division
- `sqrt(a: f64) -> f64` - Square root
- `power(base: f64, exponent: f64) -> f64` - Power operation
- `clear()` - Reset calculator

### Standalone Functions
- `add_numbers(a: f64, b: f64) -> f64` - Direct addition
- `multiply_numbers(a: f64, b: f64) -> f64` - Direct multiplication

## 🌐 Browser Compatibility

- ✅ Chrome 57+
- ✅ Firefox 52+
- ✅ Safari 11+
- ✅ Edge 16+

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) for Rust-WebAssembly bindings
- [wasm-pack](https://github.com/rustwasm/wasm-pack) for the build tool
- The Rust WebAssembly community for excellent documentation

## 🔗 Related Projects

- [Rust WebAssembly Book](https://rustwasm.github.io/docs/book/)
- [wasm-bindgen Guide](https://rustwasm.github.io/wasm-bindgen/)
- [WebAssembly MDN Docs](https://developer.mozilla.org/en-US/docs/WebAssembly)

---

**Made with ❤️ and Rust** 🦀