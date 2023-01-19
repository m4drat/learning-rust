use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    iter::zip,
};

fn find_factors(input: &str) -> HashMap<char, i64> {
    let mut factors: HashMap<char, i64> = HashMap::with_capacity(10);
    let mut position: u32 = 0;
    let mut sign: i64 = -1;

    input
        .chars()
        .filter(|ch| !ch.is_whitespace())
        .rev()
        .for_each(|ch| match ch {
            '=' => {
                sign = 1;
                position = 0
            }
            '+' => {
                position = 0;
            }
            _ => {
                *factors.entry(ch).or_insert(0) += sign * 10_i64.pow(position);
                position += 1;
            }
        });

    factors
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let first_letters = input
        .split(&['+', '='])
        .filter_map(|part| part.trim().chars().nth(0))
        .collect::<HashSet<_>>();
    let factors = find_factors(input);

    for perm in (0..=9).permutations(factors.len()) {
        let sum = factors
            .values()
            .zip(&perm)
            .fold(0, |sum, (factor, num)| sum + factor * num);
        let first_char_is_invalid = factors
            .keys()
            .zip(&perm)
            .any(|(ch, num)| *num == 0 && first_letters.contains(ch));
        if sum == 0 && !first_char_is_invalid {
            return Some(
                factors
                    .keys()
                    .zip(&perm)
                    .map(|(k, v)| (*k, *v as u8))
                    .collect::<HashMap<_, _>>(),
            );
        }
    }

    None
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

    for perm in (0..=9).permutations(unique_chars.len()) {
        let curr_try: HashMap<&char, &u8> =
            HashMap::from_iter(zip(unique_chars.iter(), perm.iter()));

        // if lhs
        //     .iter()
        //     .any(|num_str| **curr_try.get(&num_str.chars().next().unwrap()).unwrap() == 0)
        //     || **curr_try.get(&rhs.chars().next().unwrap()).unwrap() == 0
        // {
        //     continue;
        // }

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
