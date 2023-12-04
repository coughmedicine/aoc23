use std::{
    collections::{HashMap, HashSet},
    fs,
};

pub fn day4_1() {
    let binding =
        fs::read_to_string("src/day4/input.txt").expect("Should have been able to read the file");
    let lines = binding.split("\n").filter(|x| *x != "");
    let mut sum = 0;
    for l in lines {
        let v: Vec<_> = l.split(" | ").collect();
        let winning = v[0];
        let winning_nums: HashSet<_> = winning
            .split(" ")
            .filter_map(|x| x.parse::<i32>().ok())
            .collect();
        let yours = v[1];
        let your_nums = yours.split(" ").filter_map(|x| x.parse::<i32>().ok());

        let mut times_it_appears: i32 = -1;
        for your in your_nums {
            if (winning_nums.contains(&your)) {
                times_it_appears += 1;
            }
        }
        if times_it_appears >= 0 {
            sum += 2_i32.pow(times_it_appears as u32);
        }
    }
    println!("Sum overall: {sum}");
}

pub fn day4_2() {
    let binding =
        fs::read_to_string("src/day4/input.txt").expect("Should have been able to read the file");
    let lines = binding.split("\n").filter(|x| *x != "");
    let mut cards = HashMap::new();

    for (i, l) in lines.enumerate() {
        cards.entry(i).or_insert(1);
        let v: Vec<_> = l.split(" | ").collect();
        let winning = v[0];
        let winning_nums: HashSet<_> = winning
            .split(" ")
            .filter_map(|x| x.parse::<i32>().ok())
            .collect();
        let yours = v[1];
        let your_nums = yours.split(" ").filter_map(|x| x.parse::<i32>().ok());

        let mut times_it_appears: i32 = 0;
        for your in your_nums {
            if winning_nums.contains(&your) {
                times_it_appears += 1;
            }
        }
        for j in 0..times_it_appears {
            let ij = i as usize + j as usize + 1;
            *cards.entry(ij).or_insert(1) += 1 * cards.get(&i).unwrap();
        }
    }
    println!("The dictionary: {cards:?}");
    println!("Sum overall: {}", cards.values().sum::<i32>());
}
