use std::thread;
use std::time::Duration;

use closures::Cacher;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        let ratioed_time = (2000.0 * 0.1) as f32;

        println!("Calculating slowly...!");
        thread::sleep(Duration::from_millis(ratioed_time.round() as u64));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity),);
        println!("Next, do {} situps!", expensive_result.value(intensity),);
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}
