fn main() {
    // test_if();
    // test_loop_return();
    // test_loop_tag();
    // while_demo();
    for_demo();
}


fn for_demo(){
    let students = [3;5];
    for student in students {
        println!("the value of student is {student}");
    }
}

fn while_demo(){
    let mut count = 10;
    while count != 0 {
        println!("the value of count is {count}");
        count -= 1;
    }
}

fn test_if(){
    let number = 3;
    if number > 0 {
        println!("the number is above than 0");
    }else{
        println!("the number is less than 0")
    }
}

fn test_loop_return (){
    let mut count = 0;
    let result = loop {
        count = count + 1;
        if count == 10 {
            break count * 2
        }
    };
    println!("the value of result is :{result}");
}

fn test_loop_tag(){
    let mut count = 0;
    'counting_up : loop {
        println!("the value of count is {count}");
        let mut remaining = 10;


        loop {
            println!("the value of remaining is {remaining}");
            if remaining == 0 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count  = {count}");
}

