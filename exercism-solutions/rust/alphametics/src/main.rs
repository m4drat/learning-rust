use alphametics::solve;

fn main() {
    // let alphametic = "AND + A + STRONG + OFFENSE + AS + A + GOOD == DEFENSE";
    let alphametic = "ACI + CI = DFI";
    if let Some(solution) = solve(alphametic) {
        println!("Got a solution: {:?}", solution);
    } else {
        println!("Couldn't find a solution :(");
    }
}
