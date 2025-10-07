use std::sync:: mpsc;
use std::thread;
use std::time::Duration;

// We will use a channel for thread synchonization
// Channels are a common and idiomatic way to implement producer-consumer in rust with thread safety
fn main() {
    let (tx, rx) = mpsc::channel();
    let num_messages = 6;

    let producer = thread::spawn(move || {
        for i in 0..num_messages {
            tx.send(format!("Message {}", i)).unwrap();
            println!("Producer sent message: {}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });

    let consumer = thread::spawn(move || {
        for received in rx {
            println!("Consumer received: {}", received);
            thread::sleep(Duration::from_millis(200));
        }
    });
    producer.join().unwrap();
    consumer.join().unwrap();
}

