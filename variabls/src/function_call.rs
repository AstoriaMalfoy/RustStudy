use std::io;

fn main() {
    print_number_function();
}

fn print_number_function(){
    println!("hello world");
    let mut index = String::new();
    println!("please input number");
    io::stdin()
        .read_line(&mut index)
        .expect("get value error");
    let index = index.trim()
        .parse()
        .expect("input is not a number");
    print_number(index);
}

fn print_number(x:i32){
    println!("{x}");
}
