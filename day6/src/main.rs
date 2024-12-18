use core::fmt;
use std::fs;

#[derive(Debug, Clone)]
enum GuardDirection {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug, Clone)]
struct Guard {
    start: (isize, isize),
    current: (isize, isize),
    direction: GuardDirection,
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
    fn try_step(&self) -> (isize, isize) {
        let (delta_x, delta_y) = self.get_step_deltas();
        (self.current.0 + delta_x, self.current.1 + delta_y)
    }

    fn step(&mut self) {
        let (delta_x, delta_y) = self.get_step_deltas();
        self.current = (self.current.0 + delta_x, self.current.1 + delta_y);
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

#[derive(Debug, Clone)]
struct Location {
    loc_type: LocationType,
    visited: bool,
}

impl Location {
    pub fn new(loc_type: LocationType) -> Self {
        Self {
            loc_type: loc_type,
            visited: false
        }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.loc_type {
            LocationType::Space => write!(f, "."),
            LocationType::Obstacle => write!(f, "#"),
        }
    }
}

#[derive(Debug, Clone)]
struct Lab {
    guard: Guard,
    map: Vec<Vec<Location>>
}

impl fmt::Display for Lab {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (y, line) in self.map.iter().enumerate() {
            for (x, location) in line.iter().enumerate() {
                if self.guard.current == (x.try_into().unwrap(), y.try_into().unwrap()) {
                    write!(f, "{}", self.guard)?;
                }
                write!(f, "{location}")?;
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
    println!("Guard moved {} steps", count_guard_steps(&lab));
}

fn create_lab(contents: &str) -> Lab {
    let mut map: Vec<Vec<Location>> = Vec::new();
    let mut guard = Guard{start: (0,0), current: (0,0), direction: GuardDirection::Down};
    // Iterate through each line
    for (y, line) in contents.lines().enumerate() {
        let mut line_vec: Vec<Location> = Vec::new();
        for (x, line_char) in line.chars().enumerate() {
            match line_char {
                '.' => line_vec.push(Location::new(LocationType::Space)),
                '#' => line_vec.push(Location::new(LocationType::Obstacle)),
                '^' => {
                    line_vec.push(Location::new(LocationType::Space));
                    guard = Guard{start: (x.try_into().unwrap(), y.try_into().unwrap()), current: (x.try_into().unwrap(), y.try_into().unwrap()), direction: GuardDirection::Up};
                },
                _ => panic!("Unexpected char found: {line_char}")
            }
        }
        map.push(line_vec);
    }
    Lab{map: map, guard: guard}
}

fn count_guard_steps(lab: &Lab) -> u32 {
    let mut map = lab.map.clone();
    let mut guard = lab.guard.clone();
    let x_max = map[0].len();
    let y_max = map.len();
    while lab.guard.current.0 < x_max.try_into().unwrap() && guard.current.1 < y_max.try_into().unwrap() && guard.current.0 >= 0 && guard.current.1 >= 0 {
        println!("{lab}");
        let x: usize = guard.current.0.try_into().unwrap();
        let y: usize = guard.current.1.try_into().unwrap();
        let location = &mut map[y][x];
        if let LocationType::Space = location.loc_type {
            if !location.visited {
                location.visited = true;
            }
        }

        let (next_x, next_y) = guard.try_step();
        if next_x < x_max.try_into().unwrap() && next_y < y_max.try_into().unwrap() && next_x >= 0 && next_y >= 0 {
            let next_x: usize = next_x.try_into().unwrap();
            let next_y: usize = next_y.try_into().unwrap();

            let next_location = &map[next_y][next_x];
            match next_location.loc_type {
                LocationType::Space => guard.step(),
                LocationType::Obstacle => guard.rotate(),
            }
        } else {
            break;
        }
    }

    let mut count: u32 = 0;
    for x in 0..x_max {
        for y in 0..y_max {
            if map[y][x].visited {
                count += 1;
            }
        }
    }
    count
}
