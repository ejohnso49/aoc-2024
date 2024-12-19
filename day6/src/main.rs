use core::fmt;
use std::{collections::HashMap, fs};

#[derive(Debug, Clone)]
enum GuardDirection {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug, Clone)]
struct Guard {
    current: (usize, usize),
    direction: GuardDirection,
    steps: HashMap<(usize, usize), u32>,
}

impl fmt::Display for Guard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.direction {
            GuardDirection::Up => write!(f, "↑"),
            GuardDirection::Down => write!(f, "↓"),
            GuardDirection::Left => write!(f, "←"),
            GuardDirection::Right => write!(f, "→"),
            
        }
    }
}

impl Guard {
    fn new(start: (usize, usize)) -> Self {
        Guard{
            current: start,
            direction: GuardDirection::Up,
            steps: HashMap::new()
        }
    }

    fn step(&mut self, new_location: (usize, usize)) {
        self.current = new_location;
    }

    fn increment_step_count(&mut self) -> Result<(), u32> {
        let count_entry = self.steps.entry((self.current.0, self.current.1)).or_default();
        *count_entry += 1;
        if *count_entry > 4 {
            Err(*count_entry)
        } else {
            Ok(())
        }

    }

    fn rotate(&mut self) {
        self.direction = match self.direction {
            GuardDirection::Up => GuardDirection::Right,
            GuardDirection::Down => GuardDirection::Left,
            GuardDirection::Left => GuardDirection::Up,
            GuardDirection::Right => GuardDirection::Down,
        }
    }

    fn get_step_deltas(&self) -> (isize, isize) {
        match self.direction {
            GuardDirection::Up => (0, -1),
            GuardDirection::Down => (0, 1),
            GuardDirection::Left => (-1, 0),
            GuardDirection::Right => (1, 0),
        }
    }

    fn unique_steps(&self) -> u32 {
        self.steps.keys().len().try_into().unwrap()
    }
}

#[derive(Debug, Clone)]
enum LocationType {
    Space,
    Obstacle,
}

impl fmt::Display for LocationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LocationType::Space => write!(f, "."),
            LocationType::Obstacle => write!(f, "#"),
        }
    }
}

#[derive(Debug, Clone)]
struct Lab {
    guard: Guard,
    map: Vec<Vec<LocationType>>
}

impl fmt::Display for Lab {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (y, line) in self.map.iter().enumerate() {
            for (x, location) in line.iter().enumerate() {
                if self.guard.current == (x.try_into().unwrap(), y.try_into().unwrap()) {
                    write!(f, "{}", self.guard)?;
                } else {
                    write!(f, "{location}")?;
                }
            }
            write!(f, "\n")?;
        }
        fmt::Result::Ok(())
    }
}

fn main() {
    let file_result = fs::read_to_string("input.txt");
    let mut obstruction_locations: u32 = 0;

    let contents = match file_result {
        Ok(contents) => contents,
        Err(error) => panic!("fuck hit an {error}"),
    };

    let lab = create_lab(&contents);
    let guard = run_guard_path(&lab).unwrap();
    println!("{lab}");
    println!("Guard moved {:?} steps", guard.unique_steps());

    let original_steps: Vec<&(usize, usize)> = guard.steps.keys().collect();
    for location in original_steps {
        if location.0 == lab.guard.current.0 && location.1 == lab.guard.current.1 {
            continue;
        }

        let mut new_lab = lab.clone();
        new_lab.map[location.1][location.0] = LocationType::Obstacle;
        // println!("Placing obstacle at {}, {}", location.0, location.1);
        match run_guard_path(&new_lab) {
            Err(_) => obstruction_locations += 1,
            Ok(_) => (),
        }
    }

    println!("Found {obstruction_locations} obstruction locations");
}

fn create_lab(contents: &str) -> Lab {
    let mut map: Vec<Vec<LocationType>> = Vec::new();
    let mut guard = Guard::new((0,0));
    // Iterate through each line
    for (y, line) in contents.lines().enumerate() {
        let mut line_vec: Vec<LocationType> = Vec::new();
        for (x, line_char) in line.chars().enumerate() {
            match line_char {
                '.' => line_vec.push(LocationType::Space),
                '#' => line_vec.push(LocationType::Obstacle),
                '^' => {
                    line_vec.push(LocationType::Space);
                    guard = Guard::new((x.try_into().unwrap(), y.try_into().unwrap()));
                },
                _ => panic!("Unexpected char found: {line_char}")
            }
        }
        map.push(line_vec);
    }
    Lab{map: map, guard: guard}
}

fn run_guard_path(lab: &Lab) -> Result<Guard, u32> {
    let map = lab.map.clone();
    let mut guard = lab.guard.clone();
    let x_max = map[0].len();
    let y_max = map.len();
    while lab.guard.current.0 < x_max && guard.current.1 < y_max {
        // println!("{lab}");
        let x: usize = guard.current.0;
        let y: usize = guard.current.1;
        match map[y][x] {
            LocationType::Space => guard.increment_step_count()?,
            LocationType::Obstacle => panic!("Guard occupies obstacle at {x}, {y}"),
        }

        // Get the next steps for the guard
        let (delta_x, delta_y) = guard.get_step_deltas();

        // Calculate the new x and y values. If we've moved out of the boundaries, calculate the unique steps
        let new_x = match guard.current.0.checked_add_signed(delta_x) {
            Some(x) => x,
            None => return Ok(guard),
        };
        let new_y = match guard.current.1.checked_add_signed(delta_y) {
            Some(y) => y,
            None => return Ok(guard),
        };

        // Check that we're within array bounds, if not return current unique steps
        if new_x >= x_max || new_y >= y_max {
            return Ok(guard);
        }

        let next_location = &map[new_y][new_x];
        match next_location {
            LocationType::Space => guard.step((new_x, new_y)),
            LocationType::Obstacle => guard.rotate(),
        }
    }
    panic!("We exited the while loop through an unexpected path");
}
