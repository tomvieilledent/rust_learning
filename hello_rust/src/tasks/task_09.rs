/**
 * run - Display numbers in descending order
 *
 * Description: Uses a while loop to count down from 5 to 1
 */
pub fn run() {
    let mut i = 5;

    while i > 0
    {
       println!("{}", i);
       i -= 1;
    }
}
