#![allow(dead_code)]
enum DaysOfTheWeek {
    Sunday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

fn is_weekend(day: DaysOfTheWeek) -> bool {
    match day {
        DaysOfTheWeek::Sunday | DaysOfTheWeek::Saturday => true,
        _ => false,
    }
}

pub fn main() {
    let day = DaysOfTheWeek::Tuesday;

    println!("is weekend? {}", is_weekend(day));

    colors(Color::Rgb(255, 255, 255));
}

#[allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue,
    Rgb(u8, u8, u8),
    Cymk {
        cyan: u8,
        magenta: u8,
        yellow: u8,
        black: u8,
    },
}

fn colors(color: Color) {
    println!(
        "Color = {}",
        match color {
            Color::Red => "red",
            Color::Green => "green",
            Color::Blue => "blue",
            Color::Rgb(0, 0, 0)
            | Color::Cymk {
                cyan: _,
                magenta: _,
                yellow: _,
                black: 255,
            } => "black",
            Color::Rgb(255, 255, 255) => "white",
            Color::Rgb(_, _, _) => "rgb unknown",
            Color::Cymk {
                cyan: _,
                magenta: _,
                yellow: _,
                black: _,
            } => "cymk unknown",
        }
    )
}
