/*
TODO:
- What the heck does [..] do?
- Usage of 'static lifetime, " this function's return type contains a borrowed value, but no value to be borrowed from"
- Splitting on whitespace without using parser combinators or regex seems hard? Was this just an OS system issue when I pasted the input file, if so code the probably only works for this OS?
- In validator, I use .as_ref() on an option a couple of times to avoid E0507 error. Explore as_ref()
 */

use crate::parsers::{parse_key_value, parse_possible_passports, Passport};

pub fn day04_1(input: &str) -> usize {
    find_passport_candidates(input)
        .iter()
        .map(|y| find_passport_keys(y))
        .filter(|x| validate_passport(x))
        .count()
}

pub fn day04_2(input: &str) -> usize {
    match parse_possible_passports(input) {
        Err(_) => 0,
        Ok((_, passports)) => passports
            .iter()
            .map(String::as_str)
            .map(parse_key_value)
            .map(Passport::new)
            .filter(|p| p.is_valid())
            .count(),
    }
}

fn find_passport_candidates(input: &str) -> Vec<&str> {
    input
        .split("\r\n\r\n")
        .filter(|&x| !x.trim().is_empty())
        .map(str::trim)
        .collect()
}

fn find_passport_keys(passport_candidates: &str) -> Vec<&str> {
    passport_candidates
        .split(&[' ', '\n'][..])
        .filter(|&x| !x.trim().is_empty())
        .flat_map(|x| x.split(':'))
        .filter(|x| get_required_keys().contains(x))
        .collect()
}

fn get_required_keys() -> Vec<&'static str> {
    vec!["ecl", "pid", "eyr", "hcl", "byr", "iyr", "hgt"]
}

fn validate_passport(passport_keys: &Vec<&str>) -> bool {
    let required_keys = get_required_keys();
    required_keys.iter().all(|x| passport_keys.contains(x))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    const SNIPPET: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
    byr:1937 iyr:2017 cid:147 hgt:183cm\r\n\r\n\
    iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
    hcl:#cfa07d byr:1929\r\n\r\n\
    hcl:#ae17e1 iyr:2013
    eyr:2024
    ecl:brn pid:760753108 byr:1931
    hgt:179cm\r\n\r\n\
    hcl:#cfa07d eyr:2025 pid:166559648
    iyr:2011 ecl:brn hgt:59in";

    #[test]
    fn it_should_count_number_of_valid_passports() {
        assert_eq!(day04_1(SNIPPET), 2);
    }

    #[test]
    fn it_should_ignore_blank_lines() {
        let expected = vec![
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
    byr:1937 iyr:2017 cid:147 hgt:183cm",
            "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
    hcl:#cfa07d byr:1929",
            "hcl:#ae17e1 iyr:2013
    eyr:2024
    ecl:brn pid:760753108 byr:1931
    hgt:179cm",
            "hcl:#cfa07d eyr:2025 pid:166559648
    iyr:2011 ecl:brn hgt:59in",
        ];
        assert_eq!(find_passport_candidates(SNIPPET), expected);
        assert_eq!(find_passport_candidates(SNIPPET).len(), 4);
    }

    #[test]
    fn it_should_find_all_the_required_passport_keys() {
        let snippet = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
    byr:1937 iyr:2017 cid:147 hgt:183cm";
        let expected = vec!["ecl", "pid", "eyr", "hcl", "byr", "iyr", "hgt"];
        assert_eq!(find_passport_keys(snippet), expected);
    }

    #[test]
    fn it_should_validate_allowable_passport() {
        let mut hash: HashMap<&str, bool> = HashMap::new();
        hash.insert(
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
    byr:1937 iyr:2017 cid:147 hgt:183cm",
            true,
        );
        hash.insert(
            "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
    hcl:#cfa07d byr:1929",
            false,
        );
        hash.insert(
            "hcl:#ae17e1 iyr:2013
    eyr:2024
    ecl:brn pid:760753108 byr:1931
    hgt:179cm",
            true,
        );
        hash.insert(
            "hcl:#cfa07d eyr:2025 pid:166559648
    iyr:2011 ecl:brn hgt:59in",
            false,
        );

        hash.iter()
            .map(|(&x, &y)| assert_eq!(validate_passport(&find_passport_keys(x)), y))
            .last();
    }

    #[test]
    fn it_should_count_valid_passports() {
        let invalid_passports: &str = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

        let valid_passports: &str = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

        assert_eq!(day04_2(invalid_passports), 0);
        assert_eq!(day04_2(valid_passports), 4)
    }
}
