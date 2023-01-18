use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    iter::zip,
};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    solve_improved(input)
}

pub fn solve_improved(input: &str) -> Option<HashMap<char, u8>> {
    let equation = input.split("==").collect::<Vec<_>>();
    let lhs = equation[0].trim().split(" + ").collect::<Vec<_>>();
    let rhs = equation[1].trim();
    let unique_chars = input
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect::<HashSet<_>>();

    if unique_chars.len() > 10 {
        return None;
    }

    for perm in (0..10).permutations(unique_chars.len()) {
        let curr_try: HashMap<&char, &u8> =
            HashMap::from_iter(zip(unique_chars.iter(), perm.iter()));

        let lhs_sum = lhs.iter().try_fold(0 as u64, |sum, elem| {
            Some(sum + check_and_convert(elem, &curr_try)?)
        });
        let rhs_sum = check_and_convert(rhs, &curr_try);

        if lhs_sum.is_none() || rhs_sum.is_none() {
            continue;
        }

        if rhs_sum.unwrap() == lhs_sum.unwrap() {
            return Some(HashMap::from_iter(curr_try.iter().map(|(k, v)| (**k, **v))));
        }
    }

    None
}

fn check_and_convert(to_convert: &str, map: &HashMap<&char, &u8>) -> Option<u64> {
    if **map.get(&to_convert.chars().next()?)? == 0 {
        return None;
    }

    let mut curr_mult = 10_u64.pow(to_convert.len() as u32);
    Some(to_convert.chars().fold(0, |sum, ch| {
        let curr_num = **map.get(&ch).unwrap() as u64;

        curr_mult /= 10;
        sum + curr_num * curr_mult
    }))
}

const NUMBERS: &[char] = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

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
