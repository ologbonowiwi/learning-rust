fn sum(a: i32, b: i32) -> i32 {
    // ; will supress expression values
    // the last expression without ; will return the expression value
    a + b
}

fn main() {
    let a = 50;
    let b = 5943;

    println!("Sum of {} and {} = {}", a, b, sum(a, b));
}
