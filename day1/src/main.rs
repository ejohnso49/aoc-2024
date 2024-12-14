use std::{collections::HashMap, fs};

fn main() {
    println!("Hello, world!");

    let file_result = fs::read_to_string("input.txt");

    let contents = match file_result {
        Ok(contents) => contents,
        Err(error) => panic!("fuck hit an {error}"),
    };

    println!("{contents}");
    let (mut list_0, mut list_1) = split_into_lists(&contents);
    println!("List lengths {}, {}", list_0.len(), list_1.len());

    list_0.sort();
    list_1.sort();

    let distance = calculate_list_distances(&list_0, &list_1);
    println!("List distance = {distance}");

    let similarity = calculate_similarity(&list_0, &list_1);
    println!("List similarity = {similarity}");
}

fn split_into_lists(input_text: &str) -> (Vec<u32>, Vec<u32>) {
    let mut list_0: Vec<u32> = Vec::new();
    let mut list_1: Vec<u32> = Vec::new();
    for line in input_text.lines() {
        let list_entries: Vec<&str> = line.split("   ").collect();
        list_0.push(list_entries[0].parse::<u32>().expect("What the fuck this wasn't a number string?"));
        list_1.push(list_entries[1].parse::<u32>().expect("What the fuck this wasn't a number string either?"));
    }

    (list_0, list_1)
}

fn calculate_list_distances(list_0: &Vec<u32>, list_1: &Vec<u32>) -> u32 {
    let mut result: u32 = 0;
    for (entry_0, entry_1) in list_0.iter().zip(list_1.iter()) {
        result += entry_0.abs_diff(*entry_1);
    }
    result
}

fn calculate_similarity(list_0: &Vec<u32>, list_1: &Vec<u32>) -> u32 {
    let mut result: u32 = 0;

    // Make frequency map of list 1
    let mut map_1: HashMap<u32, u32> = HashMap::new();

    for entry_1 in list_1 {
        map_1.entry(*entry_1).and_modify(|counter| *counter += 1).or_insert(1);
    }

    for location_id in list_0 {
        let count_1 = map_1.entry(*location_id).or_default();
        result += *count_1 * *location_id;
    }
    result
}
