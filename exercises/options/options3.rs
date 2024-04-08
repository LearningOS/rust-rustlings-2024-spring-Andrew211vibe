// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.



struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    // method 1
    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }

    // method 2
    // if let Some(Point{x, y}) = y {
    //     println!("Co-ordinates are {},{} ", x, y);
    // } else {
    //     panic!("no match!");
    // }

    y; // Fix without deleting this line.
}
