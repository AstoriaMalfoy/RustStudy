use std::io;


fn main() {
    //变量定义
    shadowing();
    // 数据类型
    dataStructor();
    // 函数
    function();
    // 顺序控制
    control();
    // 所有权
    onwerShip();
}
// 所有权
fn onwerShip(){
    // 所有权是一组规则，用来指导RUST语言的内存管理，在JAVA中，由虚拟机在运行时候按照引用来寻找垃圾空间，而在C语言中，内存的合理分配和销毁由开发者控制，
    // 在RUST中，内存是基于一套所所有权系统和编译规则来实现的，如果在编译的时候违反了任何的规则，在编译的时候都将无法通过。
    
}

// 流程控制
fn control(){
    println!("***************************** the control part *****************************");
    // 基本的IF
    if_control();
    // 多条件控制
    many_condition_control();
    // 使用IF作为表达式
    if_as_expression();

    // loop 循环
    base_loop();
    // loop with return value
    value_loop();
    // 带有tag的loop
    jump_tag();

    // while循环
    while_loop();

    // 使用while遍历数据
    traverse_list_by_while();

    // 使用for遍历数组
    traverse_list_by_for();

    // 遍历index
    traverse_inex_set();

}


fn traverse_inex_set(){
    for elem in (1..4).rev() {
        println!("{elem}!");
    }
}

fn traverse_list_by_for(){
    let tempList = [123,14,321,5,3132,4123,4];
    for elem in tempList  {
        println!("current value is :{elem}");
    }
}

fn traverse_list_by_while(){
    let tempList = [1,2,2,45,7];
    let mut index = 0;
    while index < 5{
        let currentPrint = tempList[index];
        println!("the value of current index :{index} is :{currentPrint}");
        index += 1;
    }
}

fn while_loop(){
    let mut count = 3;
    while count > 0{
        println!("the count is :{count}");
        count -= 1;
    }
}

// 消除多层循环跳转的歧义
fn jump_tag(){
    let mut count = 0;
    // 定义循环tag需要使用一个单引号来定义，接在continue或者break之后使用
    'outerloop : loop{
        println!("the value of count is :{count}");
        let mut innerValue = 10;
        loop{
            println!("the value of inner value is :{innerValue}");
            if innerValue == 9 {
                break;
            }
            if count == 2 {
                break 'outerloop;
            }
            innerValue -= 1;
        }
        count += 1;
    }
    println!("the end of count is :{count}");
}

// 代表表达式的循环条件
fn value_loop(){
    let mut count = 0;
    let result = loop{
        if count < 10 {
            count = count + 1;
        }else{
            // 如果表达式语句块的最后一条语句是跳转控制，可以使用如下的方式来返回返回值
            break count * 2
        }
    };

    println!("the value of result is :{result}");
}

// loop循环
fn base_loop(){
    let mut print_count = 0;
    loop{
        if print_count < 5 {
            print_count = print_count + 1;
        }else{
            break;
        }
        println!("loop again : {print_count}");
    }
}

// if 表达式
fn if_as_expression(){
    let num = 1;
    // 如果使用if的结果作为表达式的结果，那么需要确保if的所有分支返回的类型必须相同，否则会在编译的时候报错
    let y = if num > 0 {1} else {0};
    println!("the value of y is :{y}");
}

// 多条件控制
fn many_condition_control(){
    let number = 10;
    if number < 3 {
        println!("the value is less than 3");
    }else if number < 5{
        println!("the value is less then 5");
    }else if number < 50{
        println!("the value is less than 50");
    }else{
        println!("the value is out of range");
    }
}

// 基本的IF
fn if_control(){
    let number = 3;
    // if 判断的条件必须是bool,并且Number类型无法转换为bool类型
    // RUST 不会自动隐式的将其他类型转换为Bool类型
    if number >= 5 {
        println!("the value is above five");
    }else{
        println!("the value is less than five");
    }
}

// 函数
fn function(){
    // RUST中的函数通常采用蛇行小写的形式，即所有的字母都是小写的，单词和单之间使用下划线分隔
    println!("***************************** the function part *****************************");
    // 函数的调用和函数声明的位置无关，只要函数的定义范围大于当前作用域，就能调用到
    print_hello_world();
    // 在函数中 parameter--形参，argument-实参
    print_the_argument(12);

    print_the_many_argument(23,'c');
    // 语句和表达式
    statements_and_expressions();

    let value = five();
    println!("the value is :{value}");

    let value = plus_one(value);
    println!("the value of plus one is :{value}");

}

// 加一
fn plus_one(value:i32)->i32{
    value + 1
}

// 带有返回值的函数
fn five() -> i32{ 
    5
}

// 语句和表达式
fn statements_and_expressions(){
    // 语句是执行某些操作，但是不返回值的指令
    // 表达式则是计算结果值
    let x = 6;      // 在Rust中，赋值语句不是表达式，不能连续使用 不能使用类似如下的代码 ： let x = y = 6;
    // 在RUST中，计算结果是一个表达式，函数调用是一个表达式，宏调用是一个表达式，定义一个语句块是一个表达式
    let y ={
        let x = 1;
        // 这里定义了一个语句块，要想让语句块可以作为一个表达式，需要满足两个条件，一个是语句块最后一条语句需要是表达式，并且这个表达式不能以分号结尾，如果添加了分号，就变成了语句，而不是表达式
        x +  1
    };
    println!("the value of y is :{y}");
}

// 简单函数定义
fn print_hello_world(){
    println!("hello world!");
}

// 包含一个参数的的函数
fn print_the_argument(x:i32){
    println!("the function argument is :{x}");
}

// 包含两个参数的函数
fn print_the_many_argument(x:i32 , y:char){
    println!("the function arguments is x:{x},y:{y}");
}

// 数据类型
fn dataStructor(){

    println!("***************************** the data struct part *****************************");

    // 整数类型
    let int8 : i8 = 2;      // 8位有符号整形
    let uint8 : u8 = 3;     // 8位无符号整型
    let int16 : i16 = 12;   // 16位有符号整形
    let uint16 : u16 = 13;  // 16位无符号整型
    let int32:i32 = 32;     // 32位有符号整型
    let uInt32:u32 = 33;    // 32位无符号整型
    let int64:i64 = 63;     // 64位有符号整型
    let uint64:i64 = 76;    // 64位无符号整型
    let intN:isize = 123;   // 有符号整型，位数取决于操作系统
    let uIntN:usize = 1423; // 无符号整型，位数取决于操作系统

    let value10 = 1231;    // 十进制
    let value16 = 0x123;        // 十六进制
    let value8 = 0o123;              // 八进制
    let value2 = 0b10101;           // 二进制
    let charVlaue = b'C';               //字节

    println!("the int values is :{int8},{uint8},{int16},{uint16},{int32},{uInt32},{int64},{uint64},{intN},{uIntN},{value10},{value16},{value8},{value2},{charVlaue}");


    // 浮点类型 在指定小数的时候，如果没有指明类型，默认是双精度浮点数
    let doubleVlaue = 0.4;
    let floatValue: f32 = 0.6;

    println!("the float value is {doubleVlaue},{floatValue}");


    // 布尔类型
    let boolValueT:bool = true;
    let boolValueF = false;
    println!("the default value of bool is {boolValueT},{boolValueF}");


    // 字符类型 字符类型中的一个字符是在unioncode编码下的一个字符， 总共四个字节。
    let c = 'z';
    let z:char = 'z';
    let emoji = '😀';

    println!("the value of char is :{c},{z},{emoji}");


    // 元组 元组中的内容类型可以不相同，元组的长度一旦确定下来之后就不能在变化了，如果要获取元组中的内容，可以使用映射的方式来实现。
    let tup :(i32,f32,char) = (12,12.0,'c');
    // 将元组解构为三个变量
    let (x,y,z) = tup;
    println!("the value of x is :{x}");
    // 也可以直接寻秩访问
    let one = tup.1;
    println!("the vlaue of .1 is :{one}");


    // 数组 变量名:[(类型/初始化值);长度] = [1,2,3,....]
    let list:[i32;5] = [1,2,3,4,5];
    // 数组的长度一旦确定之后就不能在修改，所以当确定使用元素数量不会发变化的时候再使用，否则建议使用向量。
    // 如果是定义期间的数组越界会被编译器了拦截，但是如果在运行时期的数组越界会被抛出异常。
    let firsetElem = list[0];
    println!("the first value of list is :{firsetElem}");

    // 运行时数组越绝会抛出异常
    // let mut inputValue = String::new();

    // println!("Please input the value of list index:");

    // io::stdin()
    //     .read_line(&mut inputValue)
    //     .expect("input the wrong value");

    // let inputIndex:usize =  inputValue
    //     .trim()
    //     .parse()
    //     .expect("input value is not a number");
    
    // let sanncerIndexValue =  list[inputIndex];
    // println!("the value of input index is :{sanncerIndexValue}");

}

// shardowing变量
fn shadowing(){


    println!("***************************** the var part *****************************");

    // 定义常量
    const HOUR_OF_SECOND : u32 = 60 * 60 * 60;
    println!("the constatn value is :{HOUR_OF_SECOND}");

    // 变量阴影 
    let x  = 1;
    println!("the value of x is:{x}");

    let x = x + 1;
    println!("the value of x is :{x}");

    {
        // 在作用域对变量进行修改的时候，修改也只会在特定的作用域内有效
        let x = x * 2;
        println!("the value of x is :{x}");
    }

    println!("the value of x is :{x}");


    // 如果定义的变使用 mut 进行修饰，同样可以使用影子变量。但是需要注意的，如果定义的是变量，那么必须要有修改，修改也必须要有使用。
    let mut y = 1;
    println!("the value of y is :{y}");
    y = y + 1;
    println!("the value of same y is :{y}");

    
    let mut y = y  + 1;
    println!("the value of y is :{y}");
    y =  y + 1;
    println!("the value of same y is :{y}");


    // 使用影子变量的一个好处是 如果声明了一个影子变量，就相当于定义了一个新的变量，不仅仅可以进行同类型的数据赋值，也可以使用不同类型的值赋值。这一点mut就做不到。
    let str = "this is a demo String";
    println!("the string of init is :{str}");
    let str = str.len();
    println!("the string of length is :{str}");  
}

