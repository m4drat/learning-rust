/// Check a Luhn checksum.
#[allow(clippy::unnecessary_lazy_evaluations)]
pub fn is_valid(code: &str) -> bool {
    const RADIX: u32 = 10;

    let cleaned = code.replace(' ', "");

    if cleaned.len() < 2 {
        return false;
    }

    cleaned
        .chars()
        .rev()
        .enumerate()
        .try_fold(0, |acc, (idx, val)| -> Option<u32> {
            let mut number = val.to_digit(RADIX)?;

            if idx % 2 == 1 {
                println!("idx: {}, {}", idx, (number * 2).gt(&9));
                // We're using `then` here and not the `then_some`, because `then_some` does eager evaluation,
                // which will result in a panic if the number after the substraction is negative.
                number = (number * 2)
                    .gt(&9)
                    .then(|| number * 2 - 9)
                    .or(Some(number * 2))?;
            }

            Some(acc + number)
        })
        .is_some_and(|acc| acc % RADIX == 0)
}
