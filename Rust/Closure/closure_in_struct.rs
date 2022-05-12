use std::thread;
use std::time::Duration;

/*
    Every closure as one of the following traits implemented.
    1. Fn,
    2. FnMut,
    3. FnOnce
*/

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    closure: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(closure: T) -> Cacher<T> {
        Cacher {
            closure,
            value: None,
        }
    }

    fn get(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(n) => n,
            None => {
                let n = (self.closure)(arg);
                self.value = Some(n);
                n
            }
        }
    }
}
/*
    Here, instead of passing at .get() method we can pass it
    on ::new() assoicate function itself.
*/

fn main() {
    let power2 = |num: u32| -> u32 {
        // expensive calculation simulation.
        println!("Calculating the power2");
        thread::sleep(Duration::from_secs(1));
        num * num
    };
    let mut cacher = Cacher::new(power2);

    println!("{}", cacher.get(5));
    println!("{}", cacher.get(5));
    println!("{}", cacher.get(10));
    /*
        Output:
        Calculating the power2
        25
        25
        25 (since, it doesn't check for the arg)
    */
}
