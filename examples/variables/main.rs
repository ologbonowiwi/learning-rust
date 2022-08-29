fn main() {
    // When u use const, rust will replace all locations of your code that uses the constant
    // Const should contain a type, need an explicit type annotation, need to be initialized with constant function/values
    const PI: f32 = 3.14;
    println!("PI = {}, PI size = {}", PI, std::mem::size_of_val(&PI));

    // Static works similar to const, but the static value will represent a location in memory, as an variable.
    // The compiler will suggest you to change the name of your statics to use UPPER CASE names.
    // Usually u should use const instead static.
    // Read more informations about const and static variables in: https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/const-and-static.html
    static RANDOM_NUMBER: i8 = 12;
    println!("random static number = {}", RANDOM_NUMBER);

    let unsigned: u16 = 300;
    println!(
        "unsigned integer = {}, unsigned integer size = {} bytes",
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
