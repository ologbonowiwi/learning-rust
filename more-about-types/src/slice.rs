pub fn main() {
    // use value with f32 annotation will parse the value to float 32.
    let grades: [f32; 4] = [10f32, 8f32, 9.5, 6.0];
    let integer: usize = 0;

    println!("{}", grades[integer]);

    // This will start the list with 4 positions, with the 6.5 value
    // let grades = [6.5; 4];
    for grade in 0..grades.len() {
        println!("Grade {} = {}", grade, grades[grade]);
    }

    matrix();
}

fn matrix() {
    let matrix = [[0.0, 1.2, 0.1], [1.3, 0.3, 1.4]];

    for line in matrix {
        for item in line {
            println!("Item = {}", item);
        }
    }
}
