fn with_while(multiplier: u8) {
    let mut counter: i8 = 0;

    while counter < 10 {
        counter += 1;
        println!(
            "{} * {} = {}",
            counter,
            multiplier,
            i16::from(counter) * i16::from(multiplier)
        );
    }
}

fn with_loop(multiplier: u8) {
    let mut counter: i8 = 0;
    loop {
        if counter >= 10 {
            break;
        }

        counter += 1;
        println!(
            "{} * {} = {}",
            counter,
            multiplier,
            i16::from(counter) * i16::from(multiplier)
        );
    }
}

fn main() {
    println!("----------------------- While -----------------------");
    with_while(255);

    println!("----------------------- Loop ----------------------- ");
    with_loop(255);
}
