use std::collections::HashSet;

#[allow(dead_code)]
const TEST: &str = r#"abc

a
b
c

ab
ac

a
a
a
a

b"#; // 3, 3, 3, 1, 1

pub fn run() {
    let input = super::get_input(6, "").collect::<Vec<_>>();
    // let input = TEST.lines().collect::<Vec<_>>();

    let mut count_a = 0;
    let mut count_b = 0;

    // let create_all_chars_set  = || {
    //     let mut tmp = HashSet::new();
    //     for c in 'a'..'z' {
    //         tmp.
    //     }
    // };

    for group in input.split(|l| l.len() == 0) {
        let mut chars_in_any_group = HashSet::new();
        let mut chars_in_all_group = ('a'..='z').collect::<HashSet<_>>();
        for answer in group {
            let mut chars_in_group = HashSet::new();
            for c in answer.chars() {
                chars_in_group.insert(c);
            }
            chars_in_all_group = chars_in_all_group.intersection(&chars_in_group).copied().collect::<HashSet<_>>();
            chars_in_any_group = chars_in_any_group.union(&chars_in_group).copied().collect::<HashSet<_>>();
        }
        count_a += chars_in_any_group.len();
        count_b += chars_in_all_group.len();
    }

    println!("a: {}", count_a);
    println!("a: {}", count_b);
}
