fn main() {
    // * Push to string
    let mut s = String::from("안녕하세요");
    s.push_str("...");
    s.push('!');
    println!("{}", s);

    // * Combine two strings
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    // ! note s1 has been moved here
    // ! and can no longer be used
    // * Combine more strings
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);
    // ? String is actually a wrapper over a  Vec<u8>
    let hello = String::from("नमस्ते");
    // ? hello is actually thse scalar values: ['न', 'म', 'स', '्', 'त', 'े']
    // * Slicing strings 
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    // let s = &hello[0..1]; // ! This would panic
    println!("{}", s);
    // * Iterating over string chars
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    // * Iterating over string bytes
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
    // ? crate.io for more stuffs
}
