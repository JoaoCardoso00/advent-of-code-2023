fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = file.lines().collect();

    let res = solve_2(lines);
    println!("Result: {}", res);
}
// #[derive(Debug, Clone)]
// struct Set {
//     red: i32,
//     blue: i32,
//     green: i32,
// }

// #[derive(Debug)]
// struct Game {
//     id: usize,
//     is_valid: bool,
//     sets: Vec<Set>,
// }

// fn solve_1(lines: Vec<&str>) -> i32 {
//     const NUM_RED_CUBES: i32 = 12;
//     const NUM_BLUE_CUBES: i32 = 14;
//     const NUM_GREEN_CUBES: i32 = 13;

//     let mut games: Vec<Game> = Vec::new();

//     for (id, game) in lines.iter().enumerate() {
//         let strip_index = game.find(":").unwrap();
//         let game = &game[strip_index + 2..];

//         let sets = game.split(";").collect::<Vec<&str>>();
//         let sets = sets
//             .iter()
//             .map(|s| s.split_whitespace().collect::<Vec<&str>>())
//             .collect::<Vec<Vec<&str>>>();

//         let mut game = Game {
//             id: id + 1,
//             is_valid: true,
//             sets: Vec::new(),
//         };

//         for set in sets {
//             let mut res = Set {
//                 red: 0,
//                 blue: 0,
//                 green: 0,
//             };
//             let mut cursor = 0;

//             loop {
//                 let num = set[cursor];
//                 let mut color = set[cursor + 1];

//                 if color.chars().last().unwrap() == ',' {
//                     color = &color[..color.len() - 1];
//                 }

//                 match color {
//                     "red" => {
//                         if num.parse::<i32>().unwrap() > NUM_RED_CUBES {
//                             game.is_valid = false;
//                             break;
//                         }

//                         res.red += num.parse::<i32>().unwrap();
//                     }
//                     "blue" => {
//                         if num.parse::<i32>().unwrap() > NUM_BLUE_CUBES {
//                             game.is_valid = false;
//                             break;
//                         }

//                         res.blue += num.parse::<i32>().unwrap();
//                     }
//                     "green" => {
//                         if num.parse::<i32>().unwrap() > NUM_GREEN_CUBES {
//                             game.is_valid = false;
//                             break;
//                         }

//                         res.green += num.parse::<i32>().unwrap();
//                     }
//                     _ => panic!("Unknown color"),
//                 }

//                 game.sets.push(res.clone());

//                 cursor += 2;

//                 if cursor >= set.len() {
//                     break;
//                 }

//                 continue;
//             }
//         }

//         games.push(game);
//     }

//     let sum = games
//         .iter()
//         .filter(|g| g.is_valid)
//         .map(|g| g.id as i32)
//         .sum::<i32>();

//     sum
// }

fn solve_2(lines: Vec<&str>) -> i32 {
    let mut games: Vec<Game> = Vec::new();

    for (id, game) in lines.iter().enumerate() {
        let strip_index = game.find(":").unwrap();
        let game = &game[strip_index + 2..];

        let sets = game.split(";").collect::<Vec<&str>>();
        let sets = sets
            .iter()
            .map(|s| s.split_whitespace().collect::<Vec<&str>>())
            .collect::<Vec<Vec<&str>>>();

        let mut game = Game {
            id: id + 1,
            power: 0,
            is_valid: true,
            lowest_red: 0,
            lowest_blue: 0,
            lowest_green: 0,
        };

        for set in sets {
            let mut res = Set {
                red: 0,
                blue: 0,
                green: 0,
            };
            let mut cursor = 0;

            loop {
                let num = set[cursor];
                let mut color = set[cursor + 1];

                if color.chars().last().unwrap() == ',' {
                    color = &color[..color.len() - 1];
                }

                match color {
                    "red" => {
                        res.red += num.parse::<i32>().unwrap();
                    }
                    "blue" => {
                        res.blue += num.parse::<i32>().unwrap();
                    }
                    "green" => {
                        res.green += num.parse::<i32>().unwrap();
                    }
                    _ => panic!("Unknown color"),
                }

                if res.red > game.lowest_red {
                    game.lowest_red = res.red;
                }

                if res.blue > game.lowest_blue {
                    game.lowest_blue = res.blue;
                }

                if res.green > game.lowest_green && res.green != 0 {
                    game.lowest_green = res.green;
                }

                cursor += 2;

                if cursor >= set.len() {
                    break;
                }

                continue;
            }
        }

        game.power = game.lowest_red * game.lowest_blue * game.lowest_green;

        games.push(game);
    }

    println!("{:#?}", games);

    let sum = games
        .iter()
        .filter(|g| g.is_valid)
        .map(|g| g.power)
        .sum::<i32>();

    sum
}

#[derive(Debug, Clone)]
struct Set {
    red: i32,
    blue: i32,
    green: i32,
}

#[derive(Debug)]
struct Game {
    id: usize,
    is_valid: bool,
    power: i32,
    lowest_red: i32,
    lowest_blue: i32,
    lowest_green: i32,
}
