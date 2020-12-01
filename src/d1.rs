fn find_pair(numbers: &[u32]) -> (u32, u32) {
    for i in 0..numbers.len() {
        for j in (i+1)..numbers.len() {
            if numbers[i] + numbers[j] == 2020 {
                return (numbers[i], numbers[j])
            }
        }
    }
    panic!("couldn't find numbers")
}

fn find_triple(numbers: &[u32]) -> (u32, u32, u32) {
    for i in 0..numbers.len() {
        for j in (i+1)..numbers.len() {
            for k in (j+1)..numbers.len() {
                if numbers[i] + numbers[j] + numbers[k] == 2020 {
                    return (numbers[i], numbers[j], numbers[k])
                }
            }
        }
    }
    panic!("couldn't find numbers")
}

pub fn run() {
    let input = crate::get_input(1, "");
    // let test_vec = vec![1721, 979, 366, 299, 675, 1456]; let input = test_vec.iter().map(|i| i.to_string());

    let numbers = input.map(|s| s.parse::<u32>().unwrap()).collect::<Vec<_>>();

    let (a, b) = find_pair(&numbers);
    println!("a: {}", a * b);

    let (a, b, c) = find_triple(&numbers);
    println!("b: {}", a * b * c);
}
