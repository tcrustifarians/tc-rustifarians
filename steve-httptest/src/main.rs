extern crate iron;
extern crate rustc_serialize;
extern crate postgres;

use iron::prelude::*;
use iron::status;
use iron::headers;
use iron::modifiers;
use postgres::{Connection, SslMode};

use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable)]
struct Greeting {
    msg: String
}

fn main() {
    fn hello_world(_: &mut Request) -> IronResult<Response> {
        let greeting = Greeting { msg: "Hello, World".to_string() };
        let payload = json::encode(&greeting).unwrap();

        Ok(Response::with((
          status::Ok, 
          modifiers::Header(headers::ContentType("application/json".parse().unwrap())),
          payload)))
    }

    println!("On 3030");
    let conn = Connection::connect("postgres://stevetuckner@localhost:5432/golf_development", &SslMode::None)
            .unwrap();

    let stmt = conn.prepare("SELECT id, email FROM users").unwrap();
    for row in stmt.query(&[]).unwrap() {
        let id:i32 = row.get(0);
        let email:String = row.get(1);
        println!("Found person id={} email={}", id, email);
    }

    Iron::new(hello_world).http("localhost:3030").unwrap();
}
