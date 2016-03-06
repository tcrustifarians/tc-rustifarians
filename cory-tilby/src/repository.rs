use std::fs;
use std::io;
use std::io::ErrorKind;

pub static TILBY: &'static str = "tilby";

pub fn setup() -> io::Result<()> {
    match fs::metadata(self::TILBY) {
        Ok(_) => { },
        Err(ref e) if e.kind() == ErrorKind::NotFound =>
            try!(fs::create_dir(self::TILBY)),
        Err(e) =>
            panic!("{:?}", e),
    }
    Ok(())
}
