fn ownership() {
    let random_string = String::from("Wesley");
    steal(random_string);

    // print this value will give a error, because on the line 3, when the "random_string" is passed to steal method
    // the "ownership" of this value is passed too, so the value "Wesley" doesn't exist anymore on this scope, because
    // he was passed to the method "steal"
    // println!("{}", random_string);
}

fn steal(string: String) {
    println!("steal: {}", string);
}

fn ownership_with_borrow() {
    let random_string = String::from("Wesley");

    // Now I will pass not the value (with the ownership) of this variable, but I will pass a REFERENCE of random_string
    // And the ownership will remain on this scope
    borrow(&random_string);

    println!("ownership_with_borrow: {}", random_string);
}

fn borrow(string: &String) {
    println!("borrow: {}", string);
}

fn ownership_with_borrow_and_mutability() {
    let mut random_string = String::from("Wesley");

    // I need to pass the value with the "mut" annotation
    // The variable passed as reference need to be mutable too
    borrow_and_change(&mut random_string);

    println!("ownership_with_borrow_and_mutability: {}", random_string);
}

// The param need to receive the mut annotation to guarantee that this value is mutable
fn borrow_and_change(string: &mut String) {
    string.push_str(" Matos");
    println!("borrow_and_change: {}", string);
}

fn main() {
    ownership();
    ownership_with_borrow();
    ownership_with_borrow_and_mutability();
}
