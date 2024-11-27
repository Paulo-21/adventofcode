use std::fs::read_to_string;
enum Game {
    Rock,
    Paper,
    Scissors
}

fn main() {
    let file = read_to_string("input").unwrap();
    let mut final_score = 0;
    for line in file.lines() {
        let mut s = line.split_whitespace();
        let op = s.next().unwrap();
        let me = s.next().unwrap();
        let op = match op {
            "A" => Game::Rock,
            "B" => Game::Paper,
            "C" | _ => Game::Scissors,
        };
        let me = match me {
            "X" => Game::Rock,
            "Y" => Game::Paper,
            "Z" | _ => Game::Scissors
        };
        let score = match op {
            Game::Rock => {
                match me {
                    Game::Rock => 3,
                    Game::Paper => 6,
                    Game::Scissors => 0,
                }
            },
            Game::Paper => {
                match me {
                    Game::Rock => 0,
                    Game::Paper => 3,
                    Game::Scissors => 6,
                }
            },
            Game::Scissors => {
                match me {
                    Game::Rock => 6,
                    Game::Paper => 0,
                    Game::Scissors => 3,
                }
            },
            _ => { 0 }
        };
        let movescore = match me {
            Game::Rock => 1,
            Game::Paper => 2,
            Game::Scissors  => 3,
        };
        final_score+= score+movescore;
    }
    println!("{}", final_score);
}
