use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("please input a number");

    let rng_number = rand::thread_rng().gen_range(1,100);

    // 无限循环
    loop {
        println!("generate number is:{}",rng_number);
        // 生成一个字符串
        let mut input_value = String::new();
        
        io::stdin().read_line(&mut input_value).expect("input method return error");

        let input_value : u32 = match input_value.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, 
        }; 

        println!("the input value is :{}",input_value);

        match input_value.cmp(&rng_number) {
            Ordering::Less => println!("inpu too smale"),
            Ordering::Greater => println!("input too big"),
            Ordering::Equal => {
                println!("you win");
                break;
            }, 
        }
    }
}
