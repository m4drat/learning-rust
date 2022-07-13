use rectangles::count as count_rectangles;

fn main() {
    let lines = &[
    //             10
    //    1 3 5 7 9|11 
    //   0 2 4 6 8 ||12
        "+------+----+", // 0
        "|      |    |", // 1
        "+------+    |", // 2
        "|   |       |", // 3
        "+---+-------+", // 4
    ];
    println!("Total rectangles: {}", count_rectangles(lines));
}