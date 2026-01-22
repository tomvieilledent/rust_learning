/**
 * run - Calculate grades average and determine pass/fail status
 *
 * Description: Displays individual grades, computes their average,
 * and determines if the student passed or failed
 */
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

/**
 * average - Calculate the average of an array of grades
 * @grades: Reference to an array of i32 integers
 *
 * Description: Iterates through the grades array, sums all values,
 * and divides by the number of elements
 *
 * Return: The average as an i32
 */
fn average(grades: &[i32]) -> i32 {
    let mut sum = 0;
    
    for grade in grades.iter() {
        sum += grade;
    }
    
    sum / grades.len() as i32
}
