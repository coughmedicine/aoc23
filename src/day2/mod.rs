use std::fs;

pub fn day2_1() {
    use regex::Regex;

    const RED: i32 = 12;
    const GREEN: i32 = 13;
    const BLUE: i32 = 14;

    let binding =
        fs::read_to_string("src/day2/input.txt").expect("Should have been able to read the file");
    let lines = binding.split("\n").filter(|x| *x != "");

    let re = Regex::new(r"(?<number>\d+) (?<colour>(red|green|blue))").unwrap();

    let mut sum = 0;

    for (i, l) in lines.enumerate() {
        println!("new game: {}", i);
        let l = l.split(":").nth(1).unwrap();
        let sections: Vec<_> = l.split(";").collect();
        let mut is_possible = true;
        for subsecs in sections {
            println!("new round: ");
            let caps = re.captures_iter(subsecs);
            for c in caps {
                let col = &c["colour"];
                let num = &c["number"];

                let max = match col {
                    "red" => RED,
                    "blue" => BLUE,
                    "green" => GREEN,
                    _ => unreachable!(),
                };

                if num.parse::<i32>().unwrap() > max {
                    is_possible = false;
                }
                println!("Colour: {:?}, number: {:?}", &c["colour"], &c["number"]);
            }
        }

        if is_possible {
            sum += i + 1;
        }
    }
    println!("{}", sum);
}

pub fn day2_2() {
    use regex::Regex;

    let binding =
        fs::read_to_string("src/day2/input.txt").expect("Should have been able to read the file");
    let lines = binding.split("\n").filter(|x| *x != "");

    let re = Regex::new(r"(?<number>\d+) (?<colour>(red|green|blue))").unwrap();

    let mut sum = 0;

    for (i, l) in lines.enumerate() {
        let mut max_red: i32 = 0;
        let mut max_green: i32 = 0;
        let mut max_blue: i32 = 0;

        println!("new game: {}", i);
        let l = l.split(":").nth(1).unwrap();
        let sections: Vec<_> = l.split(";").collect();
        for subsecs in sections {
            println!("new round: ");
            let caps = re.captures_iter(subsecs);
            for c in caps {
                let col = &c["colour"];
                let num = &c["number"];

                match col {
                    "red" => {
                        if num.parse::<i32>().unwrap() > max_red {
                            max_red = num.parse::<i32>().unwrap();
                        }
                    }
                    "blue" => {
                        if num.parse::<i32>().unwrap() > max_blue {
                            max_blue = num.parse::<i32>().unwrap();
                        }
                    }
                    "green" => {
                        if num.parse::<i32>().unwrap() > max_green {
                            max_green = num.parse::<i32>().unwrap();
                        }
                    }
                    _ => unreachable!(),
                }

                println!("Colour: {:?}, number: {:?}", &c["colour"], &c["number"]);
            }
        }

        sum += max_red * max_green * max_blue;

        println!("Red: {}, Green: {}, Blue: {}", max_red, max_green, max_blue);
    }

    println!("{}", sum);
}
