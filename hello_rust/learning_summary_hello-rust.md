L## 📚 Learning Summary (hello_rust)

### ✅ 1. Structure of a Rust Project
- **Cargo** is the official tool to create and manage a Rust project.
- A project typically contains:
  - `Cargo.toml`: project configuration
  - `src/main.rs`: entry point (`fn main()`)
  - `src/lib.rs` (optional): library
- To organize exercises, we created modules (`mod`) and several files in `src/tasks/`.

---

### ✅ 2. Displaying Text
- `println!()` allows you to display text in the terminal.
- Example:
```rust
println!("Hello Rust");
```

---

### ✅ 3. Variables
- You declare a variable with `let`.
- By default, a variable is **immutable** (cannot be modified).
- Example:
```rust
let age = 20;
```

---

### ✅ 4. Mutable Variables
- To modify a variable, add `mut`.
```rust
let mut i = 5;
i -= 1;
```

---

### ✅ 5. Basic Types
- `i32`: integer (e.g., `10`, `-5`)
- `bool`: boolean (`true` / `false`)
- `String`: dynamic string

---

### ✅ 6. String
- `String::from("Tom")` creates a `String` from a string literal.
- This is useful when you want a modifiable or manipulable string.

---

### ✅ 7. Mathematical Operations
- `+`, `-`, `*`, `/`
- Average example:
```rust
let average = (x + y + z) / 3;
```

---

### ✅ 8. Conditions (`if`)
- Allows executing a code block based on a condition.
- Example:
```rust
if age >= 18 {
    println!("You are an adult");
} else {
    println!("You are a minor");
}
```

---

### ✅ 9. Loops

#### 🔹 `for` Loop
- Iterates over a range:
```rust
for i in 1..6 {
    println!("{}", i);
}
```

#### 🔹 `while` Loop
- Executes while a condition is true:
```rust
while i > 0 {
    println!("{}", i);
    i -= 1;
}
```

---

### ✅ 10. Functions
- A function is defined with `fn`.
- Example:
```rust
fn average(grades: &[i32]) -> i32 {
    ...
}
```
- You call a function with:
```rust
let avg = average(&grades);
```

---

### ✅ 11. Arrays
- A fixed array is declared like:
```rust
let grades = [12, 8, 15];
```
- You can iterate over it with `iter()`.

---

### ✅ 12. `enumerate()`
- `enumerate()` allows you to have an index during a loop.
```rust
for (index, grade) in grades.iter().enumerate() {
    println!("Grade {}: {}", index + 1, grade);
}
```

---

### 🎯 Conclusion of hello_rust
You learned:
- the basics of Rust syntax
- how to structure a project
- how to create variables, conditions, loops, functions, and arrays
- to write simple but solid programs

