#![allow(missing_docs)]
#![feature(plugin)]
#![plugin(rocket_codegen)]
#![allow(unmounted_route)]

#[macro_use]
extern crate pretty_assertions;
extern crate rocket;
extern crate rocket_contrib;
extern crate rocket_openapi;
extern crate serde_json;

use std::io::Read;
use std::fs::File;

pub use rocket::config::{ConfigBuilder, Environment};
pub use rocket::request::State;
pub use rocket::Rocket;
pub use rocket_contrib::JSON;
pub use serde_json::{Value, Map};

mod test_endpoints {
    use super::*;

    #[get("/hello")]
    fn hello() -> &'static str {
        "world"
    }

    #[get("/hello/<world>")]
    fn hello_param(world: String) -> String {
        world
    }

    // returns a configured Rocket, but with no mounts.
    pub fn ignite() -> Rocket {
        let config = ConfigBuilder::new(Environment::Development);
        let port = 0_u16;

        let config = config.log_level(rocket::config::LoggingLevel::Debug);
        let config = config.port(port);
        let config = config.finalize().expect("Rocket config incorrectly setup");

        rocket::custom(config, true)
    }
}

fn test_data(path: &str) -> String {
    let mut test_file = File::open(path).expect("could not open test file");
    let mut test_data = String::new();
    test_file.read_to_string(&mut test_data).expect("could not read test data");
    test_data
}

#[test]
fn test_hello() {
    let lit = test_endpoints::ignite();
    let lit = lit.mount("/", routes![test_endpoints::hello]);

    let spec = rocket_openapi::build_swagger_spec(&lit, "Hello World", "1.0");
    let test_data = test_data("tests/basic_rocket_test_hello.json");
    
    let json = serde_json::to_string_pretty(&spec).expect("cound not serialize json");
    assert_eq!(json, test_data);
}

#[test]
fn test_hello_param() {
    let lit = test_endpoints::ignite();
    let lit = lit.mount("/", routes![test_endpoints::hello_param]);

    let spec = rocket_openapi::build_swagger_spec(&lit, "Hello World", "1.0");
    let test_data = test_data("tests/basic_rocket_test_hello_param.json");
    
    let json = serde_json::to_string_pretty(&spec).expect("cound not serialize json");
    assert_eq!(json, test_data);
}