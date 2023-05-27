fn fizz_buzz() {
    let mut counter = 0;

    for count in 1..=301 {
        if count % 3 == 0 && count % 5 == 0 {
            println!("fizz buzz");
            counter += 1;
        } else if count % 3 == 0 {
            println!("fizz");
        } else if count % 5 == 0 {
            println!("buzz");
        }
    }

    println!("Number of fizz buzz occurrences: {}", counter);
}

fn main() {
    println!("Welcome to the Fizz Buzz program!");

    fizz_buzz();
}
