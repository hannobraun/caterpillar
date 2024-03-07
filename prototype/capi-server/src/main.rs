#[rocket::launch]
fn rocket() -> _ {
    rocket::build().mount("/", rocket::fs::FileServer::from("."))
}
