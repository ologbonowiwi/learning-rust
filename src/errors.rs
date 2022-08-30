fn errors() {
    match result() {
        Ok(s) => println!("result fn success string = {}", s),
        Err(code) => println!("result fn error code = {}", code),
    };

    match error() {
        Ok(s) => println!("error fn success string = {}", s),
        Err(code) => println!("error fn error code = {}", code),
    };
}

fn result() -> Result<String, u8> {
    Ok(String::from("Everything is good"))
}

fn error() -> Result<String, u8> {
    Err(42)
}

fn main() {
    errors();
}
