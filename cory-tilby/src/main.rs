extern crate getopts;

mod cli;
mod document;
mod repository;
mod search;

fn main() {
    match repository::setup() {
      Ok(_) => { },
      Err(e) => panic!("{:?}", e),
    }
    cli::parse();
}
