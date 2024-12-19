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
    LoopDetected(u32)
}

#[derive(Debug, Clone)]
struct Guard {
    start: (isize, isize),
    current: (isize, isize),
    direction: GuardDirection,
    steps: HashMap<(isize, isize), u32>,
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
    fn new(start: (isize, isize)) -> Self {
        Guard{
            start: start,
            current: start,
            direction: GuardDirection::Up,
            steps: HashMap::new()
        }
    }

    fn get_next_step(&self) -> (isize, isize) {
        let (delta_x, delta_y) = self.get_step_deltas();
        (self.current.0 + delta_x, self.current.1 + delta_y)
    }

    fn step(&mut self) {
        let (delta_x, delta_y) = self.get_step_deltas();
        self.current = (self.current.0 + delta_x, self.current.1 + delta_y);
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
    while lab.guard.current.0 < x_max.try_into().unwrap() && guard.current.1 < y_max.try_into().unwrap() && guard.current.0 >= 0 && guard.current.1 >= 0 {
        // println!("{lab}");
        let x: usize = guard.current.0.try_into().unwrap();
        let y: usize = guard.current.1.try_into().unwrap();
        match map[y][x] {
            LocationType::Space => guard.increment_step_count()?,
            LocationType::Obstacle => panic!("Guard occupies obstacle at {x}, {y}"),
        }

        let (next_x, next_y) = guard.get_next_step();
        if next_x >= 0 && next_y >= 0 && next_x < x_max.try_into().unwrap() && next_y < y_max.try_into().unwrap() {
            let next_x: usize = next_x.try_into().unwrap();
            let next_y: usize = next_y.try_into().unwrap();

            let next_location = &map[next_y][next_x];
            match next_location {
                LocationType::Space => guard.step(),
                LocationType::Obstacle => guard.rotate(),
            }
        } else {
            break;
        }
    }

    Ok(guard.steps.keys().len().try_into().unwrap())
}
