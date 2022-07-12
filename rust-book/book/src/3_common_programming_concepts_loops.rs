fn main() {
    let mut count = 0;
    let result = 'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        if count == 3 {
            break -1337;
        }

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 4 {
                break 'counting_up -128;
            }
            remaining -= 1;
        }

        count += 1;
    };
    println!("End count = {count}");
    println!("End result = {result}");
}