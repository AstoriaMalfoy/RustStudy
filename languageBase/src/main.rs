
mod function_run;

fn main() {
    let x = get_value();
    println!("the value of x :{x}");
    let x = add_value(x);
    println!("the value of x :{x}");
}

fn add_value(x:i32) -> i32{
    return x + 1;
}

fn get_value()->i32{
    5
}
