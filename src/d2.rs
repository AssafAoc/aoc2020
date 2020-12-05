#[allow(dead_code)]
pub fn run() {
    struct Record {
        number_one: u8,
        number_two: u8,
        letter: char,
        password: String,
    }

    let input = super::get_input(2, "").collect::<Vec<_>>();
    // let input = vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"].into_iter().collect::<Vec<_>>();

    let get_record_iterator = || {
        input.iter().map(|line| {
            let mut line_splitted = line.split(' ');
            let range = line_splitted.next().unwrap();
            let mut range_splitted = range.split('-');
            let min = range_splitted.next().unwrap().parse::<u8>().unwrap();
            let max = range_splitted.next().unwrap().parse::<u8>().unwrap();

            let letter = line_splitted.next().unwrap().chars().next().unwrap();
            let password = line_splitted.next().unwrap().to_owned();

            Record { number_one: min, number_two: max, letter, password }
        })
    };

    let number_of_good_passwords_a = get_record_iterator().filter(|r| {
        let number_of_occurrences = r.password.chars().filter(|c| *c == r.letter).count();
        number_of_occurrences >= r.number_one as usize && number_of_occurrences <= r.number_two as usize
    }).count();
    println!("a: {}", number_of_good_passwords_a);

    let number_of_good_passwords_b = get_record_iterator().filter(|r| {
        let mut chars = r.password.chars();
        let char1_is_letter = chars.nth(r.number_one as usize - 1).unwrap() == r.letter;
        let char2_is_letter = chars.nth((r.number_two - r.number_one) as usize - 1).unwrap() == r.letter;
        (char1_is_letter && !char2_is_letter) || (!char1_is_letter && char2_is_letter)
    }).count();
    println!("b: {}", number_of_good_passwords_b);
}
