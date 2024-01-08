use std::fs;

fn p1() {
    let file = fs::read_to_string("input").unwrap();
    let mut values : Vec<usize> = file.split(',').map(|str|str.parse::<usize>().unwrap()).collect();
    
    values[1] = 12;
    values[2] = 2;
    let mut i = 0;
    while i < values.len() {
        let opcode = values[i];
        if opcode == 99 {
            break;
        }
        let input1 = values[i+1];
        let input2 = values[i+2];
        let output = values[i+3];
        let out = if opcode == 1{
            values[input1] + values[input2]
        }
        else {
            values[input1] * values[input2]
        };
        values[output] = out;
        i+=4;
    }

    println!("{}", values[0]);
}
fn p2() {
    let file = fs::read_to_string("input").unwrap();
    let mut values_saves1 : Vec<usize> = file.split(',').map(|str|str.parse::<usize>().unwrap()).collect();
    let values_saves = values_saves1.as_mut_slice();
    let mut nom = 12;
    let mut verb = 2;
    let values : &mut [usize] = &mut vec![0;values_saves.len()] ;
    loop {
        values.copy_from_slice(values_saves);
        values[1] = nom;
        values[2] = verb;
        let mut i: usize = 0;
        while i < values.len() {
            let opcode = values[i];
            if opcode == 99 {
                break;
            }
            let input1 = values[i+1];
            let input2 = values[i+2];
            let output = values[i+3];
            let out = if opcode == 1{
                values[input1] + values[input2]
            }
            else {
                values[input1] * values[input2]
            };
            values[output] = out;
            i+=4;
        }
        if values[0] == 19690720 {
            break;
        }
        if nom >= 99 {
            nom = 0;
            verb+=1;
            continue;
        }
        nom+=1;
    }
    println!("{}", (100*nom)+verb);
}

fn main() {
    p1();
    p2();
}
