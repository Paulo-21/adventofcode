use std::{fs, time::Instant};
fn main() {
    let pathname = "input.txt";
    //let pathname = "exemple.txt";
    //let pathname = "exemple2.txt";
    let input = fs::read_to_string(pathname).unwrap();
    let input = input.trim();
    let input = input.split(',');
    let mut res = 0;
    let start = Instant::now();
    for line in input {
        //println!("{}", line);

        let mut s = line.split('-');

        let lower: u64 = s.next().unwrap().parse().unwrap();
        let upper: u64 = s.next().unwrap().parse().unwrap();
        let nb_digit_total = lower.ilog10() + 1;

        let (mut current, mut current_half) = match nb_digit_total % 2 {
            0 => {
                let ch = lower / 10u64.pow((nb_digit_total) / 2);
                let c = ch * 10u64.pow((nb_digit_total) / 2) + ch;
                (c, ch)
            }
            1 => {
                let ch = 10u64.pow((nb_digit_total) / 2);
                let c = ch * 10u64.pow((nb_digit_total) / 2 + 1) + ch;
                (c, ch)
            }
            _ => {
                panic!("Impossible");
            }
        };
        if current > upper {
            continue;
        } else if current <= upper && current >= lower {
            //res += current;
            //print!("{}, half : {} and ", current, current_half);
        }
        let mut iteration = 0;
        // Preprocessing

        while current <= upper && iteration < 1000 {
            let nb_digit_total = current.ilog10() + 1;
            if nb_digit_total % 2 == 1 {
                // Impair
                current = 10u64.pow(nb_digit_total);
                //println!("CUUUR {} ", current);
                continue;
            }
            if current >= lower {
                res += current;
            }
            //println!("NB Digit {}", nb_digit_total);
            //println!("TESTING : {current} , half : {current_half}");
            let mid = nb_digit_total / 2;
            let rr = (current / 10u64.pow(mid)) + 1;
            let new = rr * 10u64.pow(mid) + rr;
            if lower <= new && new <= upper {
                //print!("{new}, half : {rr} and ");
                /*print!(
                    "{}, half : {} , upper : {upper} and ",
                    new,
                    rr
                );*/

                //res += new;
            }
            current = new;
            iteration += 1;
        }
        //println!();
        //println!();
    }
    println!("elasped : {} us", start.elapsed().as_micros());
    println!("{}", res);
    if pathname == "exemple.txt" || pathname == "exemple2.txt" {
        if res == 1227775554 {
            println!("Exemple GOOD ! ");
        } else {
            println!("Your : {res}");
            println!("True : 1227775554");
        }
    }
}
