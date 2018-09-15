#![feature(plugin, decl_macro)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate rocket;

#[get("/")]
fn hello() -> String {
    format!("Try /hello/<name>/<age>")
}

#[get("/hello/<name>/<age>")]
fn hello_name_age(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

fn main() {
    rocket::ignite().mount("/", routes![hello,hello_name_age]).launch();
}