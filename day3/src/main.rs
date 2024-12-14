use std::fs;
use regex::Regex;

struct MulOp {
    x: i32,
    y: i32,
}

fn main() {

    let file_result = fs::read_to_string("input.txt");

    let contents = match file_result {
        Ok(contents) => contents,
        Err(error) => panic!("fuck hit an {error}"),
    };

    let mul_ops = get_mulops(&contents);
    let products: Vec<i32> = mul_ops.iter().map(|mul_op| mul_op.x * mul_op.y).collect();
    let mut sums = 0;
    for product in products {
        sums += product;
    }
    println!("Sum is {sums}");
}
// Hellodon't()worlddo()I'mdon't() Eric
fn get_mulops(contents: &str) -> Vec<MulOp> {
    let mut mul_ops: Vec<MulOp> = Vec::new();
    let no_newlines = contents.replace("\n", "");
    let dontdo_regex = Regex::new(r"(?Um)don't\(\)(.*)do\(\)").expect("Problem creating dodont regex");
    let mulop_regex = Regex::new(r"mul\((\d+),(\d+)\)").expect("Problem creating mul regex");

    let replaced_dont_do = dontdo_regex.replace_all(&no_newlines, "");
    println!("{replaced_dont_do}");
    let cleaned_line = match replaced_dont_do.contains("don't") {
        true => replaced_dont_do.split_once("don't()").expect("No ending don't()?").0,
        false => &replaced_dont_do
    };
    println!("clean changed? {}", cleaned_line == replaced_dont_do);
    for capture in mulop_regex.captures_iter(cleaned_line) {
        let x = capture.get(1).expect("Problem matching x").as_str().parse::<i32>().expect("Problem parsing x");
        let y = capture.get(2).expect("Problem matching y").as_str().parse::<i32>().expect("Problem parsing y");
        mul_ops.push(MulOp { x: x, y: y });
    }

    mul_ops
}
