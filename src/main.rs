#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[cfg(test)] mod tests;

#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/hello/<name>")]
fn hi(name: String) -> String {
    name
}

#[get("/hello2/<name>/<last_name>")]
fn hi2(name: String, last_name: String) -> String {
    format!("Name: {}, last name: {}", name, last_name)
}


fn main() {
    rocket::ignite().mount("/", routes![hello, hi,hi2]).launch();
}