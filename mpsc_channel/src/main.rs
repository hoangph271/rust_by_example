use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx_1 = tx.clone();
    thread::spawn(move || {
        let messages = vec![
            String::from("Rust"),
            String::from("by"),
            String::from("Example"),
        ];

        for message in messages {
            tx_1.send(message).unwrap();
            thread::sleep(Duration::from_millis(100))
        }
    });

    let tx_2 = tx.clone();
    thread::spawn(move || {
        let messages = vec![
            String::from("Close"),
            String::from("when"),
            String::from("This...!"),
        ];

        for message in messages {
            tx_2.send(message).unwrap();
            thread::sleep(Duration::from_millis(100))
        }
    });

    for received in rx {
        println!("Received: {}", received);
    }
}
