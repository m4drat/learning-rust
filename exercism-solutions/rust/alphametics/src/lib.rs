use itertools::Itertools;
use std::collections::{HashMap, HashSet};

const NUMBERS: &[char] = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    solve_improved(input)
}

pub fn solve_improved(input: &str) -> Option<HashMap<char, u8>> {
    let cleaned_string = input.replace(" ", "").replace("==", "=");
    let unique_chars = cleaned_string
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect::<HashSet<_>>();

    'loop_: for comb in NUMBERS.iter().permutations(unique_chars.len()) {
        let char_to_nums = unique_chars.iter().zip(comb).collect::<HashMap<_, _>>();
        let curr_try = cleaned_string
            .chars()
            .map(|char| match char {
                '+' | '=' => char,
                _ => *char_to_nums[&char],
            })
            .collect::<String>();
        let numbers_str = curr_try.split(['+', '=']).collect::<Vec<_>>();

        for num in numbers_str.iter() {
            if num.chars().nth(0).unwrap() == '0' {
                continue 'loop_;
            }
        }

        let mut numbers = numbers_str
            .iter()
            .map(|num| num.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        let target = numbers.pop().unwrap();

        // println!("{}. {} -> {:?} == {}", i, &curr_try, &numbers, &target);

        if numbers.iter().sum::<u64>() == target {
            return Some(
                char_to_nums
                    .into_iter()
                    .map(|(k, v)| (*k, v.to_string().parse::<u8>().unwrap()))
                    .collect::<HashMap<char, u8>>(),
            );
        }
    }

    None
}

pub fn solve_default(input: &str) -> Option<HashMap<char, u8>> {
    let cleaned_string = input.replace(" ", "").replace("==", "=");
    let unique_chars = cleaned_string
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect::<HashSet<_>>();

    'loop_: for comb in NUMBERS.iter().permutations(unique_chars.len()) {
        let char_to_nums = unique_chars.iter().zip(comb).collect::<HashMap<_, _>>();
        let curr_try = cleaned_string
            .chars()
            .map(|char| match char {
                '+' | '=' => char,
                _ => *char_to_nums[&char],
            })
            .collect::<String>();
        let numbers_str = curr_try.split(['+', '=']).collect::<Vec<_>>();

        for num in numbers_str.iter() {
            if num.chars().nth(0).unwrap() == '0' {
                continue 'loop_;
            }
        }

        let mut numbers = numbers_str
            .iter()
            .map(|num| num.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        let target = numbers.pop().unwrap();

        // println!("{}. {} -> {:?} == {}", i, &curr_try, &numbers, &target);

        if numbers.iter().sum::<u64>() == target {
            return Some(
                char_to_nums
                    .into_iter()
                    .map(|(k, v)| (*k, v.to_string().parse::<u8>().unwrap()))
                    .collect::<HashMap<char, u8>>(),
            );
        }
    }

    None
}
