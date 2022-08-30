fn optional_content() -> String {
    let file_content = read_file(String::from(""));

    // match file_content {
    //     Some(value) => value,
    //     None => String::from("File not found"),
    // }

    // This is another way to implement the match statement
    if let Some(value) = file_content {
        value
    } else {
        String::from("Empty")
    }
}

fn read_file(_file_path: String) -> Option<String> {
    Some(String::from("File content"))
    // This is the equivalent from "null" in another languages such as Java or JavaScript
    // None
}

pub fn main() {
    println!("{}", optional_content());
}
