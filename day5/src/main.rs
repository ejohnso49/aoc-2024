use std::{collections::HashMap, fs};
use regex::Regex;

fn main() {
    let file_result = fs::read_to_string("input.txt");

    let contents = match file_result {
        Ok(contents) => contents,
        Err(error) => panic!("fuck hit an {error}"),
    };

    let rules = get_rules(&contents);
    let updates = get_page_updates(&contents);

    println!("rules: {:?}", rules);
    println!("updates: {:?}", updates);
    let result = determine_valid_updates(&updates, &rules);
    println!("Found {result}");
}

fn get_rules(contents: &str) -> HashMap<u32, Vec<u32>> {
    let mut rules = HashMap::new();
    let rules_regex = Regex::new(r"(?m)^(\d+)\|(\d+)$").expect("Error creating regex");
    for capture in rules_regex.captures_iter(contents) {
        let rule_page = capture.get(1).expect("Missing first number for rule").as_str().parse::<u32>().expect("Couldn't parse first number");
        let dependent_page = capture.get(2).expect("Missing second number for rule").as_str().parse::<u32>().expect("Couldn't parse second number");

        let rule = rules.entry(rule_page).or_insert(Vec::new());
        rule.push(dependent_page);
    }

    rules
}

fn get_page_updates(contents: &str) -> Vec<Vec<u32>> {
    let mut page_updates = Vec::new();
    let page_update_regex = Regex::new(r"(?m)^(\d+,?)+$").expect("Error creating page update regex");

    for capture in page_update_regex.captures_iter(contents) {
        let page_update: Vec<u32> = capture.get(0).expect("Could not get regex capture").as_str().split(",").map(|page_str| page_str.parse::<u32>().expect("Error parsing page number")).collect();
        page_updates.push(page_update);
    }

    page_updates
}

fn determine_valid_updates(updates: &Vec<Vec<u32>>, rules: &HashMap<u32, Vec<u32>>) -> u32 {
    let mut result: u32 = 0;
    
    for update in updates {
        let mut good_update = true;
        let mut page_positions: HashMap<u32, usize> = HashMap::new();
        for (index, page) in update.iter().enumerate() {
            page_positions.insert(*page, index);
        }

        println!("page_positions: {:?}", page_positions);
        'pages: for i in 0..update.len() {
            if !rules.contains_key(&update[i]) {
                continue;
            }

            let page_rule = rules.get(&update[i]).expect(format!("No rules for page {}", update[i]).as_str());
            for page in page_rule {
                if !page_positions.contains_key(page) {
                    continue;
                }
                let page_position = page_positions.get(page).expect(format!("No position found for {page}").as_str());
                if i > *page_position {
                    good_update = false;
                    break 'pages;
                }
            }
        }
        if good_update {
            let middle_index = update.len() / 2;
            result += update[middle_index];
        }
    }
    result
}