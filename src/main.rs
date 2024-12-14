use std::fs;

fn main() {
    println!("Hello, world!");

    let file_result = fs::read_to_string("input.txt");

    let contents = match file_result {
        Ok(contents) => contents,
        Err(error) => panic!("fuck hit an {error}"),
    };

    println!("{contents}");
}
