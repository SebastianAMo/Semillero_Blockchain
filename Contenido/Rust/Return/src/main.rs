#![allow(unused)]
use std::fs::File;
use std::io::{Error, ErrorKind, Read, Result};


fn main() -> Result<()> {
    //ver el uso de return
    let mut file = match File::open("/Users/vfabian/Desarrollo/Rust/Semillero_utp/7/ejercios_clase/src/foo.txt") {
        Ok(f) => f,
        Err(e) => return Err(e),
    };

    let mut contents = String::new();
    let size = match file.read_to_string(&mut contents) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    if contents.contains("impossible!") {
        return Err(Error::new(ErrorKind::Other, "oh no!"));
    }

    if size > 9000 {
        return Err(Error::new(ErrorKind::Other, "over 9000!"));
    }

    assert_eq!(contents, "Hello, world!");
    Ok(())

    //Ejemplo de closures
    

}

