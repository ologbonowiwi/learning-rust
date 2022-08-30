fn pattern_matching() {
    for x in 1..=20 {
        let how_much = match x {
            1 => "A little",
            2 | 3 => "A little bit",
            4..=10 => "A bit",
            _ if x % 2 == 0 => "A good quantity",
            _ => "A lot",
        };

        println!("{} = {}", x, how_much);
    }
}

fn main() {
    pattern_matching();
}
