use std::f64::consts::PI;

fn main() {
    // ? Formatted print
    println!("PI is is roughly {}", format!("{:.4}", PI));
    println!(
        "Hello, I'm {name}, a {age} years olds coder from {country}...!",
        name = "@HHP",
        age = 23,
        country = "Vietnam"
    );
}
