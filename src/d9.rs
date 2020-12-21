use crate::get_input;

const TEST: &str = r#"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576
"#; // ANSWER A: 127, B: 62

fn has_two_add_to_number(number: u64, slice: &[u64]) -> bool {
    for i in 0..slice.len() {
        for j in (i + 1)..slice.len() {
            if slice[i] + slice[j] == number {
                return true;
            }
        }
    }
    false
}

fn a(numbers: &[u64], preamble_len: usize) -> u64 {
    for current in (preamble_len)..numbers.len() {
        if !has_two_add_to_number(numbers[current], &numbers[current - preamble_len..current]) {
            return numbers[current];
        }
    }
    panic!("hack not found");
}

fn b(numbers: &[u64], preamble_len: usize) -> u64 {
    let number = a(numbers, preamble_len);
    let mut low = 0;
    let mut high = 1;
    loop {
        let slice = &numbers[low..high];
        let slice_sum = slice.iter().sum::<u64>();
        if slice_sum == number {
            return slice.iter().min().unwrap() + slice.iter().max().unwrap();
        }
        if slice_sum < number {
            high += 1;
        } else {
            low += 1;
            high = low + 1;
        }
    }
    panic!("hack not found");
}

pub fn run() {
    let (input, preamble_len) = (get_input(9, ""), 25usize);
    // let (input, preamble_len) = (TEST.lines(), 5usize);

    let numbers = input.map(|s| s.parse::<u64>().unwrap()).collect::<Vec<_>>();

    println!("a: {}", a(&numbers, preamble_len));
    println!("b: {}", b(&numbers, preamble_len));
}