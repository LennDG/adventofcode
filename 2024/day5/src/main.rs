use std::collections::HashMap;

fn main() {
    let (rules_input, updates_input) = include_str!("example").split_once("\n\n").unwrap();

    let rules = parse_rules(rules_input);
    let updates = parse_updates(updates_input);
    let rules_reversed = parse_rules_reversed(rules_input);
    //part_one(rules, updates);
    part_two(rules, rules_reversed, updates);
}

fn part_one(rules: HashMap<u32, Vec<u32>>, updates: Vec<Vec<u32>>) {
    let mut sum = 0;

    for update in updates {
        let lookup = updates_lookup(&update);

        if check_valid(&rules, &update) {
            sum += get_middle_page(&update);
        }
    }

    println!("{}", sum);
}

fn part_two(
    rules: HashMap<u32, Vec<u32>>,
    rules_reversed: HashMap<u32, Vec<u32>>,
    updates: Vec<Vec<u32>>,
) {
    let mut sum = 0;
    // Algorithm:
    // For the invalid updates
    // For each offending page, move it to the end of the update
    // Continue from the same index
    for mut update in updates {
        if !check_valid(&rules, &update) {
            println!("Invalid update: {:?}", update);
            let lookup = updates_lookup(&update); // duplicate lookup creation but eh.
            let mut i = 0;
            while i < update.len() {
                let mut changed_update = false;
                // Check if there is a rule for this page
                if let Some(befores) = rules_reversed.get(&update[i]) {
                    // Check if the rule applies to any pages after this one
                    for before in befores {
                        if let Some(j) = lookup.get(&before) {
                            // If the after page is before the current index, move it to the end
                            if *j > i {
                                // Move this page to the end of the update
                                let moved = update.remove(i);
                                update.push(moved);
                                println!("{}", moved);
                                println!("{:?}", update);
                                changed_update = true;
                                break;
                            }
                        }
                    }
                }

                if !changed_update {
                    i += 1;
                    if i > update.len() / 2 {
                        break;
                    }
                }
            }

            sum += get_middle_page(&update);
        }
    }

    println!("{}", sum);
}

fn check_valid(rules: &HashMap<u32, Vec<u32>>, update: &Vec<u32>) -> bool {
    let lookup = updates_lookup(&update);

    update.iter().enumerate().all(|(i, s)| {
        if let Some(after) = rules.get(s) {
            // If the index of the "after" page is greater than the current index,
            // then this page is valid
            after.iter().all(|after| {
                if let Some(index) = lookup.get(after) {
                    *index > i
                } else {
                    true
                }
            })
        } else {
            true
        }
    })
}

fn get_middle_page(update: &Vec<u32>) -> u32 {
    update[update.len() / 2]
}

fn parse_rules(rules: &str) -> HashMap<u32, Vec<u32>> {
    let mut map = HashMap::new();

    for rule in rules.lines() {
        let (first, second) = rule.split_once("|").unwrap();
        let first = first.parse::<u32>().unwrap();
        let second = second.parse::<u32>().unwrap();

        map.entry(first).or_insert(vec![]).push(second);
    }

    map
}

fn parse_rules_reversed(rules: &str) -> HashMap<u32, Vec<u32>> {
    let mut map = HashMap::new();

    for rule in rules.lines() {
        let (first, second) = rule.split_once("|").unwrap();
        let first = first.parse::<u32>().unwrap();
        let second = second.parse::<u32>().unwrap();

        map.entry(second).or_insert(vec![]).push(first);
    }

    map
}

fn parse_updates(updates: &str) -> Vec<Vec<u32>> {
    updates
        .lines()
        .map(|line| line.split(",").map(|s| s.parse::<u32>().unwrap()).collect())
        .collect()
}

fn updates_lookup(updates: &Vec<u32>) -> HashMap<u32, usize> {
    updates.iter().enumerate().map(|(i, s)| (*s, i)).collect()
}
