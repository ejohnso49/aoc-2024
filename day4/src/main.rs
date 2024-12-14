use std::fs;

struct Location {
    value: char,
    visited: bool,
}

fn main() {
    let file_result = fs::read_to_string("input.txt");

    let contents = match file_result {
        Ok(contents) => contents,
        Err(error) => panic!("fuck hit an {error}"),
    };

    // Transform letters into a 2D array
    let mut array = convert_contents_to_array(&contents);

    // Search array for XMAS
    search_locations(&mut array, "XMAS");

}

fn convert_contents_to_array(contents: &str) -> Vec<Vec<Location>> {
    let mut array: Vec<Vec<Location>> = Vec::new();
    for line in contents.lines() {
        let line_vec: Vec<Location> = line.chars().map(|c| Location{value: c, visited: false}).collect(); 
        array.push(line_vec);
    }
    array
}

fn search_locations(array: &mut Vec<Vec<Location>>, letters: &str) {
    let x_size = array.len();
    let y_size = array[0].len();
    let mut result: u32 = 0;

    for x in 1..x_size - 1 {
        for y in 1..y_size - 1 {
            let location = &array[x][y];
            if location.visited || location.value != 'A' {
                continue;
            }

            // Search right
            println!("Found A at {x},{y}");
            // Order: TL,TR,BL,BR
            // Search M,M,S,S
            if array[x - 1][y - 1].value == 'M' && array[x - 1][y + 1].value == 'M' && array[x + 1][y - 1].value == 'S' && array[x + 1][y + 1].value == 'S' {
                println!("XMAS found [M,M,S,S] at {x},{y}");
                result += 1;
            }
            // Search M,S,M,S
            if array[x - 1][y - 1].value == 'M' && array[x - 1][y + 1].value == 'S' && array[x + 1][y - 1].value == 'M' && array[x + 1][y + 1].value == 'S' {
                println!("XMAS found [M,S,M,S] at {x},{y}");
                result += 1;
            }
            // Search S,S,M,M
            if array[x - 1][y - 1].value == 'S' && array[x - 1][y + 1].value == 'S' && array[x + 1][y - 1].value == 'M' && array[x + 1][y + 1].value == 'M' {
                println!("XMAS found [S,S,M,M] at {x},{y}");
                result += 1;
            }
            // Search S,M,S,M
            if array[x - 1][y - 1].value == 'S' && array[x - 1][y + 1].value == 'M' && array[x + 1][y - 1].value == 'S' && array[x + 1][y + 1].value == 'M' {
                println!("XMAS found [S,M,S,M] at {x},{y}");
                result += 1;
            }


            array[x][y].visited = true;
        }
    }
    println!("Found {result} XMASs");
}