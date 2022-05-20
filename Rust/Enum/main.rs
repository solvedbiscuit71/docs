#![allow(dead_code)]

// struct like enum
enum Events {
    ProgramStart,
    ProgramStop,

    Input(String),
    Output(String),

    Panic { id: u16, message: String },
}

// C like enum
enum Number {
    One = 1,
    Two,
    Three,
}

use crate::Events::*;
fn inspect(e: Events) {
    match e {
        ProgramStart => println!("Program init"),
        ProgramStop => println!("Program completed"),
        Input(x) => println!("Input: {}", x),
        Output(x) => println!("Output: {}", x),
        Panic { id, message: mess } => {
            println!("{{\n\tError Code: {}\n\tMessage: '{}'\n}}", id, mess)
        }
    }
}

fn main() {
    let e1 = ProgramStart;
    let e2 = Input("/api".to_string());
    let e3 = Panic {
        id: 404,
        message: "Page not Found".to_string(),
    };
    let e4 = ProgramStop;

    inspect(e1);
    inspect(e2);
    inspect(e3);
    inspect(e4);

    println!("enum Number::Two has a value of {}", Number::Two as i32);
}
