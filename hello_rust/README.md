# Rust Exercises — Beginner

This repository contains 10 progressive Rust exercises to learn the basics: variables, conditions, loops, functions, arrays, and `String`.

These exercises are designed to be completed in order, with each one building on concepts from previous exercises.

---

## 🚀 Getting Started

### Prerequisites
- Install Rust: https://www.rust-lang.org/tools/install
- Verify installation: `rustc --version`

### Running the Exercises

1. Navigate to the project folder:
```bash
cd hello_rust
```

2. Open `src/main.rs` and uncomment the task you want to run:
```rust
mod tasks;

fn main() {
    tasks::task_01::run();  // ← Active
    // tasks::task_02::run();  // ← Commented out
    // tasks::task_03::run();
    // ...
}
```

3. Run the program:
```bash
cargo run
```

4. Check the output matches the expected output for that task

### Tips
- Only activate one task at a time to see its output clearly
- Read the task description before looking at the solution
- Try to solve it yourself first!
- If stuck, check the solution at the bottom of this file
- Experiment by changing values to see what happens

---

## Project Structure

```
hello_rust/
├── Cargo.toml           # Project configuration
├── src/
│   ├── main.rs          # Entry point (activate tasks here)
│   └── tasks/
│       ├── mod.rs       # Module declaration
│       ├── task_01.rs   # Each task in its own file
│       ├── task_02.rs
│       └── ...
└── target/              # Compiled code (generated automatically)
```

In `main.rs`, activate the task you want to execute:

```rust
mod tasks;

fn main() {
    tasks::task_01::run();
    // tasks::task_02::run();
    // ...
}
```

Then run:

```bash
cargo run
```

---

## 📝 Exercise List

### Task 01 — Hello Rust
**Concepts:** `println!` macro, basic output  
**Objective:** Display "Hello Rust" then the number 42

**What you'll learn:**
- How to use `println!` to print text
- Printing different types (text and numbers)
- Each `println!` creates a new line

**Instructions:**
1. Use `println!` to display the text "Hello Rust"
2. Use another `println!` to display the number 42

**Expected output:**
```
Hello Rust
42
```

---

### Task 02 — Variable and Display
**Concepts:** Variables with `let`, string formatting  
**Objective:** Declare a variable `age = 20` and display it in a sentence

**What you'll learn:**
- Creating variables with `let`
- Type inference (Rust guesses the type)
- Using `{}` placeholders in `println!`

**Instructions:**
1. Create a variable `age` with value 20
2. Print "I am 20 years old" using the variable

**Expected output:**
```
I am 20 years old
```

---

### Task 03 — Sum of Two Numbers
**Concepts:** Multiple variables, arithmetic operations  
**Objective:** Calculate and display the sum of two numbers

**What you'll learn:**
- Working with multiple variables
- Addition operation with `+`
- Storing calculation results

**Instructions:**
1. Create variable `a` with value 10
2. Create variable `b` with value 4
3. Calculate their sum and store in variable `sum`
4. Display "The sum is 14"

**Expected output:**
```
The sum is 14
```

---

### Task 04 — String and Display
**Concepts:** `String` type, `String::from()`  
**Objective:** Create a `String` variable and display it in a greeting

**What you'll learn:**
- Difference between `&str` and `String`
- Using `String::from()` to create owned strings
- Strings can be used in placeholders too

**Instructions:**
1. Create a `String` variable `name` with value "Tom"
2. Display "Hello, Tom" using the variable

**Expected output:**
```
Hello, Tom
```

---

### Task 05 — Average
**Concepts:** Multiple operations, integer division  
**Objective:** Calculate the average of three numbers

**What you'll learn:**
- Parentheses for order of operations
- Integer division (important gotcha!)
- Working with multiple variables

**Instructions:**
1. Create three variables: `x=10`, `y=20`, `z=30`
2. Calculate their average: `(x + y + z) / 3`
3. Display "The average is 20"

**Expected output:**
```
The average is 20
```

**Challenge:** 
Try with values that don't divide evenly and see what happens. What does (10 + 15 + 16) / 3 give you?

---

### Task 06 — Adult / Minor Condition
**Concepts:** `if`/`else`, comparison operators  
**Objective:** Check if someone is an adult or minor based on age

**What you'll learn:**
- Using `if` and `else` for branching logic
- Comparison operator `>=` (greater than or equal)
- Code blocks with `{}`

**Instructions:**
1. Create variable `age` with value 18
2. If `age >= 18`, print "You are an adult"
3. Otherwise, print "You are a minor"

**Expected output:**
```
You are an adult
```

**Challenge:**
Try changing the age to 17 or 25 and see the different output!

---

### Task 07 — Boolean
**Concepts:** `bool` type, conditional based on boolean  
**Objective:** Display a message based on a boolean variable

**What you'll learn:**
- Boolean type can be `true` or `false`
- Using booleans directly in `if` conditions
- Common pattern for flags/switches

**Instructions:**
1. Create a boolean variable `is_raining` set to `true`
2. If it's raining, print "Take an umbrella"
3. Otherwise, print "No umbrella needed"

**Expected output:**
```
Take an umbrella
```

**Challenge:**
Change `is_raining` to `false` and see the different message!

---

### Task 08 — For Loop
**Concepts:** `for` loops, ranges  
**Objective:** Display numbers 1 to 5 using a loop

**What you'll learn:**
- `for` loop syntax
- Range syntax: `1..6` means 1,2,3,4,5 (6 excluded!)
- Loop variables are automatically created

**Instructions:**
1. Create a `for` loop with `i` from 1 to 5
2. Print each value of `i`

**Expected output:**
```
1
2
3
4
5
```

**Challenge:**
- Try `1..=5` instead and verify it works the same
- Try printing numbers in reverse: `(1..=5).rev()`

---

### Task 09 — While Loop
**Concepts:** `while` loops, mutable variables  
**Objective:** Display countdown from 5 to 1

**What you'll learn:**
- `while` loops repeat while condition is true
- Need `mut` to modify the loop counter
- Must update the counter to avoid infinite loops!

**Instructions:**
1. Create a mutable variable `i` with value 5
2. While `i > 0`, print `i` and then decrement it

**Expected output:**
```
5
4
3
2
1
```

**Challenge:**
What happens if you change `i > 0` to `i >= 0`? Try it!

---

### Task 10 — Array + Average + Condition
**Concepts:** Arrays, iteration, functions, combining concepts  
**Objective:** Work with an array of grades - display them, calculate average, determine pass/fail

This is the most complex exercise, combining everything you've learned!

**What you'll learn:**
- Creating and using arrays
- Iterating with `enumerate()` to get index + value
- Calling functions with array references (`&`)
- Creating a helper function
- Using `if` as an expression to assign values

**Instructions:**
1. Create an array `grades` with values `[12, 8, 15]`
2. Loop through and display each grade with its number
3. Create a function `average` that calculates the mean
4. Call the function and display the average
5. Determine if "Passed" (avg >= 10) or "Failed" and display

**Expected output:**
```
Grade 1: 12
Grade 2: 8
Grade 3: 15
Average: 11
Passed
```

**Challenge:**
- Try adding more grades to the array
- Change the passing grade threshold
- Add grade categories (A, B, C, D, F) based on average

---

# 📖 Solutions

**⚠️ Try to solve each exercise yourself before looking at the solutions!**

The solutions are provided here for reference and to help if you get stuck. Learning happens through struggle and problem-solving, so give each task a genuine attempt first.

### Understanding the Solutions

Each solution follows these patterns:
- `pub fn run()` makes the function public so it can be called from `main.rs`
- Variables are declared with descriptive names
- Code is kept simple and readable

---

### Task 01 — Hello Rust
```rust
pub fn run() {
    println!("Hello Rust");  // Print text in quotes
    println!("42");          // Numbers can be in quotes too
    // Or: println!(42);     // Numbers without quotes work too
}
```

**Explanation:**
- `println!` is a macro (notice the `!`)
- Each call creates a new line
- You can print numbers as text (in quotes) or as numbers (no quotes)

---

### Task 02 — Variable and Display
```rust
pub fn run() {
    let age = 20;                          // Declare variable
    println!("I am {} years old", age);    // {} is replaced by age
}
```

**Explanation:**
- `let age = 20;` creates an immutable variable
- Rust infers the type as `i32` (32-bit integer)
- The `{}` placeholder gets replaced with the value of `age`
- This is called "string formatting"

---

### Task 03 — Sum of Two Numbers
```rust
pub fn run() {
    let a = 10;          // First number
    let b = 4;           // Second number
    let sum = a + b;     // Calculate sum (14)
    println!("The sum is {}", sum);
}
```

**Explanation:**
- We can do arithmetic when declaring variables
- `+` operator adds the numbers
- The result is stored in `sum`

**Alternative - inline calculation:**
```rust
pub fn run() {
    let a = 10;
    let b = 4;
    println!("The sum is {}", a + b);  // Calculate directly in println
}
```

---

### Task 04 — String and Display
```rust
pub fn run() {
    let name = String::from("Tom");     // Create owned String
    println!("Hello, {}", name);
}
```

**Explanation:**
- `String::from("Tom")` creates a `String` (growable, heap-allocated)
- Different from `&str` which is a fixed string slice
- Use `String` when you need to own and possibly modify the text

**Alternative with &str:**
```rust
pub fn run() {
    let name = "Tom";  // This is &str (string slice)
    println!("Hello, {}", name);
}
```
Both work for this simple case, but `String` is more flexible.

---

### Task 05 — Average
```rust
pub fn run() {
    let x = 10;
    let y = 20;
    let z = 30;
    let average = (x + y + z) / 3;      // Parentheses ensure correct order
    println!("The average is {}", average);
}
```

**Explanation:**
- Parentheses `()` ensure addition happens before division
- This is **integer division**: (10 + 20 + 30) / 3 = 60 / 3 = 20
- Result has no decimal places (both operands are integers)

**For decimal results:**
```rust
pub fn run() {
    let x = 10.0;  // Note the .0 makes these floats
    let y = 20.0;
    let z = 30.0;
    let average = (x + y + z) / 3.0;
    println!("The average is {}", average);  // 20.0
}
```

---

### Task 06 — Adult / Minor Condition
```rust
pub fn run() {
    let age = 18;

    if age >= 18 {
        println!("You are an adult");
    } else {
        println!("You are a minor");
    }
}
```

**Explanation:**
- `if` checks the condition `age >= 18`
- If true, executes first block
- If false, executes `else` block
- `>=` means "greater than or equal to"

**Extended version with multiple conditions:**
```rust
pub fn run() {
    let age = 18;

    if age >= 18 {
        println!("You are an adult");
    } else if age >= 13 {
        println!("You are a teenager");
    } else {
        println!("You are a child");
    }
}
```

---

### Task 07 — Boolean
```rust
pub fn run() {
    let is_raining = true;

    if is_raining {
        println!("Take an umbrella");
    } else {
        println!("No umbrella needed");
    }
}
```

**Explanation:**
- `is_raining` is a `bool` type (true or false)
- No need to write `if is_raining == true`
- Just `if is_raining` is idiomatic Rust

**Common pattern:**
```rust
pub fn run() {
    let has_permission = true;
    let is_admin = false;
    
    if has_permission && !is_admin {  // && is AND, ! is NOT
        println!("Regular user access granted");
    }
}
```

---

### Task 08 — For Loop
```rust
pub fn run() {
    for i in 1..6 {      // 1..6 means 1,2,3,4,5 (6 excluded)
        println!("{}", i);
    }
}
```

**Explanation:**
- `1..6` is a range that goes from 1 to 5
- The upper bound (6) is **excluded**
- `i` is automatically created by the loop

**Alternative with inclusive range:**
```rust
pub fn run() {
    for i in 1..=5 {     // ..= includes the upper bound
        println!("{}", i);
    }
}
```

**Reverse:**
```rust
pub fn run() {
    for i in (1..=5).rev() {  // 5, 4, 3, 2, 1
        println!("{}", i);
    }
}
```

---

### Task 09 — While Loop
```rust
pub fn run() {
    let mut i = 5;       // Must be mutable!

    while i > 0 {        // Loop while condition is true
        println!("{}", i);
        i -= 1;          // Decrement (i = i - 1)
    }
}
```

**Explanation:**
- `mut` is necessary because we modify `i`
- `while i > 0` checks condition before each iteration
- `i -= 1` is shorthand for `i = i - 1`
- Loop stops when `i` becomes 0

**⚠️ Common mistake:**
```rust
// DON'T DO THIS - infinite loop!
let mut i = 5;
while i > 0 {
    println!("{}", i);
    // Forgot to decrement! Infinite loop!
}
```

**Alternative with break:**
```rust
pub fn run() {
    let mut i = 5;
    loop {  // Infinite loop
        println!("{}", i);
        i -= 1;
        if i == 0 {
            break;  // Exit loop
        }
    }
}
```

---

### Task 10 — Array + Average + Condition
```rust
pub fn run() {
    let grades = [12, 8, 15];  // Array of 3 integers

    // Display each grade with its number
    for (index, grade) in grades.iter().enumerate() {
        println!("Grade {}: {}", index + 1, grade);
    }

    // Calculate and display average
    let avg = average(&grades);  // & passes reference
    println!("Average: {}", avg);

    // Determine pass/fail using if as expression
    let result = if avg >= 10 {
        String::from("Passed")
    } else {
        String::from("Failed")
    };

    println!("{}", result);
}

// Helper function to calculate average
fn average(grades: &[i32]) -> i32 {
    let mut sum = 0;

    for grade in grades.iter() {
        sum += grade;  // Add each grade to sum
    }

    sum / grades.len() as i32  // Divide by count
}
```

**Explanation line by line:**

1. **Array creation:**
   ```rust
   let grades = [12, 8, 15];
   ```
   - Fixed-size array of 3 integers

2. **Enumerate loop:**
   ```rust
   for (index, grade) in grades.iter().enumerate() {
   ```
   - `.iter()` creates an iterator over the array
   - `.enumerate()` adds index to each item
   - `index` starts at 0, so we print `index + 1`

3. **Function call with reference:**
   ```rust
   let avg = average(&grades);
   ```
   - `&grades` passes a reference (borrow) not ownership
   - Function can read the array but doesn't take it away

4. **If expression:**
   ```rust
   let result = if avg >= 10 {
       String::from("Passed")  // No semicolon!
   } else {
       String::from("Failed")
   };
   ```
   - `if` can return a value in Rust
   - No semicolon on the returned values
   - Both branches must return the same type

5. **Average function:**
   ```rust
   fn average(grades: &[i32]) -> i32 {
   ```
   - `&[i32]` means "slice of i32" (reference to array)
   - `-> i32` specifies return type
   - Last expression is returned (no `return` keyword needed)

6. **Array length:**
   ```rust
   sum / grades.len() as i32
   ```
   - `.len()` returns `usize` type
   - `as i32` converts to i32 for division

**Enhanced version with more features:**
```rust
pub fn run() {
    let grades = [12, 8, 15, 18, 10];
    
    // Find highest grade
    let mut highest = grades[0];
    for grade in grades.iter() {
        if *grade > highest {
            highest = *grade;
        }
    }
    
    println!("Highest grade: {}", highest);
    
    // Display all grades
    for (index, grade) in grades.iter().enumerate() {
        let status = if *grade >= 10 { "✓" } else { "✗" };
        println!("Grade {}: {} {}", index + 1, grade, status);
    }
    
    let avg = average(&grades);
    println!("Average: {}", avg);
    
    // Letter grade
    let letter = if avg >= 15 {
        'A'
    } else if avg >= 12 {
        'B'
    } else if avg >= 10 {
        'C'
    } else {
        'F'
    };
    
    println!("Letter grade: {}", letter);
}

fn average(grades: &[i32]) -> i32 {
    grades.iter().sum::<i32>() / grades.len() as i32
}
```

---

## 🎓 Learning Tips

**After completing these exercises:**

1. **Experiment!** Modify the solutions and see what breaks
2. **Read error messages** - Rust's compiler is very helpful
3. **Try combinations** - Mix concepts from different tasks
4. **Challenge yourself:**
   - Add more complex conditions
   - Create your own functions
   - Work with bigger arrays
   - Combine multiple loops

**Common beginner mistakes to watch for:**
- Forgetting `mut` when you need to modify a variable
- Using `=` (assignment) instead of `==` (comparison)
- Forgetting semicolons `;` at end of statements
- Not understanding `..` vs `..=` in ranges
- Integer division giving unexpected results

**Next steps:**
- Check out the Learning Summary for deeper explanations
- Read "The Rust Book": https://doc.rust-lang.org/book/
- Try Rustlings exercises: https://github.com/rust-lang/rustlings
- Practice on Exercism: https://exercism.org/tracks/rust

Happy coding! 🦀

