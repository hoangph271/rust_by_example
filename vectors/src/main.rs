#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mut v = vec![1, 2, 3, 4];
    // // ? Get by &v[i]
    // let third = &v[2];
    // println!("{}", third);

    // // ? Get by get & match
    // let tenth: i32;
    // match v.get(3) {
    //     Some(tenth) => println!("{}", tenth),
    //     None => println!("No 10th...!"),
    // }

    // // ! Won't work...!
    // let first = &v[0];
    // v.push(10);
    // println!("{}", first);
    // // ? Will work...!
    // let last = v[3];
    // v.push(11);
    // println!("{}", last);

    // // ? Iterating
    // for i in &v {
    //     println!("{}", i);
    // }
    // // ? Iterating & change
    // for i in &mut v {
    //     *i += 50;
    //     println!("{}", i);
    // }

    let mut rows = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    rows.push(SpreadsheetCell::Text(String::from("green")));

    for row in rows.into_iter() {
        println!("{:#?}", row);
    }
}
