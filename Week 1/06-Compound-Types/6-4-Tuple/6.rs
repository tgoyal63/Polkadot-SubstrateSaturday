// Tuples can be used as functions arguments and return values
//
// Given
//
/*fn main() {
    // Fill the blank, need a few computations here.
    let (x, y) = sum_multiply(__);

    assert_eq!(x, 5);
    assert_eq!(y, 6);

    println!("Success!");
}

fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
    (nums.0 + nums.1, nums.0 * nums.1)
}*/

// My Solution
fn main() {
    let (x, y) = sum_multiply((2, 3)); // Add tuple for function

    assert_eq!(x, 5);
    assert_eq!(y, 6);

    println!("Success!");
}

fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
    (nums.0 + nums.1, nums.0 * nums.1)
}
