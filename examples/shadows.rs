fn main() {
    let a = 123;

    {
        let b = 345;
        println!("internal b = {}", b);

        // a new scoped variable is created with the same name of the variable of the outside scope
        // this is called "shadowing"
        // u can check more details in: https://doc.rust-lang.org/rust-by-example/variable_bindings/scope.html
        let a = 777;
        println!("internal a = {}", a); // will return 777
    }
    // "a" of the inside scope doesn't exist anymore, and the value of a still 123

    // this will throw a compilation error
    // print!("external b = {}", b);

    println!("a = {}", a); // will return 123
}
