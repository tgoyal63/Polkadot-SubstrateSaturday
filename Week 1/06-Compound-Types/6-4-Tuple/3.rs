// Long tuples can be printed
//
// Given
//
// Fix the error
/*fn main() {
    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    println!("too long tuple: {:?}", too_long_tuple);
}*/

// My Solution
fn main() {
    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12); // Remove one element to allow it to print
    println!("too long tuple: {:?}", too_long_tuple);
}
