### 📚 Learning Summary (hello_rust)

### ✅ 1. Structure of a Rust Project
- **Cargo** is the official tool to create and manage a Rust project. Think of it like npm for JavaScript or pip for Python.
- To create a new project: `cargo new project_name`
- To compile and run: `cargo run`
- To just compile: `cargo build`

A project typically contains:
  - **`Cargo.toml`**: project configuration file (like package.json)
    - Contains project name, version, dependencies
    - Example:
    ```toml
    [package]
    name = "hello_rust"
    version = "0.1.0"
    edition = "2021"
    ```
  - **`src/main.rs`**: entry point of your program
    - Contains the `fn main()` function that runs when you execute the program
  - **`src/lib.rs`** (optional): for creating libraries that other projects can use
  - **`target/`**: folder where compiled code goes (you can ignore this)
  
- **Modules** (`mod`): to organize code into separate files
  - We created `src/tasks/` folder with multiple task files
  - In `main.rs`, we declare: `mod tasks;` to use the tasks module
  - Each task file has a `pub fn run()` function to execute the exercise

---

### ✅ 2. Displaying Text
- **`println!`** is a **macro** (notice the `!`) that allows you to display text in the terminal.
- Macros are like special functions in Rust that can do more complex things
- **Basic usage:**
```rust
println!("Hello Rust");  // Displays: Hello Rust
```
- **With variables (string formatting):**
```rust
let name = "Tom";
println!("Hello, {}", name);  // {} is replaced by the variable
```
- **Multiple variables:**
```rust
let age = 25;
let city = "Paris";
println!("{} is {} years old and lives in {}", name, age, city);
```
- **Named placeholders (more readable):**
```rust
println!("My name is {name} and I'm {age} years old", name="Tom", age=25);
```

---

### ✅ 3. Variables
- You declare a variable with `let`.
- **By default, a variable is immutable** (cannot be modified) - this is a key Rust safety feature!
- Variable names use `snake_case` (lowercase with underscores)
- **Basic examples:**
```rust
let age = 20;           // i32 (integer) type inferred automatically
let name = "Tom";       // &str (string slice) type
let pi = 3.14;          // f64 (float) type
let is_student = true;  // bool (boolean) type
```
- **Type annotations (optional but sometimes useful):**
```rust
let age: i32 = 20;      // explicitly saying age is an i32
let height: f64 = 1.75; // explicitly a 64-bit float
```
- **Why immutable by default?**
  - Prevents accidental changes
  - Makes code safer and easier to understand
  - Forces you to think about what should change

---

### ✅ 4. Mutable Variables
- To make a variable modifiable, add `mut` keyword.
- Use `mut` only when you really need to change the value
```rust
let mut i = 5;    // Now i can be modified
i -= 1;           // i is now 4
i = i * 2;        // i is now 8
```
- **Common use cases for `mut`:**
  - Counters in loops
  - Accumulating values (like sums)
  - Building strings progressively
  
**Example - counter:**
```rust
let mut count = 0;
for _ in 0..10 {
    count += 1;  // Add 1 each iteration
}
println!("Final count: {}", count);  // 10
```

---

### ✅ 5. Basic Types
Rust has several primitive types. Here are the most common:

**Integers (whole numbers):**
- `i32`: 32-bit signed integer (most common, range: -2,147,483,648 to 2,147,483,647)
- `i64`: 64-bit signed integer (for bigger numbers)
- `u32`: 32-bit unsigned integer (only positive: 0 to 4,294,967,295)
- Examples: `10`, `-5`, `1000`

**Floating point (decimals):**
- `f32`: 32-bit decimal
- `f64`: 64-bit decimal (default for decimals)
- Examples: `3.14`, `-0.5`, `2.0`

**Boolean:**
- `bool`: `true` or `false`
- Used in conditions

**Character:**
- `char`: single character in single quotes
- Examples: `'a'`, '€', '😀'`

**Strings:**
- `&str`: string slice (text in double quotes, fixed size)
- `String`: dynamic, growable string (can be modified)

**Example with types:**
```rust
let age: i32 = 25;
let temperature: f64 = 36.6;
let is_valid: bool = true;
let grade: char = 'A';
let message: &str = "Hello";
let name: String = String::from("Tom");
```

---

### ✅ 6. String vs &str
This can be confusing for beginners!

**`&str` (string slice):**
- Fixed-size text, cannot be modified
- Stored directly in your program's memory
- More efficient, lightweight
```rust
let greeting = "Hello";  // Type: &str
```

**`String` (owned string):**
- Dynamic text that can grow or shrink
- Allocated on the heap (dynamic memory)
- Can be modified if declared with `mut`
- Created with `String::from()` or `.to_string()`
```rust
let mut name = String::from("Tom");
name.push_str(" Smith");  // Now name = "Tom Smith"
```

**When to use which:**
- Use `&str` for fixed text that won't change
- Use `String` when you need to build or modify text

**Converting between them:**
```rust
let str_slice = "hello";
let string = String::from(str_slice);     // &str → String
let str_again: &str = &string;            // String → &str
```

---

### ✅ 7. Mathematical Operations
Rust supports all standard math operations:

**Basic operators:**
- `+` addition
- `-` subtraction
- `*` multiplication
- `/` division (integer division if both numbers are integers!)
- `%` modulo (remainder)

**Examples:**
```rust
let a = 10;
let b = 3;

let sum = a + b;        // 13
let diff = a - b;       // 7
let product = a * b;    // 30
let quotient = a / b;   // 3 (integer division, drops decimal)
let remainder = a % b;  // 1 (10 divided by 3 leaves remainder 1)
```

**⚠️ Integer division gotcha:**
```rust
let x = 10;
let y = 3;
let avg = (x + y) / 2;  // avg = 6, not 6.5! (integer division)

// For decimals, use floats:
let avg_float = (x as f64 + y as f64) / 2.0;  // 6.5
```

**Compound assignment operators:**
```rust
let mut count = 5;
count += 1;   // Same as: count = count + 1   → count is now 6
count -= 2;   // Same as: count = count - 2   → count is now 4
count *= 3;   // Same as: count = count * 3   → count is now 12
count /= 4;   // Same as: count = count / 4   → count is now 3
```

**Example - calculating average:**
```rust
let x = 10;
let y = 20;
let z = 30;
let average = (x + y + z) / 3;  // (60) / 3 = 20
println!("The average is {}", average);
```

---

### ✅ 8. Conditions (`if`)
Conditions allow executing code blocks based on whether something is true or false.

**Basic structure:**
```rust
if condition {
    // Code runs if condition is true
} else {
    // Code runs if condition is false
}
```

**Example - age check:**
```rust
let age = 18;

if age >= 18 {
    println!("You are an adult");
} else {
    println!("You are a minor");
}
```

**Comparison operators:**
- `==` equal to
- `!=` not equal to
- `>` greater than
- `<` less than
- `>=` greater than or equal
- `<=` less than or equal

**Multiple conditions with `else if`:**
```rust
let score = 85;

if score >= 90 {
    println!("Grade: A");
} else if score >= 80 {
    println!("Grade: B");
} else if score >= 70 {
    println!("Grade: C");
} else {
    println!("Grade: F");
}
```

**Logical operators:**
- `&&` AND (both conditions must be true)
- `||` OR (at least one condition must be true)
- `!` NOT (inverts the condition)

```rust
let age = 25;
let has_license = true;

if age >= 18 && has_license {
    println!("You can drive");
}

if age < 18 || !has_license {
    println!("You cannot drive");
}
```

**`if` as an expression (returns a value):**
```rust
let number = 10;
let result = if number > 5 {
    "big"  // No semicolon!
} else {
    "small"
};
println!("The number is {}", result);  // "big"
```

---

### ✅ 9. Loops

Loops allow repeating code multiple times. Rust has three main types:

#### 🔹 `for` Loop (most common for iterating)
Used when you know how many times to loop or want to iterate over a collection.

**Range syntax:**
```rust
for i in 1..6 {  // 1, 2, 3, 4, 5 (6 is excluded!)
    println!("{}", i);
}

for i in 1..=5 {  // 1, 2, 3, 4, 5 (5 is included with ..=)
    println!("{}", i);
}
```

**Iterate over array:**
```rust
let numbers = [10, 20, 30, 40];

for num in numbers.iter() {
    println!("{}", num);
}
```

**With index using `enumerate()`:**
```rust
let fruits = ["apple", "banana", "cherry"];

for (index, fruit) in fruits.iter().enumerate() {
    println!("Fruit {}: {}", index, fruit);
}
// Output:
// Fruit 0: apple
// Fruit 1: banana
// Fruit 2: cherry
```

**Reverse iteration:**
```rust
for i in (1..=5).rev() {  // 5, 4, 3, 2, 1
    println!("{}", i);
}
```

#### 🔹 `while` Loop (runs while condition is true)
Used when you don't know how many iterations you need in advance.

```rust
let mut i = 5;

while i > 0 {
    println!("{}", i);
    i -= 1;  // Don't forget to modify i, or infinite loop!
}
// Output: 5, 4, 3, 2, 1
```

**Example - waiting for user input simulation:**
```rust
let mut tries = 3;

while tries > 0 {
    println!("You have {} tries left", tries);
    tries -= 1;
}
```

#### 🔹 `loop` (infinite loop, must break manually)
Runs forever until you explicitly `break`.

```rust
let mut count = 0;

loop {
    count += 1;
    println!("{}", count);
    
    if count >= 5 {
        break;  // Exit the loop
    }
}
```

**`loop` with return value:**
```rust
let mut counter = 0;

let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2;  // Returns 20
    }
};

println!("Result: {}", result);  // 20
```

**`continue` (skip to next iteration):**
```rust
for i in 1..=10 {
    if i % 2 == 0 {
        continue;  // Skip even numbers
    }
    println!("{}", i);  // Only prints odd: 1, 3, 5, 7, 9
}
```

---

### ✅ 10. Functions
Functions are reusable blocks of code with a specific purpose.

**Basic structure:**
```rust
fn function_name(parameter: Type) -> ReturnType {
    // Code here
    return_value  // or use `return` keyword
}
```

**Simple function (no parameters, no return):**
```rust
fn greet() {
    println!("Hello!");
}

// Call it:
greet();
```

**Function with parameters:**
```rust
fn greet_person(name: &str) {
    println!("Hello, {}!", name);
}

// Call it:
greet_person("Tom");  // Output: Hello, Tom!
```

**Function with return value:**
```rust
fn add(a: i32, b: i32) -> i32 {
    a + b  // Last expression is returned (no semicolon!)
}

// Or explicitly:
fn add_explicit(a: i32, b: i32) -> i32 {
    return a + b;  // With return keyword
}

let sum = add(5, 3);  // sum = 8
```

**Function with multiple parameters:**
```rust
fn calculate_bmi(weight: f64, height: f64) -> f64 {
    weight / (height * height)
}

let bmi = calculate_bmi(70.0, 1.75);
println!("BMI: {:.1}", bmi);  // {:.1} formats to 1 decimal place
```

**Function with slice parameter (array reference):**
```rust
fn average(grades: &[i32]) -> i32 {
    let mut sum = 0;
    
    for grade in grades.iter() {
        sum += grade;
    }
    
    sum / grades.len() as i32  // .len() returns array length
}

let my_grades = [12, 8, 15];
let avg = average(&my_grades);  // & passes a reference
println!("Average: {}", avg);   // 11
```

**Why `&` (reference)?**
- Passing `&grades` lets the function borrow the array without taking ownership
- The original array is still usable after the function call
- More efficient (doesn't copy the data)

**Function scope:**
```rust
fn outer() {
    let x = 10;
    
    fn inner() {
        // Can't access x here! (different scope)
    }
    
    inner();
}
```

---

### ✅ 11. Arrays
Arrays are fixed-size collections of elements of the same type.

**Declaration:**
```rust
let grades = [12, 8, 15];  // Array of 3 i32 elements
```

**With explicit type and size:**
```rust
let numbers: [i32; 5] = [1, 2, 3, 4, 5];
// [type; size]
```

**Initialize with same value:**
```rust
let zeros = [0; 10];  // [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
```

**Accessing elements (zero-indexed):**
```rust
let fruits = ["apple", "banana", "cherry"];

println!("{}", fruits[0]);  // "apple"
println!("{}", fruits[1]);  // "banana"
println!("{}", fruits[2]);  // "cherry"
```

**Array length:**
```rust
let numbers = [10, 20, 30, 40];
let length = numbers.len();  // 4
```

**Modifying elements (array must be mutable):**
```rust
let mut scores = [5, 10, 15];
scores[1] = 20;  // scores is now [5, 20, 15]
```

**Iterating over an array:**
```rust
let grades = [12, 8, 15];

// Method 1: Using iter()
for grade in grades.iter() {
    println!("{}", grade);
}

// Method 2: Direct iteration
for grade in &grades {
    println!("{}", grade);
}

// Method 3: With index using enumerate()
for (i, grade) in grades.iter().enumerate() {
    println!("Grade {}: {}", i + 1, grade);
}
```

**Array vs Vector:**
- **Array:** Fixed size, stack-allocated, fast
  ```rust
  let arr = [1, 2, 3];  // Size cannot change
  ```
- **Vector:** Dynamic size, heap-allocated, flexible
  ```rust
  let mut vec = vec![1, 2, 3];
  vec.push(4);  // Can grow!
  ```

**Slices (borrowing part of an array):**
```rust
let numbers = [1, 2, 3, 4, 5];
let slice = &numbers[1..4];  // [2, 3, 4]
```

---

### ✅ 12. `enumerate()` - Looping with Index
`enumerate()` gives you both the **index** and the **value** during iteration.

**Basic usage:**
```rust
let grades = [12, 8, 15];

for (index, grade) in grades.iter().enumerate() {
    println!("Grade {}: {}", index + 1, grade);
}
// Output:
// Grade 1: 12
// Grade 2: 8
// Grade 3: 15
```

**Without enumerate (the hard way):**
```rust
let grades = [12, 8, 15];
let mut index = 0;

for grade in grades.iter() {
    println!("Grade {}: {}", index + 1, grade);
    index += 1;
}
```

**Why use it?**
- Cleaner code
- No manual counter needed
- Less prone to errors

**Starting index from different number:**
```rust
let items = ["First", "Second", "Third"];

for (i, item) in items.iter().enumerate() {
    println!("Item #{}: {}", i + 1, item);  // Start counting from 1
}
```

**Practical example - finding position:**
```rust
let names = ["Alice", "Bob", "Charlie", "Diana"];
let search = "Charlie";

for (i, name) in names.iter().enumerate() {
    if *name == search {
        println!("Found {} at position {}", search, i);  // Position 2
        break;
    }
}
```

**With strings:**
```rust
let sentence = "Hello Rust";

for (i, ch) in sentence.chars().enumerate() {
    println!("Character {} at position {}", ch, i);
}
```

---

### 🎯 Conclusion of hello_rust
You learned:
- **Project structure**: How Cargo organizes Rust projects with modules and files
- **Rust syntax basics**: Variables, types, and how Rust's safety features work
- **Control flow**: Conditions (`if`/`else`) and loops (`for`, `while`, `loop`)
- **Functions**: Creating reusable code blocks with parameters and return values
- **Collections**: Working with arrays and iterating with `enumerate()`
- **String handling**: Difference between `&str` and `String`
- **Math operations**: Arithmetic and the integer division gotcha
- **Rust's philosophy**: Immutability by default, explicit mutability, references

**Next steps to continue learning:**
1. **Error handling**: `Result` and `Option` types
2. **Ownership & borrowing**: Rust's unique memory management
3. **Structs**: Creating custom data types
4. **Enums & Pattern matching**: Powerful type-safe code
5. **Vectors**: Dynamic arrays that can grow
6. **HashMaps**: Key-value storage
7. **Traits**: Rust's interfaces
8. **Lifetimes**: Advanced memory safety

**Practice tips:**
- Try modifying the exercises to do different things
- Experiment with breaking the code to see error messages
- Read the Rust book: https://doc.rust-lang.org/book/
- Use `cargo doc --open` to see documentation of your dependencies

Keep practicing and don't worry about mistakes - Rust's compiler is very helpful and will guide you! 🦀

