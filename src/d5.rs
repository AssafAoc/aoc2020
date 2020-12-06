use std::collections::BTreeSet;

#[allow(dead_code)]
const TEST: &str = r#"FBFBBFFRLR
BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL"#; // 357, 567, 119, 820

#[allow(dead_code)]
pub fn run() {
    let input = super::get_input(5, "");
    // let input = TEST.lines();

    let mut seat_ids = BTreeSet::new();

    for boarding_pass in input {
        let mut min_row = 0;
        let mut max_row = 127;
        let mut min_col = 0;
        let mut max_col = 7;
        let mut chars = boarding_pass.chars();
        for _ in 0..7 {
            match chars.next() {
                Some('F') => max_row = (max_row + min_row) / 2,
                Some('B') => min_row = (max_row + min_row) / 2,
                None | Some(_) => panic!("error")
            }
        }
        for _ in 0..3 {
            match chars.next() {
                Some('L') => max_col = (max_col + min_col) / 2,
                Some('R') => min_col = (max_col + min_col) / 2,
                None | Some(_) => panic!("error")
            }
        }
        let seat_id = max_row * 8 + max_col;
        seat_ids.insert(seat_id);
        // max_seat_id = max_seat_id.max(seat_id);
    }

    println!("a: {}", seat_ids.iter().max().unwrap());
    let mut seat_id_iterator = seat_ids.iter().peekable();
    let mut previous_id = seat_id_iterator.next().unwrap();
    'find_seat: for current_id in seat_id_iterator {
        if current_id - previous_id == 2 {
            println!("b: {}", previous_id + 1);
            break 'find_seat;
        }
        previous_id = current_id;
    }
}
