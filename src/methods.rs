fn sum(a: i32, b: i32) -> i32 {
    // ; will supress expression values
    // the last expression without ; will return the expression value
    a + b
}

fn main() {
    let a = 50;
    let b = 5943;

    println!("{} + {} = {}", a, b, sum(a, b));
    println!("{} * {} = {}", a, b, times(a, b));
}

// Methods can be declared after where they are called
fn times(a: i32, b: i32) -> i32 {
    a * b
}
