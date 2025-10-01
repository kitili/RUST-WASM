# Peer Testing Guide - Rust WebAssembly Calculator

## 🎯 Testing Objectives

This guide helps peers test the Rust WebAssembly calculator project to ensure it meets all capstone requirements and functions correctly across different environments.

## 📋 Pre-Testing Checklist

### System Requirements Verification
- [ ] Rust installed (check with `rustc --version`)
- [ ] wasm-pack installed (check with `wasm-pack --version`)
- [ ] Python 3 installed (check with `python3 --version`)
- [ ] Modern web browser (Chrome 57+, Firefox 52+, Safari 11+)
- [ ] Git installed (check with `git --version`)

## 🚀 Testing Steps

### Step 1: Clone and Setup
```bash
# Clone the repository
git clone https://github.com/kitili/RUST-WASM.git
cd RUST-WASM

# Verify all files are present
ls -la
# Should see: Cargo.toml, src/, index.html, README.md, CAPSTONE_TOOLKIT.md
```

### Step 2: Build the Project
```bash
# Build the WebAssembly module
wasm-pack build --target web --out-dir pkg

# Verify build output
ls pkg/
# Should see: calculator.js, calculator_bg.wasm, calculator.d.ts
```

### Step 3: Run the Application
```bash
# Start local server
python3 -m http.server 8000

# Open browser to http://localhost:8000
```

### Step 4: Functional Testing

#### Basic Calculator Operations
- [ ] **Number Input**: Click number buttons (0-9) - display should update
- [ ] **Addition**: Try 5 + 3 = 8
- [ ] **Subtraction**: Try 10 - 4 = 6
- [ ] **Multiplication**: Try 7 × 4 = 28
- [ ] **Division**: Try 15 ÷ 3 = 5
- [ ] **Clear Function**: Press C - display should reset to 0

#### Advanced Functions
- [ ] **Square Root**: Click √ then 16 - should show 4
- [ ] **Power**: Click x² then 5 - should show 25
- [ ] **Decimal Numbers**: Try 3.14 + 2.86 = 6

#### Demo Functions
- [ ] **Demo Add**: Click "5 + 3 = ?" - should show 8
- [ ] **Demo Multiply**: Click "7 × 4 = ?" - should show 28
- [ ] **Demo Sqrt**: Click "√16 = ?" - should show 4
- [ ] **Demo Power**: Click "2³ = ?" - should show 8

#### Error Handling
- [ ] **Division by Zero**: Try 5 ÷ 0 - should show "Error"
- [ ] **Negative Square Root**: Try √(-4) - should show "Error"

### Step 5: Browser Compatibility Testing

#### Chrome Testing
- [ ] Open in Chrome
- [ ] Check browser console for errors (F12 → Console)
- [ ] Verify all functions work correctly
- [ ] Test responsive design (resize window)

#### Firefox Testing
- [ ] Open in Firefox
- [ ] Check browser console for errors
- [ ] Verify WebAssembly loads correctly
- [ ] Test all calculator functions

#### Safari Testing (if available)
- [ ] Open in Safari
- [ ] Check for any compatibility issues
- [ ] Verify performance

### Step 6: Performance Testing
- [ ] **Load Time**: Page should load within 2-3 seconds
- [ ] **Calculation Speed**: Operations should be instant
- [ ] **Memory Usage**: Check browser dev tools for memory leaks
- [ ] **Console Logs**: Verify Rust console.log messages appear

### Step 7: Documentation Review

#### README.md Review
- [ ] **Setup Instructions**: Are they clear and complete?
- [ ] **Prerequisites**: Are all requirements listed?
- [ ] **Usage Guide**: Is the interface explained well?
- [ ] **Code Examples**: Are they accurate and helpful?

#### CAPSTONE_TOOLKIT.md Review
- [ ] **Technology Overview**: Is Rust WebAssembly explained clearly?
- [ ] **Installation Steps**: Can you follow them successfully?
- [ ] **Code Examples**: Do they match the actual implementation?
- [ ] **AI Prompt Journal**: Are the prompts realistic and helpful?
- [ ] **Common Issues**: Are the solutions accurate?

## 🐛 Bug Reporting Template

If you find issues, please report them using this format:

```
**Issue Title**: [Brief description]

**Steps to Reproduce**:
1. [Step 1]
2. [Step 2]
3. [Step 3]

**Expected Result**: [What should happen]

**Actual Result**: [What actually happened]

**Browser/OS**: [e.g., Chrome 120 on Ubuntu 22.04]

**Console Errors**: [Any error messages from browser console]

**Screenshots**: [If applicable]
```

## ✅ Success Criteria

The project passes peer testing if:

- [ ] **Functionality**: All calculator operations work correctly
- [ ] **Documentation**: Setup instructions are clear and complete
- [ ] **Code Quality**: Code is well-commented and organized
- [ ] **Browser Compatibility**: Works in at least 2 major browsers
- [ ] **Performance**: Loads quickly and responds instantly
- [ ] **Error Handling**: Gracefully handles edge cases
- [ ] **Learning Value**: Demonstrates Rust WebAssembly concepts clearly

## 📝 Feedback Form

After testing, please provide feedback on:

1. **Ease of Setup** (1-5): How easy was it to get the project running?
2. **Code Quality** (1-5): How well-written and documented is the code?
3. **Documentation** (1-5): How helpful and complete is the documentation?
4. **Learning Value** (1-5): How well does it teach Rust WebAssembly concepts?
5. **Overall Experience** (1-5): How would you rate the overall project?

**Additional Comments**:
[Any other feedback, suggestions, or observations]

---

**Thank you for testing!** Your feedback helps improve the project and ensures it meets all capstone requirements.