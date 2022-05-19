use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
struct Point(i64, i64);

impl Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /* Here, semicolon doesn't come */
        write!(f, "({},{})", self.0, self.1)
    }
}

fn main() {
    let no = Point(25, -15);

    /* This will work as we implemented the Debug Trait */
    println!("{:?}", no);
    dbg!(&no);

    /* And, since we manually implemented Display Trait */
    println!("{}", no);

    /*
     * NOTE:
     * {} -> should implement the Display trait (manually done)
     * {:?} -> should implement the Debug trait (can be derived)
     * {:#?} -> pretty print (require Debug trait)
     */
}
