fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = file.lines().collect();

    // let res = solve_1(lines);
    // println!("Result: {}", res);

    let res = solve_2(lines);
    println!("Result: {}", res);
}

fn solve_1(lines: Vec<&str>) -> i32 {
    let mut sum: i32 = 0;

    for line in lines {
        let mut left: char = ' ';

        for c in line.chars() {
            if c.is_numeric() {
                left = c;
                break;
            }
        }

        for c in line.chars().rev() {
            if c.is_numeric() {
                let str = format!("{}{}", left, c);
                let num: i32 = str.parse().unwrap();
                sum += num;
                println!("line: {}", line);
                println!("{} + {} = {}", left, c, num);
                break;
            }
        }
    }

    sum
}

fn solve_2(lines: Vec<&str>) -> i32 {
    let mut sum: i32 = 0;
    let nums = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for line in lines {
        let mut left: char = ' ';
        let mut buf: String = String::new();

        for c in line.chars() {
            if c.is_ascii_alphabetic() {
                buf.push(c);

                let matches = find_matches(&buf, &nums);
                if !matches.is_empty() {
                    let x = matches.get(0).unwrap();

                    match *x {
                        "one" => left = '1',
                        "two" => left = '2',
                        "three" => left = '3',
                        "four" => left = '4',
                        "five" => left = '5',
                        "six" => left = '6',
                        "seven" => left = '7',
                        "eight" => left = '8',
                        "nine" => left = '9',
                        _ => (),
                    }

                    break;
                }

                continue;
            }

            if c.is_numeric() {
                left = c;
                break;
            }
        }

        let mut buf: String = String::new();

        for c in line.chars().rev() {
            if c.is_ascii_alphabetic() {
                buf.push(c);

                let inverted: String = buf.chars().rev().collect();
                let matches = find_matches(inverted.as_str(), &nums);
                if !matches.is_empty() {
                    let x = matches.get(0).unwrap();
                    let mut right = ' ';

                    match *x {
                        "one" => right = '1',
                        "two" => right = '2',
                        "three" => right = '3',
                        "four" => right = '4',
                        "five" => right = '5',
                        "six" => right = '6',
                        "seven" => right = '7',
                        "eight" => right = '8',
                        "nine" => right = '9',
                        _ => (),
                    }

                    let str = format!("{}{}", left, right);
                    let num: i32 = str.parse().unwrap();

                    sum += num;
                    break;
                }
            }

            if c.is_numeric() {
                let str = format!("{}{}", left, c);
                let num: i32 = str.parse().unwrap();
                sum += num;
                break;
            }
        }
    }

    sum
}

fn find_matches<'a>(string: &str, patterns: &'a [&str]) -> Vec<&'a str> {
    patterns
        .iter()
        .filter(|&&pattern| string.contains(pattern))
        .cloned()
        .collect()
}
