use crate::get_input;
use std::collections::HashSet;

#[allow(dead_code)]
const TEST: &str = r#"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
"#;

#[derive(Debug, Copy, Clone)]
enum Op {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

fn a(ribbon: &[Op]) -> i32 {
    let mut position = 0i32;
    let mut acc = 0;

    let mut visited_position = HashSet::new();

    while !visited_position.contains(&position) {
        visited_position.insert(position);
        match ribbon[position as usize] {
            Op::Acc(i) => {
                acc += i;
                position += 1;
            }
            Op::Jmp(offset) => position += offset,
            Op::Nop(_) => position += 1,
        }
    }

    acc
}

fn b(ribbon: &[Op]) -> i32 {
    let mut last_changed_position = None;
    let acc = 'find_loop: loop {
        let mut position = 0i32;
        let mut acc = 0;

        let mut visited_position = HashSet::new();

        let mut ribbon_attempt = ribbon.to_vec();

        let p = ribbon_attempt.iter_mut().skip(last_changed_position.unwrap_or(0)).position(|o| matches!(o, Op::Jmp(_)) || matches!(o, Op::Nop(_))).map(|p| p + last_changed_position.unwrap_or(0)).expect("ran out of ops before finding solution");

        if let Op::Jmp(i) = ribbon_attempt[p] {
            ribbon_attempt[p] = Op::Nop(i);
            last_changed_position = Some(p + 1);
        } else if let Op::Nop(i) = ribbon_attempt[p] {
            ribbon_attempt[p] = Op::Jmp(i);
            last_changed_position = Some(p + 1);
        }

        while !visited_position.contains(&position) && (position as usize) < ribbon_attempt.len() {
            visited_position.insert(position);
            match ribbon_attempt[position as usize] {
                Op::Acc(i) => {
                    acc += i;
                    position += 1;
                }
                Op::Jmp(offset) => position += offset,
                Op::Nop(_) => position += 1,
            }
        }

        if position as usize == ribbon.len() {
            break 'find_loop acc
        }
    };

    acc
}

#[allow(dead_code)]
pub fn run() {
    let input = get_input(8, "");
    // let input = TEST.lines();

    let ribbon = input.map(|s| {
        let mut splitted = s.split(" ");
        let op = splitted.next().unwrap();
        let num = splitted.next().unwrap().parse::<i32>().unwrap();
        match op {
            "nop" => Op::Nop(num),
            "jmp" => Op::Jmp(num),
            "acc" => Op::Acc(num),
            _ => panic!(format!("no op named: {}", op))
        }
    }).collect::<Vec<_>>();

    println!("a: {}", a(&ribbon));
    println!("b: {}", b(&ribbon));
}
