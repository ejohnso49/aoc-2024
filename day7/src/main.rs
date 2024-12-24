use std::{fmt::Display, fs, ops::{Add, Mul}};
use regex::Regex;

#[derive(Debug)]
struct Equation {
    answer: i64,
    terms: Vec<i64>,
    cat_operation: bool,
}

impl Equation {
    fn new(answer: i64, terms: Vec<i64>) -> Self {
        Equation{answer, terms, cat_operation: false}
    }

    fn is_valid(&self) -> bool {
        match self.cat_operation {
            false => self.is_valid_no_cat(),
            true => self.is_valid_cat(),
        }
    }

    fn is_valid_no_cat(&self) -> bool {
        let term_length = self.terms.len();
        for mut i in 0..2_i64.pow(term_length.try_into().unwrap()) {
            let mut result: i64 = self.terms[0];
            // print!("{}: {} ", self.answer, result);
            for term in &self.terms[1..] {
                let operator = Operator::get(i);
                // print!("{operator} {term} ");
                result = operator.apply(result, *term);
                if result >= self.answer {
                    break
                }

                i = i >> 1;
            }
            // print!("\n");
            // println!("{result} => {}", self.answer);

            if result == self.answer {
                return true
            }
        }
        false
    }

    fn is_valid_cat(&self) -> bool {
        let term_length = self.terms.len();
        for mut i in 0..3_i64.pow(term_length.try_into().unwrap()) {
            let mut result: i64 = self.terms[0];
            // print!("{}: {} ", self.answer, result);
            for term in &self.terms[1..] {
                let operator = Operator::get_cat(i);
                // print!("{operator} {term} ");
                result = operator.apply(result, *term);
                if result >= self.answer {
                    break
                }

                i = i / 3;
            }
            // print!("\n");
            // println!("{result} => {}", self.answer);

            if result == self.answer {
                return true
            }
        }
       false
    }
}

enum Operator {
    Addition,
    Multiplication,
    Concatenate,
}

impl Operator {
    fn apply(&self, a: i64, b: i64) -> i64 {
        match self {
            Operator::Addition => a.add(b),
            Operator::Multiplication => a.mul(b),
            Operator::Concatenate => {
                let mut a_shift = a;
                let mut b_shift = b;
                while b_shift > 0 {
                    a_shift = a_shift * 10;
                    b_shift = b_shift / 10;
                }
                a_shift.add(b)
            }
        }
    }

    fn get(index: i64) -> Self {
        match index & 0x01 == 0 {
            true => Operator::Addition,
            false => Operator::Multiplication
        }
    }

    fn get_cat(index: i64) -> Self {
        match index % 3  {
            0 => Operator::Addition,
            1 => Operator::Multiplication,
            2 => Operator::Concatenate,
            _ => panic!("Weird!"),
        }
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Addition => write!(f, "+"),
            Operator::Multiplication => write!(f, "*"),
            Operator::Concatenate => write!(f, "||"),
        }
    }
}

fn main() {
    let file_result = fs::read_to_string("input.txt");

    let contents = match file_result {
        Ok(contents) => contents,
        Err(error) => panic!("fuck hit an {error}"),
    };

    let mut result: i64 = 0;
    let mut equations = get_equations(&contents);
    for equation in &equations {
        if equation.is_valid() {
            result += equation.answer;
        }
    }
    println!("{result}");

    for equation in &mut equations {
        equation.cat_operation = true;
    }

    result = 0;
    for equation in &equations {
        if equation.is_valid() {
            result += equation.answer;
        }
    }
    println!("{result}");
}

fn get_equations(contents: &str) -> Vec<Equation> {
    let mut result = Vec::new();
    for line in contents.lines() {
        let equation_regex = Regex::new(r"(?m)(\d+): (.*)").expect("Problem creating equation mutex");
        for capture in equation_regex.captures_iter(line) {
            let answer = capture.get(1).expect("Missing answer capture group").as_str().parse::<i64>().expect("Error parsing answer");
            let rest_str = capture.get(2).expect("Missing rest capture group").as_str();
            let splits = rest_str.split(" ");
            let mut terms: Vec<i64> = Vec::new();
            for term in splits {
                terms.push(term.parse::<i64>().expect("Error parsing term"));
            }
            result.push(Equation::new(answer, terms));
        }
    }
    result
}
