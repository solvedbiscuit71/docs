fn main() {
    // In, C++ we have RAII but in Rust we have OBRM (Ownership based resource management)

    /*
       - Every value has only owner at a time.
       - Whenever, the owner goes out of scope it's value is *drop*
    */

    let _str = String::from("Pointers");
    println!("{}", _str);
}
// Here, _str is drop because it goes out of scope.
