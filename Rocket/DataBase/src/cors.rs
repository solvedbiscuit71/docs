use rocket::{fairing::AdHoc, http::Header};

pub fn cors() -> AdHoc {
    AdHoc::on_response("CORS", |_, res| {
        Box::pin(async {
            res.set_header(Header::new("Access-Control-Allow-Origin", "*"));
            res.set_header(Header::new(
                "Access-Control-Allow-Methods",
                "POST, GET, PUT, DELETE",
            ));
            res.set_header(Header::new("Access-Control-Allow-Headers", "*"));
            res.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        })
    })
}
