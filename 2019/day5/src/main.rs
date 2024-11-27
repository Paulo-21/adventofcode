use std::fs;

fn p1() {
    let file = fs::read_to_string("input").unwrap();
    let mut values : Vec<i32> = file.split(',').map(|str|str.parse::<i32>().unwrap()).collect();
    
    let mut i = 0;
    while i < values.len() {
        let opcode = values[i];
        if opcode == 99 {
            break;
        }
        let input1 = values[i+1];
        let input2 = values[i+2];
        let output = values[i+3];
        match opcode {
            1 => {},
            2 => {},
            _=> {}
        }
        i+=4;
    }

    println!("{}", values[0]);
}

fn main() {
    p1();
}
