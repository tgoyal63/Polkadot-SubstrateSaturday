// Continue will skip over the remaining code in current iteration and go to the next iteration
//
// Fill in the blanks
/*fn main() {
    let mut n = 0;
    for i in 0..=100 {
       if n != 66 {
           n+=1;
           __;
       }

       __
    }

    assert_eq!(n, 66);

    println!("Success!");
}*/

// My Solution

// Fill in the blanks
fn main() {
    let mut n = 0;
    for i in 0..=100 {
       if n != 66 {
           n+=1;
           continue; // Going to next iteration
       }
       
       break; //Breaking
    }

    assert_eq!(n, 66);

    println!("Success!");
}

