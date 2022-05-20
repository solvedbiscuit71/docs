use std::thread::{self, JoinHandle};

fn main() {
    let handle: JoinHandle<()> = thread::spawn(|| {
        for i in 0..15 {
            println!("Spawn: {}", i);
        }
    });

    for i in 0..5 {
        println!("Main: {}", i);
    }

    /*
     * The printed value will change everytime because of the
     * operating system assign the thread.
     *
     * And, also note that, hanlde thread doesn't complete
     * because, of main thread terminated!
     */

    handle.join().unwrap(); // This stop the main thread until handle is completed!
}
