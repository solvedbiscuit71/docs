fn main() {
    println!("{name} is static type {0}.", "language", name = "Rust");

    println!("Round off by 2 of 3.147 is {:.2}", 3.147);
    println!("37 in 8-bit binary = {:08b}", 37);

    println!("Right Align -> '{:>10}'", "Rust");
    println!("Right Align with Fill -> '0.{:0>8}'", 1);

    /*
     * similar to println!() they are,
     * 1. print!()
     * 2. format!()
     * 3. write!()
     */
}
