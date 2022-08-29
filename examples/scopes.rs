fn scope() {
    const PI: f32 = 3.14;
    println!("PI = {}, PI size = {}", PI, std::mem::size_of_val(&PI));

    static RANDOM_NUMBER: i8 = 12;
    println!("random static number = {}", RANDOM_NUMBER);

    let unsigned: u16 = 300;
    println!(
        "first declaration unsigned integer = {}, unsigned integer size = {} bytes",
        unsigned,
        std::mem::size_of_val(&unsigned)
    );
    // You can redeclare variables but compiler will show a warning if you doesn't use the first value.
    let unsigned: u16 = 301;
    println!(
        "second declaration unsigned integer = {}, unsigned integer size = {} bytes",
        unsigned,
        std::mem::size_of_val(&unsigned)
    );

    let decimal: f32 = 2.5;
    println!(
        "decimal = {}, decimal size = {} bytes",
        decimal,
        std::mem::size_of_val(&decimal)
    );

    let boolean: bool = true;
    println!(
        "Boolean = {}, boolean size = {} bytes",
        boolean,
        std::mem::size_of_val(&boolean)
    );

    let letter: char = 'C';
    println!(
        "Char = {}, char size = {}",
        letter,
        std::mem::size_of_val(&letter)
    );
}

fn main() {
    scope();

    // this will not work because variable is not on this scope
    // println!("decimal = {}", decimal);
}
