/**
 * run - Check weather and suggest umbrella
 *
 * Description: Uses a boolean to determine if an umbrella
 * is needed based on weather conditions
 */
pub fn run() {
    let is_raining = true;

    if is_raining
    {
        println!("Take an umbrella");
    }
    else
    {
        println!("No umbrella needed");
    }
}
