
use std::io;
fn sum(numbers: &[i32]) -> i32 {
    let mut result = 0;
    for number in numbers {
        result += number;
    }
    result
}

/**
 * Modified version to accept user's input (Option 1)
 * This option adds the numbers "on the fly"
 */
fn sum_n_values(n_elements: u16) -> i32 {
    let mut result = 0;
    let mut input = String::new();
    // Read users input
    for i in 1..=n_elements {
        input.clear();
        println!("Enter your {} number:", i);
        io::stdin()
        .read_line(&mut input).expect("Failed to read input");
        let temp: i32 = input
        .trim() //Removes leading and trailing whitespaces and newlines (\n). Useful as user usually ends every input with an enter click (\n).
        .parse() //Transform the digits string into an integer.
        .expect("Failed to read input"); //If parse fails, print this and exit.
        result += temp;
    }
    result
}

/**
 * Modified version to accept user's input (Option 2)
 * This option creates a vector and adds the numbers after the vector is complete.
 */
fn sum_values_vector() -> i32 {
    let mut result = 0;
    let mut input = String::new();
    let mut elements: Vec<i32> = Vec::new();
    // Read users input
    loop {
        input.clear();
        println!("Finish this process by entering \"end\"\nEnter your number: ");
        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
        let input_temp = input
        .trim(); //Removes leading and trailing whitespaces and newlines (\n). Useful as user usually ends every input with an enter click (\n).
        if input_temp != "end" { //Ends loop when user asks.
            let temp: i32 = input_temp
            .parse() //Transform the digits string into an integer.
            .expect("Failed to read input"); //If parse fails, print this and exit.
            elements.push(temp);
        } else {
            break;
        }
    }
    for i in elements {
        result += i;
    }
    result
}


fn main() {
    // There are no variadic arguments in Rust
    let numbers = [1, 2, 3, 4, 5];
    let mut result = sum(&numbers);
    println!("The sum is {}", result);
    
    result = sum_n_values(3);
    println!("The sum is {}", result);

    result = sum_values_vector();
    println!("The sum is {}", result);
}
