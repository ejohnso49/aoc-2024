use core::fmt;
use std::{collections::HashMap, fs};

#[derive(Debug, Clone)]
enum GuardDirection {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug)]
enum GuardError {
    Exit,
    LoopDetected(u32)
}

#[derive(Debug, Clone)]
struct Guard {
    start: (usize, usize),
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
            start: start,
            current: start,
            direction: GuardDirection::Up,
            steps: HashMap::new()
        }
    }

    fn step(&mut self, new_location: (usize, usize)) {
        self.current = new_location;
    }

    fn increment_step_count(&mut self) -> Result<(), GuardError> {
        let count_entry = self.steps.entry((self.current.0, self.current.1)).or_default();
        if *count_entry > 2 {
            Err(GuardError::LoopDetected(*count_entry))
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

    let contents = match file_result {
        Ok(contents) => contents,
        Err(error) => panic!("fuck hit an {error}"),
    };

    let lab = create_lab(&contents);
    println!("{lab}");
    println!("Guard moved {:?} steps", count_guard_steps(&lab));
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

fn count_guard_steps(lab: &Lab) -> Result<u32, GuardError> {
    let map = lab.map.clone();
    let mut guard = lab.guard.clone();
    let x_max = map[0].len();
    let y_max = map.len();
    while lab.guard.current.0 < x_max && guard.current.1 < y_max {
        // println!("{lab}");
        let x: usize = guard.current.0.try_into().unwrap();
        let y: usize = guard.current.1.try_into().unwrap();
        match map[y][x] {
            LocationType::Space => guard.increment_step_count()?,
            LocationType::Obstacle => panic!("Guard occupies obstacle at {x}, {y}"),
        }

        // Get the next steps for the guard
        let (delta_x, delta_y) = guard.get_step_deltas();

        // Calculate the new x and y values. If we've moved out of the boundaries, calculate the unique steps
        let new_x = match guard.current.0.checked_add_signed(delta_x) {
            Some(x) => x,
            None => return Ok(guard.unique_steps()),
        };
        let new_y = match guard.current.1.checked_add_signed(delta_y) {
            Some(y) => y,
            None => return Ok(guard.unique_steps()),
        };

        // Check that we're within array bounds, if not return current unique steps
        if new_x >= x_max || new_y >= y_max {
            return Ok(guard.unique_steps());
        }

        let next_location = &map[new_y][new_x];
        match next_location {
            LocationType::Space => guard.step((new_x, new_y)),
            LocationType::Obstacle => guard.rotate(),
        }
    }
    panic!("We exited the while loop through an unexpected path");
}
