pub fn day1_1() {
    use std::fs;

    let binding =
        fs::read_to_string("src/day1/input.txt").expect("Should have been able to read the file");
    let lines = binding.split("\n").filter(|x| *x != "");
    println!("{:?}", lines.clone().collect::<Vec<_>>());

    let mut sum = 0;

    for l in lines {
        let mut first = 0;
        let mut last = 0;

        for c in l.chars() {
            if c.is_numeric() {
                first = c.to_digit(10).unwrap();
                break;
            }
        }
        for c in l.chars().rev() {
            if c.is_numeric() {
                last = c.to_digit(10).unwrap();
                break;
            }
        }

        sum += first * 10 + last;
    }

    println!("{}", sum);
}

pub fn day1_2() {
    use std::fs;

    let binding =
        fs::read_to_string("src/day1/input.txt").expect("Should have been able to read the file");
    let lines = binding.split("\n").filter(|x| *x != "");
    //println!("{:?}", lines.clone().collect::<Vec<_>>());

    let mut sum = 0;

    for l in lines {
        let mut first = (0, usize::MAX);
        let mut last = (0, -1isize);

        for (i, ch) in [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ]
        .iter()
        .enumerate()
        {
            let index = l.find(ch);
            if index != None && first.1 > index.unwrap() {
                first = (i + 1, index.unwrap());
            }
            let n = format!("{}", i + 1);

            let index = l.find(&n);
            if index != None && first.1 > index.unwrap() {
                first = (i + 1, index.unwrap());
            }

            let index = l.rfind(ch);
            if index != None && last.1 < index.unwrap() as isize {
                last = (i + 1, index.unwrap() as isize);
            }
            let n = format!("{}", i + 1);

            let index = l.rfind(&n);
            if index != None && last.1 < index.unwrap() as isize {
                last = (i + 1, index.unwrap() as isize);
            }
        }
        if first.0 == 0 || last.0 == 0 {
            panic!("l: {}, first: {:?}, last: {:?}", l, first, last)
        }
        sum += first.0 * 10 + last.0;
    }

    println!("{}", sum);
}
