mod arrayGet;

use std::io;


fn main() {

    let a = [1,2,3,4,5];

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Fail to read line");

    let index: usize = index.trim()
        .parse()
        .expect("Index entered was not a number");

    let elem = a[index];
    println!("The value of the elem at index {index} is : {elem}")
}
