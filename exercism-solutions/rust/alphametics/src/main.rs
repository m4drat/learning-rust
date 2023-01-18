use alphametics::solve_default;

fn main() {
    let alphametic = "AND + A + STRONG + OFFENSE + AS + A + GOOD == DEFENSE";
    if let Some(solution) = solve_default(alphametic) {
        println!("Got a solution: {:?}", solution);
    } else {
        println!("Couldn't find a solution :(");
    }
}
