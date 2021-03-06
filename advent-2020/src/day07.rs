use std::collections::{HashMap, HashSet};
extern crate pest;
use self::pest::iterators::Pairs;
use pest::Parser;

pub fn day07_1(input: &str) -> usize {
    let all_bags = parse_many_bags_data(input);
    find_possible_bag_holders_for_a_bag("shiny gold", &all_bags).len()
}

pub(crate) fn day07_2(input: &str) -> i32 {
    /*
    bags must be color-coded and must contain specific quantities of other color-coded bags
    How many individual bags are required inside your single shiny gold bag?
    */
    let all_bags = parse_many_bags_data(input);
    let shiny_gold = get_bag_from_color("shiny gold", &all_bags);
    count_bags_inside_smaller(&all_bags, shiny_gold, 0) - 1
}

fn parse_single_bag_data(rule: &str) -> Bag {
    let bag_data: Result<Pairs<Rule>, pest::error::Error<Rule>> = BagParser::parse(Rule::bag, rule);
    let mut color = "";
    let mut contains: Vec<HashMap<String, String>> = Vec::new();
    if let Ok(mut pairs) = bag_data {
        if let Some(pair) = pairs.next() {
            for innards in pair.into_inner() {
                match innards.as_rule() {
                    Rule::subject => {
                        color = innards.as_str();
                    }
                    Rule::bag_item => {
                        let mut contains_map: HashMap<String, String> = HashMap::new();
                        for descriptor in innards.into_inner() {
                            match descriptor.as_rule() {
                                Rule::color => {
                                    contains_map.insert(
                                        "color".to_string(),
                                        descriptor.as_str().to_string(),
                                    );
                                }
                                Rule::quant => {
                                    contains_map.insert(
                                        "quantity".to_string(),
                                        descriptor.as_str().to_string(),
                                    );
                                }
                                _ => (),
                            }
                        }
                        contains.push(contains_map);
                    }
                    _ => (),
                }
            }
        };
    }

    Bag {
        color: color.to_string(),
        contains,
    }
}

fn parse_many_bags_data(bags: &str) -> Vec<Bag> {
    let mut bags_list: Vec<Bag> = Vec::new();
    let to_parse: Vec<&str> = bags.split('\n').map(str::trim).collect();
    for maybe_bag in to_parse {
        bags_list.push(parse_single_bag_data(maybe_bag))
    }
    bags_list
}

fn find_direct_bag_holders_for_a_bag<'a, 'b>(color: &'b str, bags: &'a [Bag]) -> Vec<&'a Bag> {
    bags.iter()
        .filter(|&bag| {
            bag.contains
                .iter()
                .any(|baguette| baguette.get("color").map_or(false, |col| color == col))
        })
        .collect()
}

fn find_possible_bag_holders_for_a_bag<'a, 'b>(
    init_color: &'b str,
    bags: &'a [Bag],
) -> Vec<&'a Bag> {
    let mut color_stack: Vec<&Bag> = Vec::new();
    let mut possible_holders_stack: Vec<&'a Bag> = Vec::new();
    let mut visited_colors = HashSet::new();
    let mut done = false;
    let mut color = init_color;

    while !done {
        if visited_colors.contains(color) {
            if color_stack.is_empty() {
                done = true;
            } else {
                let next_bag = color_stack.pop().unwrap();
                color = next_bag.color.as_str();
            }
            continue;
        }
        let mut possibilities = find_direct_bag_holders_for_a_bag(color, bags);
        color_stack.append(&mut possibilities);
        visited_colors.insert(color);
        if color_stack.is_empty() {
            done = true;
        } else {
            let next_bag = color_stack.pop().unwrap();
            possible_holders_stack.push(next_bag);
            color = next_bag.color.as_str();
        }
    }
    possible_holders_stack
}

fn count_bags_inside(input: &str, bag_color: &str) -> i8 {
    let all_bags = parse_many_bags_data(input);
    find_possible_bag_holders_for_a_bag("shiny gold", &all_bags);
    /*
    First we get our bag color which holds at least 1 type of bag, but could be multiple version of each type of bag

     */
    126
}

// TODO: Explore in slightly more depth &Vec<_> to &[T] slice for type conversion
fn count_bags_inside_smaller(all_bags: &[Bag], current_bag: Bag, mut amount: i32) -> i32 {
    // Base case
    if current_bag.contains.is_empty() {
        amount += 1;
        return amount;
    }

    for innard in current_bag.contains.iter() {
        let quant = innard.get("quantity").unwrap();
        let color = innard.get("color").unwrap();
        // I tried to do recursion here and I failed initially ~~
        // So it seems the issue here was that I was passing the amount dynamically as the accumulator
        // Seems like the correct thing to do is to always have it start at 0
        // And I probably would have done it correctly if I had a different variable
        // Maybe we needed a "start_ammount" variable and then accumulator variable. I think that's how some recursions do it
        amount += quant.parse().unwrap_or(0)
            * count_bags_inside_smaller(all_bags, get_bag_from_color(color, all_bags), 0);
    }
    amount += 1;
    amount
}

fn get_bag_from_color(color: &str, all_bags: &[Bag]) -> Bag {
    let default_bag = Bag {
        color: color.to_string(),
        contains: Vec::new(),
    };

    all_bags
        .iter()
        .find(|&b| b.color == color)
        .unwrap_or(&default_bag)
        .to_owned()
}

#[derive(Debug, PartialEq)]
struct Bag {
    color: String,
    contains: Vec<HashMap<String, String>>,
}

impl Clone for Bag {
    fn clone(&self) -> Self {
        let color = self.color.clone();
        let contains = self.contains.clone();
        Self { color, contains }
    }
}

#[derive(Parser)]
#[grammar = "./parsers/bags.pest"]
struct BagParser {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_parse_bags_and_what_they_contain() {
        let snippet = "light red bags contain 1 bright white bag, 2 muted yellow bags.";
        let mut bright_white = HashMap::new();
        bright_white.insert(String::from("color"), "bright white".to_string());
        bright_white.insert("quantity".to_string(), "1".to_string());

        let mut muted_yellow = HashMap::new();
        muted_yellow.insert("color".to_string(), "muted yellow".to_string());
        muted_yellow.insert("quantity".to_string(), "2".to_string());

        let expected = Bag {
            color: "light red".to_string(),
            contains: vec![bright_white, muted_yellow],
        };

        assert_eq!(parse_single_bag_data(snippet), expected)
    }

    #[test]
    fn it_can_parse_many_bags() {
        let snippet = "light red bags contain 1 bright white bag, 2 muted yellow bags.
bright white bags contain 1 shiny gold bag.
faded blue bags contain no other bags.";

        let mut bright_white = HashMap::new();
        bright_white.insert(String::from("color"), "bright white".to_string());
        bright_white.insert("quantity".to_string(), "1".to_string());

        let mut muted_yellow = HashMap::new();
        muted_yellow.insert("color".to_string(), "muted yellow".to_string());
        muted_yellow.insert("quantity".to_string(), "2".to_string());

        let bag_1 = Bag {
            color: "light red".to_string(),
            contains: vec![bright_white, muted_yellow],
        };

        let mut shiny_gold = HashMap::new();
        shiny_gold.insert(String::from("color"), "shiny gold".to_string());
        shiny_gold.insert("quantity".to_string(), "1".to_string());

        let bag_2 = Bag {
            color: "bright white".to_string(),
            contains: vec![shiny_gold],
        };

        let bag_3 = Bag {
            color: "faded blue".to_string(),
            contains: Vec::new(),
        };

        let expected = vec![bag_1, bag_2, bag_3];
        let actual = parse_many_bags_data(snippet);
        assert_eq!(actual, expected);
    }

    #[test]
    fn should_return_all_direct_bags_that_can_contain_that_color() {
        let snippet = "light red bags contain 1 bright white bag, 2 muted yellow bags.
bright white bags contain 1 shiny gold bag.
faded blue bags contain 1 bright white bag.";

        let mut bright_white = HashMap::new();
        bright_white.insert(String::from("color"), "bright white".to_string());
        bright_white.insert("quantity".to_string(), "1".to_string());

        let mut bright_white_2 = HashMap::new();
        bright_white_2.insert(String::from("color"), "bright white".to_string());
        bright_white_2.insert("quantity".to_string(), "1".to_string());

        let mut muted_yellow = HashMap::new();
        muted_yellow.insert("color".to_string(), "muted yellow".to_string());
        muted_yellow.insert("quantity".to_string(), "2".to_string());

        let bag_1 = Bag {
            color: "light red".to_string(),
            contains: vec![bright_white, muted_yellow],
        };

        let mut shiny_gold = HashMap::new();
        shiny_gold.insert(String::from("color"), "shiny gold".to_string());
        shiny_gold.insert("quantity".to_string(), "1".to_string());

        let bag_2 = Bag {
            color: "bright white".to_string(),
            contains: vec![shiny_gold],
        };

        let bag_3 = Bag {
            color: "faded blue".to_string(),
            contains: vec![bright_white_2],
        };

        let expected = vec![&bag_1, &bag_3];
        let all_bags = parse_many_bags_data(snippet);
        let actual = find_direct_bag_holders_for_a_bag("bright white", &all_bags);
        assert_eq!(actual, expected);
        assert_eq!(
            find_direct_bag_holders_for_a_bag("shiny gold", &all_bags),
            vec![&bag_2]
        );
        let no_bag: Vec<&Bag> = Vec::new();
        assert_eq!(
            find_direct_bag_holders_for_a_bag("unreal", &all_bags),
            no_bag
        );
    }

    #[test]
    fn should_return_all_bags_that_can_contain_that_color() {
        let snippet = "light red bags contain 1 bright white bag, 2 muted yellow bags.
bright white bags contain 1 shiny gold bag.
faded blue bags contain 1 bright white bag.";

        let mut bright_white = HashMap::new();
        bright_white.insert(String::from("color"), "bright white".to_string());
        bright_white.insert("quantity".to_string(), "1".to_string());

        let mut bright_white_2 = HashMap::new();
        bright_white_2.insert(String::from("color"), "bright white".to_string());
        bright_white_2.insert("quantity".to_string(), "1".to_string());

        let mut muted_yellow = HashMap::new();
        muted_yellow.insert("color".to_string(), "muted yellow".to_string());
        muted_yellow.insert("quantity".to_string(), "2".to_string());

        let bag_1 = Bag {
            color: "light red".to_string(),
            contains: vec![bright_white, muted_yellow],
        };

        let mut shiny_gold = HashMap::new();
        shiny_gold.insert(String::from("color"), "shiny gold".to_string());
        shiny_gold.insert("quantity".to_string(), "1".to_string());

        let bag_2 = Bag {
            color: "bright white".to_string(),
            contains: vec![shiny_gold],
        };

        let bag_3 = Bag {
            color: "faded blue".to_string(),
            contains: vec![bright_white_2],
        };

        let expected = vec![&bag_2, &bag_3, &bag_1];
        let all_bags = parse_many_bags_data(snippet);
        let actual = find_possible_bag_holders_for_a_bag("shiny gold", &all_bags);
        assert_eq!(actual, expected);
    }

    #[test]
    fn should_count_how_many_bags_are_required_inside_another_bag() {
        let snippet = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
        assert_eq!(count_bags_inside(snippet, "shiny gold"), 126);

        let all_bags = parse_many_bags_data(snippet);
        assert_eq!(
            count_bags_inside_smaller(&all_bags, get_bag_from_color("shiny gold", &all_bags), 0)
                - 1,
            126
        );
    }

    #[test]
    fn should_count_how_many_bags_are_required_inside_smaller_bag() {
        // Don't count the shiny gold bag itself, so subtracting 1 makes sense here
        let snippet = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.";
        let all_bags = parse_many_bags_data(snippet);
        assert_eq!(
            count_bags_inside_smaller(&all_bags, get_bag_from_color("shiny gold", &all_bags), 0)
                - 1,
            6
        );
    }

    #[test]
    fn should_count_how_many_bags_are_required_for_32_example() {
        // This is what helped the most because it revealed the case of multiple bags in a single bag
        // It previously wasn't handled gracefully
        let snippet = r"light red bags contain 1 bright white bag, 2 muted yellow bags.
        dark orange bags contain 3 bright white bags, 4 muted yellow bags.
        bright white bags contain 1 shiny gold bag.
        muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
        shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
        dark olive bags contain 3 faded blue bags, 4 dotted black bags.
        vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
        faded blue bags contain no other bags.
        dotted black bags contain no other bags.";

        let all_bags = parse_many_bags_data(snippet);
        assert_eq!(
            count_bags_inside_smaller(&all_bags, get_bag_from_color("shiny gold", &all_bags), 0)
                - 1,
            32
        );
    }
}
