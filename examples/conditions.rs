fn main() {
    let age: u8 = 18;
    let responsible_give_authorization: bool = true;
    let is_of_age = age >= 18;

    if is_of_age {
        println!("Can buy drinks");
    } else if age > 16 && responsible_give_authorization {
        println!("Can buy drinks with the responsible signature");
    } else {
        println!("Cannot drink");
    }

    // The values "of age" and "minor" are used as expression to fill "condition" variable
    let condition = if is_of_age { "of age" } else { "minor" };

    println!("Is {} person", condition);
}
