use std::{fs, cmp::Ordering, collections::BTreeMap};

const CARD : [char;13]= ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];
const CARD_P2 : [char;13]= ['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'];

fn p1() {
    let file = fs::read_to_string("input").unwrap();
    //let file = fs::read_to_string("example").unwrap();
    let mut sorted_hand : Vec<&str> = Vec::new();
    let mut sorted_bit : Vec<u32> = Vec::new();
    for line in file.lines() {
        let mut split = line.split_ascii_whitespace();
        let hand = split.next().unwrap();
        let bit : u32 = split.next().unwrap().parse().unwrap();
        let mut i = 0;
        let len = sorted_hand.len();
        while i < len {
            match compare(hand, sorted_hand[i]) {
                Ordering::Less => {
                    sorted_hand.insert(i, hand);
                    sorted_bit.insert(i, bit);
                    break;
                },
                _ => {}
            }
            i+=1;
        }
        if i >= len {
            sorted_hand.push(hand);
            sorted_bit.push(bit);
        }
    }
    println!("{:?}", sorted_hand);
    println!("{:?}", sorted_bit);
    let mut res = 0;
    for (i, bit) in sorted_bit.iter().enumerate() {
        res += (*bit)*(i+1) as u32;
    }
    println!("{}", res);
}
fn p2() {
    let file = fs::read_to_string("input").unwrap();
    //let file = fs::read_to_string("example").unwrap();
    let mut sorted_hand : Vec<&str> = Vec::new();
    let mut sorted_bit : Vec<u32> = Vec::new();
    for line in file.lines() {
        let mut split = line.split_ascii_whitespace();
        let hand = split.next().unwrap();
        let bit : u32 = split.next().unwrap().parse().unwrap();
        let mut i = 0;
        let len = sorted_hand.len();
        while i < len {
            match compare_p2(hand, sorted_hand[i]) {
                Ordering::Less => {
                    sorted_hand.insert(i, hand);
                    sorted_bit.insert(i, bit);
                    break;
                },
                _ => {}
            }
            i+=1;
        }
        if i >= len {
            sorted_hand.push(hand);
            sorted_bit.push(bit);
        }
    }
    println!("{:?}", sorted_hand);
    println!("{:?}", sorted_bit);
    let mut res = 0;
    for (i, bit) in sorted_bit.iter().enumerate() {
        res += (*bit)*(i+1) as u32;
    }
    println!("{}", res);
}

fn get_type_of_hand(hand : &str) -> u32 {
    let mut counts : BTreeMap<char, u8> = BTreeMap::new();
    for c in hand.chars() {
        match counts.get_mut(&c) {
            Some(v) => {
                *v+=1;
            },
            None => {
                counts.insert(c, 1);
            }
        }
    }
    let mut brelent = false;
    let mut pair = false;
    let mut pair2 = false;
    for (_c , value)in counts {
        if value == 5 {
            return 6;
        }
        else if value == 4 {
            return 5;
        }
        else if value == 3 {
            brelent = true;
        }
        else if value == 2 && !pair {
            pair = true;
        } 
        else if value == 2 && pair {
            pair2 = true;
        }
    }
    if brelent && pair {
        return 4;
    }
    if brelent {
        return 3;
    }
    if pair && pair2 {
        return 2;
    }
    if pair && !pair2 {
        return 1;
    }
    0
}
fn get_type_of_hand_p2(hand : &str) -> u32 {
    let mut nb_j = 0;
    let mut counts : BTreeMap<char, u8> = BTreeMap::new();
    for c in hand.chars() {
        if c == 'J' {
            nb_j += 1;
            continue;
        }
        match counts.get_mut(&c) {
            Some(v) => {
                *v+=1;
            },
            None => {
                counts.insert(c, 1);
            }
        }
    }
    //println!("{hand}");
    let mut key_of_max = 'W';
    let mut max = 0;
    for a in &counts {
        if *a.1 > max {
            max = *a.1;
            key_of_max = *a.0;
        }
    }
    if max == 0 {
        return 6;
    }
    //println!("KEY : {}", key_of_max);
    let b = counts.get_mut(&key_of_max).unwrap();
    *b += nb_j;  
    let mut brelent = false;
    let mut pair = false;
    let mut pair2 = false;
    for (_c , value) in counts {
        if value == 5 {
            return 6;
        }
        else if value == 4 {
            return 5;
        }
        else if value == 3 {
            brelent = true;
        }
        else if value == 2 && !pair {
            pair = true;
        } 
        else if value == 2 && pair {
            pair2 = true;
        }
    }
    if brelent && pair {
        return 4;
    }
    if brelent {
        return 3;
    }
    if pair && pair2 {
        return 2;
    }
    if pair && !pair2 {
        return 1;
    }
    0
}
fn compare_strength(hand1 : &str, hand2 : &str) -> Ordering {
    
    for (i, c) in hand1.char_indices() {
        let ind = CARD.len() - CARD.iter().position(|name| *name == c).unwrap();
        let c2 = hand2.chars().nth(i).unwrap();
        let ind2 = CARD.len() - CARD.iter().position(|name| *name == c2).unwrap();
        //println!("{} {}", ind, ind2);
        if ind != ind2 {
            return ind.cmp(&ind2);
        }
    }
    Ordering::Equal
}
fn compare_strength_p2(hand1 : &str, hand2 : &str) -> Ordering {
    
    for (i, c) in hand1.char_indices() {
        let ind = CARD_P2.len() - CARD_P2.iter().position(|name| *name == c).unwrap();
        let c2 = hand2.chars().nth(i).unwrap();
        let ind2 = CARD_P2.len() - CARD_P2.iter().position(|name| *name == c2).unwrap();
        //println!("{} {}", ind, ind2);
        if ind != ind2 {
            return ind.cmp(&ind2);
        }
    }
    Ordering::Equal
}
fn compare (hand1 : &str , hand2 : &str) -> Ordering {
    
    match get_type_of_hand(hand1).cmp(&get_type_of_hand(hand2)) {
        Ordering::Equal => {
            compare_strength(hand1, hand2) 
        },
        e => { e }
    }
}
fn compare_p2 (hand1 : &str , hand2 : &str) -> Ordering {
    match get_type_of_hand_p2(hand1).cmp(&get_type_of_hand_p2(hand2)) {
        Ordering::Equal => {
            compare_strength_p2(hand1, hand2) 
        },
        e => { e }
    }
}

fn main() {
    //p1();
    p2();

}
