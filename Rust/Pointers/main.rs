fn main() {
    // In, C++ we have RAII but in Rust we have OBRM (Ownership based resource management)

    /*
        Ownership Rules:
        1. Each value in rust has a variable that called owner
        2. There can be only one owner at a time
        3. When owner goes out of scope, the value will be dropped
    */

    let _str = String::from("Pointers");
    println!("{}", _str);
}
// Here, _str is drop because it goes out of scope.
