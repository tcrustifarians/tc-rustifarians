extern crate open;

use repository;
use std::fs::OpenOptions;

pub fn create(inp: &str) {
    let file_path = [ repository::TILBY, inp ].join("/");
    let file_path_slice = &file_path;

    match OpenOptions::new().create(true).open(file_path_slice) {
        Ok(file) => file,
        Err(e) => panic!("{:?}", e),
    };

    open::that(&file_path_slice).ok();
    if open::that(&file_path_slice).is_ok() {
        println!("w00t");
    }
}
