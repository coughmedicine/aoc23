use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn find_surr_nums((x, y): (usize, usize), g: &Grid) -> HashSet<(usize, usize)> {
    const DIRS: [Dir; 8] = [
        Dir::N,
        Dir::E,
        Dir::S,
        Dir::W,
        Dir::NE,
        Dir::NW,
        Dir::SE,
        Dir::SW,
    ];

    // set of all coords containing a number around a certain point
    let mut surr_nums = HashSet::new();
    for d in DIRS {
        let num = Grid::get_point_dir((x, y), d);
        if let Some(ch) = g.get_pos(num) {
            if ch.is_numeric() {
                surr_nums.insert(num);
            }
        }
    }
    surr_nums
}

fn number_from_digit_coord(g: &Grid, known_coords: HashSet<(usize, usize)>) -> Vec<i32> {
    let mut visited = HashSet::new();
    let mut res = Vec::new();
    for coord in known_coords {
        if !visited.contains(&coord) {
            let x = coord.0;
            let y = coord.1;
            let mut num_str = "".to_owned();

            for i in x..g.size.0 {
                let ch = g.get_pos((i, y));
                if let Some(ch) = ch {
                    if ch.is_numeric() {
                        visited.insert((i, y));
                        num_str.push(ch);
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            for i in (0..x).rev() {
                let ch = g.get_pos((i, y));
                if let Some(ch) = ch {
                    if ch.is_numeric() {
                        visited.insert((i, y));
                        num_str.insert(0, ch);
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            res.push(num_str.parse().unwrap());
        }
    }
    res
}

pub fn day3_1() {
    let binding =
        fs::read_to_string("src/day3/input.txt").expect("Should have been able to read the file");
    let lines = binding.split("\n").filter(|x| *x != "");

    let mut s = 0;

    let g = Grid::new_from_string(lines);
    for (key, value) in &g.grid_data {
        if !(value.is_numeric() || *value == '.') {
            let surr_nums = find_surr_nums(*key, &g);
            let nums = number_from_digit_coord(&g, surr_nums);
            s += nums.iter().sum::<i32>();
        }
    }

    println!("{}", s);
}
#[derive(Debug)]
struct Grid {
    size: (usize, usize),
    grid_data: HashMap<(usize, usize), char>,
}

impl Grid {
    fn new_from_string<'a>(lines: impl Iterator<Item = &'a str>) -> Self {
        let mut g = Grid {
            size: (0, 0),
            grid_data: HashMap::new(),
        };
        for (y, line) in lines.enumerate() {
            g.size.0 = 0;
            for (x, cha) in line.chars().enumerate() {
                g.size.0 += 1;
                g.grid_data.insert((x, y), cha);
            }
            g.size.1 += 1;
        }
        g
    }

    fn get_pos(&self, coords: (usize, usize)) -> Option<char> {
        self.grid_data.get(&coords).copied()
    }

    fn get_point_dir((x, y): (usize, usize), dir: Dir) -> (usize, usize) {
        match dir {
            Dir::N => (x, y - 1),
            Dir::E => (x + 1, y),
            Dir::S => (x, y + 1),
            Dir::W => (x - 1, y),
            Dir::NE => (x + 1, y - 1),
            Dir::NW => (x - 1, y - 1),
            Dir::SE => (x + 1, y + 1),
            Dir::SW => (x - 1, y + 1),
        }
    }
}

enum Dir {
    N,
    E,
    S,
    W,
    NE,
    NW,
    SE,
    SW,
}
