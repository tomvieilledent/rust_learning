# Rust Exercises — Beginner

This repository contains 10 progressive Rust exercises to learn the basics: variables, conditions, loops, functions, arrays, and `String`.

---

## Project Structure

```
src/
  main.rs
  tasks/
    task_01.rs
    task_02.rs
    task_03.rs
    task_04.rs
    task_05.rs
    task_06.rs
    task_07.rs
    task_08.rs
    task_09.rs
    task_10.rs
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

## Exercise List

### Task 01 — Hello Rust
**Objective:** Display "Hello Rust" then 42  
**Expected output:**
```
Hello Rust
42
```

---

### Task 02 — Variable and Display
**Objective:** Declare `age = 20` and display  
**Expected output:**
```
I am 20 years old
```

---

### Task 03 — Sum of Two Numbers
**Objective:** Sum of `a = 10` and `b = 4`  
**Expected output:**
```
The sum is 14
```

---

### Task 04 — String and Display
**Objective:** Use a `String` and display  
**Expected output:**
```
Hello, Tom
```

---

### Task 05 — Average
**Objective:** Average of `x=10`, `y=20`, `z=30`  
**Expected output:**
```
The average is 20
```

---

### Task 06 — Adult / Minor Condition
**Objective:** Display adult if `age >= 18`  
**Expected output:**
```
You are an adult
```

---

### Task 07 — Boolean
**Objective:** Display a message based on `is_raining`  
**Expected output:**
```
Take an umbrella
```

---

### Task 08 — For Loop
**Objective:** Display 1 to 5 with a `for` loop  
**Expected output:**
```
1
2
3
4
5
```

---

### Task 09 — While Loop
**Objective:** Display 5 to 1 with a `while` loop  
**Expected output:**
```
5
4
3
2
1
```

---

### Task 10 — Array + Average + Condition
**Objective:**
- Display each grade
- Calculate the average
- Display "Passed" if average >= 10 otherwise "Failed"

**Expected output:**
```
Grade 1: 12
Grade 2: 8
Grade 3: 15
Average: 11
Passed
```

---

# Solutions (grouped)

### Task 01
```rust
pub fn run() {
    println!("Hello Rust");
    println!("42");
}
```

### Task 02
```rust
pub fn run() {
    let age = 20;
    println!("I am {} years old", age);
}
```

### Task 03
```rust
pub fn run() {
    let a = 10;
    let b = 4;
    let sum = a + b;
    println!("The sum is {}", sum);
}
```

### Task 04
```rust
pub fn run() {
    let name = String::from("Tom");
    println!("Hello, {}", name);
}
```

### Task 05
```rust
pub fn run() {
    let x = 10;
    let y = 20;
    let z = 30;
    let average = (x + y + z) / 3;
    println!("The average is {}", average);
}
```

### Task 06
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

### Task 07
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

### Task 08
```rust
pub fn run() {
    for i in 1..6 {
        println!("{}", i);
    }
}
```

### Task 09
```rust
pub fn run() {
    let mut i = 5;

    while i > 0 {
        println!("{}", i);
        i -= 1;
    }
}
```

### Task 10
```rust
pub fn run() {
    let grades = [12, 8, 15];

    for (index, grade) in grades.iter().enumerate() {
        println!("Grade {}: {}", index + 1, grade);
    }

    let avg = average(&grades);
    println!("Average: {}", avg);

    let result = if avg >= 10 {
        String::from("Passed")
    } else {
        String::from("Failed")
    };

    println!("{}", result);
}

fn average(grades: &[i32]) -> i32 {
    let mut sum = 0;

    for grade in grades.iter() {
        sum += grade;
    }

    sum / grades.len() as i32
}
```
