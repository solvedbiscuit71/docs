use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/*
 * Mutex<T> <=> RefCell<T> but, it can be passed between thread safely.
 * Similarly, Arc<T> <=> Rc<T> but, it is atomic.
 *
 * Mutex has a lock which return a MutexGuard -> impl Deref & Drop
 * When the MutexGuard is drop! -> lock is free for other to use!
 *
 * NOTE: Mutex.lock() freeze the thread until it gets the lock();
 *
 * Deadlock -> when two thread accessing two mutex such a way that both thread freezes!
 */
fn main() {
    let count = Arc::new(Mutex::new(0));

    for _ in 0..10 {
        let count_mtx = count.clone();
        thread::spawn(move || {
            let mut value = count_mtx.lock().unwrap();
            *value += 1;
        });
    }

    thread::sleep(Duration::from_millis(100));
    println!("count -> {}", count.lock().unwrap());
}
