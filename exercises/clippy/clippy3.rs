// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
// No hints.


#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    if my_option.is_none() {
        // Instead of calling unwrap(), you can simply do nothing
    }

    let my_arr = &[
        -1, -2, -3,  // Add a comma here to separate elements
        -4, -5, -6,
    ];
    println!("My array! Here it is: {:?}", my_arr);

    // Create an empty Vec directly
    let my_empty_vec: Vec<i32> = vec![];
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;

    // Swap value_a and value_b using std::mem::replace
    std::mem::swap(&mut value_a, &mut value_b);

    println!("value a: {}; value b: {}", value_a, value_b);
}


