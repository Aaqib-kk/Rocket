#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/hello")]
fn world() -> &'static str {
    "Hello, world!"
}
#[get("/hello/<name>")]
fn world_o(name: String) -> String {
    format!("Hello: {}!", name)
}
#[get("/hello/<num>")]
fn world_n(num: usize) -> String {
    format!("The Total Number is: {}!", num+50)
}

fn main() {
    rocket::ignite()
    .mount("/", routes![world, world_n])
    .launch();
}