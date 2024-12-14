use std::fs;

struct Report {
    levels: Vec<u32>,
}

impl Report {
    fn is_monotonic(&self, levels: Option<&Vec<u32>>) -> bool {
        let levels = match levels {
            Some(levels) => levels,
            None => &self.levels,
        };
        for index in 1..levels.len() - 1 {
            let left = levels[index - 1];
            let center = levels[index];
            let right = levels[index + 1];

            let lc = if center >= left { 1 } else { 0 };
            let cr = if right >= center { 1 } else { 0 };

            if lc != cr {
                return false
            }
        }
        true
    }

    fn is_level_distance_safe(&self, levels: Option<&Vec<u32>>) -> bool {
        let levels = match levels {
            Some(levels) => levels,
            None => &self.levels,
        };

        for index in 0..levels.len() - 1 {
            let curr_level = levels[index];
            let next_level = levels[index + 1];

            let diff = curr_level.abs_diff(next_level);
            if diff < 1 || diff > 3 {
                return false
            }
        }

        true
    }

    pub fn is_safe(&self) -> bool {
        if self.is_monotonic(None) && self.is_level_distance_safe(None) {
            return true
        } else {
            for i in 0..self.levels.len() {
                let mut new_levels = self.levels.clone();
                new_levels.remove(i);
                if self.is_monotonic(Some(&new_levels)) && self.is_level_distance_safe(Some(&new_levels)) {
                    println!("Report safe after removing {i}, levels: {new_levels:?}");
                    return true
                }
            }
            false
        }
    }
}

fn main() {

    let file_result = fs::read_to_string("input.txt");

    let contents = match file_result {
        Ok(contents) => contents,
        Err(error) => panic!("fuck hit an {error}"),
    };

    let reports = get_reports_from_contents(&contents);
    println!("Found {} reports", reports.len());
    let (safe_count, unsafe_count) = check_reports(&reports);
    println!("Found {safe_count} safe reports, {unsafe_count} unsafe reports");
}

fn get_reports_from_contents(contents: &str) -> Vec<Report> {
    let mut reports: Vec<Report> = Vec::new();
    for line in contents.lines() {
        let levels: Vec<u32> = line.split(" ").map(|level| level.parse::<u32>().expect("Fuck bad number string")).collect();
        reports.push(Report{levels: levels});
    }
    // reports.push(Report{levels: vec![66, 69, 70, 73, 76, 79, 77, 83]});
    reports
}

fn check_reports(reports: &Vec<Report>) -> (u32, u32) {
    let mut unsafe_count: u32 = 0;
    let mut safe_count: u32 = 0;

    for report in reports {
        match report.is_safe() {
            true => {
                println!("Report safe: {:?}", report.levels);
                safe_count += 1;
            }
            false => unsafe_count += 1,
        }
    }

    (safe_count, unsafe_count)
}
