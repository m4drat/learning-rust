use alphametics::{solve_default, solve_improved};

fn main() {
    let alphametic = "AND + A + STRONG + OFFENSE + AS + A + GOOD == DEFENSE";
    if let Some(solution) = solve_improved(alphametic) {
        println!("Got a solution: {:?}", solution);
    } else {
        println!("Couldn't find a solution :(");
    }
}
