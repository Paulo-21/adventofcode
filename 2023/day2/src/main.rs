use std::fs;
fn day1() {
    let file = fs::read_to_string("input").unwrap();
    let games = file.lines();    
//Game 1: 4 green, 2 blue; 1 red, 1 blue, 4 green; 3 green, 4 blue, 1 red; 7 green, 2 blue, 4 red; 3 red, 7 green; 3 red, 3 green
    let mut res = 0;
    for l in games {
        let mut true_game = true;
        let mut a = l.split(':');
        let gameid = a.next().unwrap().split(' ').nth(1).unwrap().parse::<i32>().unwrap();
        //print!("{gameid} ");
        let b = a.next().unwrap();
        for c in b.split(';') {
            for d in c.split(',') {
                let mut r = d.split(' ');
                let _ = r.next();
                let number = r.next().unwrap().parse::<i32>().unwrap();
                //print!("{number} ");
                let color = r.next().unwrap();
                match color {
                    "green" => {
                        if number > 13 {
                            true_game = false;
                        }
                    },
                    "blue" => {
                        if number > 14 {
                            true_game = false;
                        }
                    },
                    "red" => {
                        if number > 12 {
                            true_game = false;
                        }
                    },
                    _=> {}
                }
            }
        }
        if true_game {
            res += gameid;
        }
        println!();
    }
    println!("{res}");
}
fn day2() {
    let file = fs::read_to_string("input").unwrap();
    let games = file.lines();    
//Game 1: 4 green, 2 blue; 1 red, 1 blue, 4 green; 3 green, 4 blue, 1 red; 7 green, 2 blue, 4 red; 3 red, 7 green; 3 red, 3 green
    let mut res = 0;
    for l in games {
        let mut maxR= 0;
        let mut maxG= 0;
        let mut maxB= 0;
        let mut true_game = true;
        let mut a = l.split(':');
        let gameid = a.next().unwrap().split(' ').nth(1).unwrap().parse::<i32>().unwrap();
        //print!("{gameid} ");
        let b = a.next().unwrap();
        for c in b.split(';') {
            for d in c.split(',') {
                let mut r = d.split(' ');
                let _ = r.next();
                let number = r.next().unwrap().parse::<i32>().unwrap();
                //print!("{number} ");
                let color = r.next().unwrap();
                match color {
                    "green" => {
                        if number > maxG {
                            maxG = number;
                        }
                    },
                    "blue" => {
                        if number > maxB {
                            maxB = number;
                        }
                    },
                    "red" => {
                        if number > maxR {
                            maxR = number;
                        }
                    },
                    _=> {}
                }
            }
        }
        if true_game {
            res += maxB*maxG*maxR;
        }
        //println!();
    }
    println!("{res}");
}
fn main() {
    //day1();
    day2();
}
