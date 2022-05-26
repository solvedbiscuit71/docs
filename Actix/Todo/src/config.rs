use dotenv_codegen::dotenv;

pub struct Env {
    pub host: String,
    pub port: u16,
}

impl Env {
    pub fn new() -> Self {
        Env {
            host: dotenv!("HOST").to_string(),
            port: dotenv!("PORT").to_string().parse().unwrap(),
        }
    }
}
