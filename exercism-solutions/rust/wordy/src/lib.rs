#[derive(Debug)]
enum Operations {
    Add,
    Sub,
    Mul,
    Div,
    Exp,
    Num(i32),
}

impl Operations {
    fn try_get_num(&self) -> Option<i32> {
        if let Operations::Num(num) = self {
            return Some(*num);
        }

        None
    }
}

fn word_to_operation(word: &str) -> Option<Operations> {
    Some(match word {
        "plus" => Operations::Add,
        "minus" => Operations::Sub,
        "multiplied" => Operations::Mul,
        "divided" => Operations::Div,
        "raised" => Operations::Exp,
        _ => Operations::Num({
            let word = word.strip_suffix("nd").unwrap_or(word);
            word.strip_suffix("th")
                .unwrap_or(word)
                .parse::<i32>()
                .ok()?
        }),
    })
}

fn parse(command: &str) -> Option<Vec<Operations>> {
    let command = command.strip_prefix("What is ")?.strip_suffix('?')?;

    command
        .to_lowercase()
        .split_whitespace()
        .filter(|chunk| chunk != &"by" && chunk != &"to" && chunk != &"the" && chunk != &"power")
        .map(word_to_operation)
        .collect::<Option<Vec<Operations>>>()
}

fn eval(operations: &[Operations]) -> Option<i32> {
    let mut result = operations.first()?.try_get_num()?;

    // Check that we have enough tokens to perform all operations.
    if operations.len() % 2 != 1 {
        return None;
    }

    for chunk in operations[1..].chunks(2) {
        let operation = &chunk[0];
        let operand = chunk[1].try_get_num()?;

        match operation {
            Operations::Add => result += operand,
            Operations::Sub => result -= operand,
            Operations::Mul => result *= operand,
            Operations::Div => result /= operand,
            Operations::Exp => result = result.pow(operand as u32),
            _ => return None,
        };
    }

    Some(result)
}

pub fn answer(command: &str) -> Option<i32> {
    let operations = parse(command)?;
    eval(&operations)
}
