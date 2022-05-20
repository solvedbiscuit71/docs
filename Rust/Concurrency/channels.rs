use std::mem;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/*
 * mpsc => multiple producer single consumer
 *
 * which means we can have more than one producer (transmitter) but only one consumer (reciever)
 * NOTE: transmitter.send() -> Result<T,E> -> Err() when reciever is drop!
 * the same is said for reciever!
 */
fn main() {
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || {
        let message = "Hi, there".to_owned();
        tx.send(message).unwrap();
    });

    // This is pause the current thread until we get Result<T,E> from rx.recv()
    // Alternative, rx.try_recv() -> immediately return Result<T,E>
    let message = rx.recv().unwrap();
    println!("Message from Thread: {}", message);
    mem::drop(rx);

    /* Multiple Producer */

    let (tx, rx) = mpsc::channel::<String>();
    let mut messages: Vec<String> = Vec::new();

    let tx2 = tx.clone();
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        tx2.send("Thread2 sending...".to_owned()).unwrap();
    });

    thread::spawn(move || {
        tx.send("Thread1 sending...".to_owned()).unwrap();
    });

    // This loop will break only after all transmitter is drop.
    for message in rx {
        messages.push(message);
    }

    println!("{:?}", messages);
}
