fn main() {
    let random_string = String::from("Wesley");
    steal(random_string);

    // print this value will give a error, because on the line 3, when the "random_string" is passed to steal method
    // the "ownership" of this value is passed too, so the value "Wesley" doesn't exist anymore on this scope, because
    // he was passed to the method "steal"
    // println!("{}", random_string);
}

fn steal(string: String) {
    println!("{}", string);
}
