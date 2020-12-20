use std::collections::{HashMap, HashSet, BTreeSet};

// TODO make it work, skipping for now

#[allow(dead_code)]
const TEST: &str = r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."#;

#[allow(dead_code)]
pub fn run() {
    let input = super::get_input(7, "");
    // let input = TEST.lines();

    let mut container_to_contained = HashMap::new();
    let mut contained_to_container = HashMap::new();

    for line in input {
        let mut splitted = line[0..line.len()-1].split(" bags contain ");
        let container_bag = splitted.next().unwrap().to_owned();
        container_to_contained.insert(container_bag.to_owned(), HashMap::new());

        for count_and_bag in splitted.next().unwrap().split(", ") {
            if count_and_bag != "no other bags" {
                let mut splitted_contained_bag = count_and_bag.splitn(2, ' ');
                let count = splitted_contained_bag.next().unwrap().parse::<u8>().unwrap();
                let contained_bag = splitted_contained_bag.next().unwrap();
                let contained_bag = contained_bag[0..contained_bag.find("bag").unwrap()-1].to_owned();

                container_to_contained.get_mut(&container_bag).unwrap().insert(contained_bag.clone(), count);
                contained_to_container.entry(contained_bag.to_owned()).or_insert(BTreeSet::new()).insert(container_bag.clone());
            }
        }
    }

    let mut options = 0;
    let mut stack = vec!["shiny gold".to_owned()];
    let mut stack = vec![("shiny gold".to_owned(), 0)];
    // let mut stack = vec![("plaid purple".to_owned(), 0)];
    let mut visited = HashSet::new();

    // while let Some(current) = stack.pop() {
    let mut all = 0;
    while let Some((current, indentation)) = stack.pop() {
        if !visited.contains(&current) {
            println!("{:indent$}{}", "", current, indent=indentation);
            visited.insert(current.clone());
            if let Some(current_containers) = contained_to_container.get(&current) {
                for container in current_containers {
                    if !visited.contains(container) {
                        options += 1;
                        stack.push((container.clone(), indentation + 4));
                    }
                    // stack.push(container.clone());
                }
            }
        }
        // let current = stack.pop().unwrap();
    }
    println!("a: {}", options);
    // let mut contains = vec!["shiny gold"];
    // let mut can_contain = HashSet::new();
    // for
}