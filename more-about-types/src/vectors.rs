pub fn main() {
    // The vector can be created with a start value, using the macro "vec!"
    // The result will be
    // let mut grades: Vec<f32> = vec![10f32, 8.8, 6.5];
    let mut grades: Vec<f32> = Vec::new();

    grades.push(10f32);
    grades.push(8.8);
    grades.push(6.5);

    // The "{:?}" is a debug annotation. It allows to see things that usually we can't see as a string (like and object, a vector, etc)
    // You can also use the "{:#?}" annotation for pretty print
    println!("{:?}", grades);

    grades.push(6.5);
    println!("{:?}", grades);

    /*
    * This is the match pattern equivalent to iterate removing data from array
    while let Some(grade) = grades.pop() {
        println!("Removed value = {}", grade);
    }
    */

    for grade in &grades {
        println!("Grade = {}", grade);
    }

    println!("{:?}", grades);
}
