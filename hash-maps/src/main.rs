use std::collections::HashMap;

fn main() {
    // * Basic syntax for creating empty hash maps
    // let mut birth_days = HashMap::new();
    // * Hash maps from iterators
    let mut birth_days: HashMap<_, _> = vec![String::from("@Phanh")]
        .into_iter()
        .zip(vec![String::from("03/09")].into_iter())
        .collect();
    let hhp_id = String::from("@HHP");
    let hhp_birth_days = String::from("27/01");

    birth_days.insert(hhp_id, hhp_birth_days);

    println!("{:?}", birth_days);
    // println!("{:?}", hhp_id); // ! Panic
    // println!("{:?}", hhp_birth_days); // ! Panic
    // ? These keys, values are OWNED by hash map

    // * Get elements
    let hhp_birth_days = birth_days.get("@HHP");
    println!("{:?}", hhp_birth_days);

    match hhp_birth_days {
        Some(birth_days) => println!("{:?}", birth_days),
        None => println!("No @HHP birthday...!"),
    }
    let vhp_birth_days = birth_days.get("@VHP");
    println!("{:?}", hhp_birth_days);

    match vhp_birth_days {
        Some(birth_days) => println!("{:?}", birth_days),
        None => println!("No @VHP birthday...!"),
    }
    // * Over writing
    birth_days.insert(String::from("@HHP"), "05/03".to_string());
    // * Insert if NOT exists
    birth_days
        .entry(String::from("@VHP"))
        .or_insert("05/03".to_string());

    match birth_days.get("@VHP") {
        Some(birth_days) => println!("Got @VHP: {:?}", birth_days),
        None => println!("No @VHP birthday...!"),
    }
    // * Update based on current values
    let text = "Updating a Value Based on the Old Value";
    let mut char_count = HashMap::new();
    let chars = text.split("");
    for c in chars {
        let count = char_count.entry(c).or_insert(0);
        *count += 1;
    }
    println!("{:?}", char_count);
}
