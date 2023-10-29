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

    match y {
        Some(ref p) => {
            let p = p; // Bind 'Some(p)' to a new variable 'p'
            println!("Co-ordinates are {},{} ", p.x, p.y);
        }
        _ => panic!("no match!"),
    }
    y;
    // Now you can use 'y' without any issues.
    // Do something with 'y' here.
}