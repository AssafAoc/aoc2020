#[allow(dead_code)]
const TEST: &str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

#[allow(dead_code)]
pub fn run() {
    let input = super::get_input(3, "");
    // let input = TEST.split('\n').map(|s| s.to_owned());

    let world = input.map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    // A
    let step = (3usize, 1usize);
    let mut current_position = (0usize, 0usize);

    let mut hit_trees = 0;
    while current_position.1 < world.len() {
        let c = world[current_position.1][current_position.0 % world[0].len()];
        if c == '#' {
            hit_trees += 1;
        }
        current_position = (current_position.0 + step.0, current_position.1 + step.1);
    }
    println!("a: {}", hit_trees);
    // END A

    //B
    let mut hit_trees_product = 1usize;
    for step in [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)].iter() {
        let mut current_position = (0usize, 0usize);

        let mut hit_trees = 0;
        while current_position.1 < world.len() {
            let c = world[current_position.1][current_position.0 % world[0].len()];
            if c == '#' {
                hit_trees += 1;
            }
            current_position = (current_position.0 + step.0, current_position.1 + step.1);
        }
        hit_trees_product *= hit_trees;
    }

    println!("b: {}", hit_trees_product);
    // END B
}