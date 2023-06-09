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
    // 引用
    reference();
    // slice类型
    slice();
}

fn slice() {
    // 获取字符串的第一个空格
    let demo_str = String::from("this is a test str");
    let str_len = get_str_first_space(&demo_str);
    println!("the len of str is {str_len}");
    get_first_word(&demo_str);
}


fn get_first_word(str: &String) -> &str{
    let bytes = str.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &str[0..item];
        }
    }
    return &str[0..str.len()];
}

// 获取字符串的第一个空个的位置，如果获取不到，就直接返回字符串的长度
fn get_str_first_space(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    return s.len();
}

fn reference() {
    // 在调用函数的时候，控制权会被转移，如果要想在函数结束依旧使用这个变量，就需要将参数值返回，实际上有更好的方案解决这个问题，就是使用引用。
    param_reference();
    // 可变引用
    param_reference_with_change();
}


fn param_reference_with_change() {
    // 因为在引用中不能对变量进行更改，所以提出了可变引用，以便在引用过程中给也能对变量的值进行更改
    let mut demo_str = String::from("test value");
    let length = get_str_size_and_append(&mut demo_str, String::from(" test"));
    println!("the value of string is :{demo_str} , length of str is :{length}");
    // 但是需要注意的时，同一时间，一个变量只能存在一个可变引用，不能同时有多个可变引用出现，这样做的目的时方式出现数据竞争。
    // 并且在使用过程中，可变引用和不可变引用也不能同时出现
    // 这里所谓的同时出现，并不严格限制同一个作用域 例如 如下的代码也是可以使用的 这里的原因是因为ref_a和ref_b在打印之后，就没有在继续使用了，这个时候可变引用对变量造成的修改无法影响到这两个引用
    let mut test_variable = String::from("test String");
    let ref_a = &test_variable;
    let ref_b = &test_variable;
    println!("the ref_a is {ref_a} ,  the ref_b is {ref_b}");
    let mod_ref = &mut test_variable;
    print!("the mod_ref is {mod_ref}");
    // 这里我们可以理解成，引用是一个S锁，而可变引用是一个X锁
}

// 可变参数引用
fn get_str_size_and_append(str: &mut String, append_str: String) -> usize {
    println!("the input str is :{str}");
    let size = str.len();
    str.push_str(&append_str);
    size
}

// 参数引用
fn param_reference() {
    let demo_str = String::from("teset value");
    let length = get_str_len(&demo_str);
    // 引用的方法中是不能对参数进行修改的，如果对参数修改后导致编译报错 如果需要在函数中对参数进行修改 需要使用可变引用
    println!("the value of demoStr is :{demo_str} the length of demoStr is :{length}");
}

fn get_str_len(str: &String) -> (usize) {
    str.len()
}


// 所有权
fn onwerShip() {
    println!("***************************** the onwerShip part *****************************");
    // 所有权是一组规则，用来指导RUST语言的内存管理，在JAVA中，由虚拟机在运行时候按照引用来寻找垃圾空间，而在C语言中，内存的合理分配和销毁由开发者控制，
    // 在RUST中，内存是基于一套所所有权系统和编译规则来实现的，如果在编译的时候违反了任何的规则，在编译的时候都将无法通过。
    string_demo();
    // onwerShipRemove
    onwerShipRemove();
    // clone 
    clone();
    // stack not wonershipRemove
    stackOwnerShip();
    // 调用函数过程中的所有权转移
    ownerRemoveOnFunction();
    // 调用函数保留所有权，同时返回参数
    return_value_with_owner_remove();
}

// 参数调用的时候保留变量所有权并且允许返回值，可以使用元祖的方式来获取返回值
fn return_value_with_owner_remove() {
    let demo_str = String::from("test string");
    let (demo_str, return_value) = get_return_value_and_ovnership(demo_str);
    println!("the vlaue of str is :{demo_str}, the value of return value is :{return_value}")
}

// 获取返回值的同时保留变量的所有权
fn get_return_value_and_ovnership(p_str: String) -> (String, i32) {
    println!("the value of string is :{p_str}");
    (p_str, 23)
}

fn ownerRemoveOnFunction() {
    let str = String::from("this is test value");

    printString(str);

    // 如果这个时候使用变量str会导致变异错误，因为调用函数时候，向形参赋值的逻辑和向引用赋值一样，会导致所有权转移

    let number = 1;

    printI32(number);

    println!("the value of number is :{number}");

    let demoStr = give_ownerShip();

    let demoStr2 = String::from("antoher test str");

    // 在调用一个函数的时候，会导致所有权转移，如果想要重新拿回所有权，需要将原来的值作为结果返回
    let demoStr3 = get_and_back_ownerShip(demoStr2);
}


// 给出变量控制权
fn give_ownerShip() -> String {
    let str = String::from("this is test string");
    str
}

// 获得并给出变量控制权
fn get_and_back_ownerShip(str: String) -> String {
    print!("the value of input String is :{str}");
    return str;
}

fn printI32(number: i32) {
    println!("the value of number is :{number}");
}

fn printString(str: String) {
    println!("the value of str is :{str}");
}

// 栈中元素没有所有权转移
fn stackOwnerShip() {
    // 下面的代码也可以正常的运行，似乎和之前的所说的所有权转移冲突，但是实际上是不冲突的，因为数字类型的是直接分配在堆栈中的，堆栈中的数据不涉及深拷贝和浅拷贝
    // 所以RUST不会让numberA失效。
    // 在RUST中还有一个Copy特性，使用了这个注解的数据类型会被分配在堆栈中，在赋值的时候，会进行简单的复制，而不是移动。所以复制不会导致原有对象失效。

    let numberA = 1;
    let numberB = numberA;
    println!("the value of numberA is :{numberA} , the number of numberB is :{numberB}");
}

// clone 函数
fn clone() {
    let s1 = String::from("test string");
    let s2 = s1.clone();
    println!("the value of s1 is :{s1} , of s2 is : {s2}")
}

// 所有权转移
fn onwerShipRemove() {
    // 在RUST语言中，当一个变量的作用域结束之后，会自动调用该变量的drop函数，当两个引用指向同一个变量的时候，作用域结束的时候，两个引用都会调用drop函数，
    // 这种称为双重释放错误，是严重的内存安全错误，所以当两个引用指向同一个变量的时候，RUST就会默认第一个变量已经失效，此刻，针对于该变量已经无法被使用
    let s1 = String::from("test String");
    let s2 = s1;
    // 在RUST中，复制也只是浅复制，实际上在堆上的内存对象并没有发生修改。如果我们想要进行深拷贝，那么可以使用clone函数
    println!("the value of s1 is :{s2}");
}

fn string_demo() {
    let mut str = String::from("test string");
    str.push_str(" append subStr");
    println!("the value of str is :{str}");
}

// 流程控制
fn control() {
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


fn traverse_inex_set() {
    for elem in (1..4).rev() {
        println!("{elem}!");
    }
}

fn traverse_list_by_for() {
    let tempList = [123, 14, 321, 5, 3132, 4123, 4];
    for elem in tempList {
        println!("current value is :{elem}");
    }
}

fn traverse_list_by_while() {
    let tempList = [1, 2, 2, 45, 7];
    let mut index = 0;
    while index < 5 {
        let currentPrint = tempList[index];
        println!("the value of current index :{index} is :{currentPrint}");
        index += 1;
    }
}

fn while_loop() {
    let mut count = 3;
    while count > 0 {
        println!("the count is :{count}");
        count -= 1;
    }
}

// 消除多层循环跳转的歧义
fn jump_tag() {
    let mut count = 0;
    // 定义循环tag需要使用一个单引号来定义，接在continue或者break之后使用
    'outerloop: loop {
        println!("the value of count is :{count}");
        let mut innerValue = 10;
        loop {
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
fn value_loop() {
    let mut count = 0;
    let result = loop {
        if count < 10 {
            count = count + 1;
        } else {
            // 如果表达式语句块的最后一条语句是跳转控制，可以使用如下的方式来返回返回值
            break count * 2;
        }
    };

    println!("the value of result is :{result}");
}

// loop循环
fn base_loop() {
    let mut print_count = 0;
    loop {
        if print_count < 5 {
            print_count = print_count + 1;
        } else {
            break;
        }
        println!("loop again : {print_count}");
    }
}

// if 表达式
fn if_as_expression() {
    let num = 1;
    // 如果使用if的结果作为表达式的结果，那么需要确保if的所有分支返回的类型必须相同，否则会在编译的时候报错
    let y = if num > 0 { 1 } else { 0 };
    println!("the value of y is :{y}");
}

// 多条件控制
fn many_condition_control() {
    let number = 10;
    if number < 3 {
        println!("the value is less than 3");
    } else if number < 5 {
        println!("the value is less then 5");
    } else if number < 50 {
        println!("the value is less than 50");
    } else {
        println!("the value is out of range");
    }
}

// 基本的IF
fn if_control() {
    let number = 3;
    // if 判断的条件必须是bool,并且Number类型无法转换为bool类型
    // RUST 不会自动隐式的将其他类型转换为Bool类型
    if number >= 5 {
        println!("the value is above five");
    } else {
        println!("the value is less than five");
    }
}

// 函数
fn function() {
    // RUST中的函数通常采用蛇行小写的形式，即所有的字母都是小写的，单词和单之间使用下划线分隔
    println!("***************************** the function part *****************************");
    // 函数的调用和函数声明的位置无关，只要函数的定义范围大于当前作用域，就能调用到
    print_hello_world();
    // 在函数中 parameter--形参，argument-实参
    print_the_argument(12);

    print_the_many_argument(23, 'c');
    // 语句和表达式
    statements_and_expressions();

    let value = five();
    println!("the value is :{value}");

    let value = plus_one(value);
    println!("the value of plus one is :{value}");
}

// 加一
fn plus_one(value: i32) -> i32 {
    value + 1
}

// 带有返回值的函数
fn five() -> i32 {
    5
}

// 语句和表达式
fn statements_and_expressions() {
    // 语句是执行某些操作，但是不返回值的指令
    // 表达式则是计算结果值
    let x = 6;      // 在Rust中，赋值语句不是表达式，不能连续使用 不能使用类似如下的代码 ： let x = y = 6;
    // 在RUST中，计算结果是一个表达式，函数调用是一个表达式，宏调用是一个表达式，定义一个语句块是一个表达式
    let y = {
        let x = 1;
        // 这里定义了一个语句块，要想让语句块可以作为一个表达式，需要满足两个条件，一个是语句块最后一条语句需要是表达式，并且这个表达式不能以分号结尾，如果添加了分号，就变成了语句，而不是表达式
        x + 1
    };
    println!("the value of y is :{y}");
}

// 简单函数定义
fn print_hello_world() {
    println!("hello world!");
}

// 包含一个参数的的函数
fn print_the_argument(x: i32) {
    println!("the function argument is :{x}");
}

// 包含两个参数的函数
fn print_the_many_argument(x: i32, y: char) {
    println!("the function arguments is x:{x},y:{y}");
}

// 数据类型
fn dataStructor() {
    println!("***************************** the data struct part *****************************");

    // 整数类型
    let int8: i8 = 2;      // 8位有符号整形
    let uint8: u8 = 3;     // 8位无符号整型
    let int16: i16 = 12;   // 16位有符号整形
    let uint16: u16 = 13;  // 16位无符号整型
    let int32: i32 = 32;     // 32位有符号整型
    let uInt32: u32 = 33;    // 32位无符号整型
    let int64: i64 = 63;     // 64位有符号整型
    let uint64: i64 = 76;    // 64位无符号整型
    let intN: isize = 123;   // 有符号整型，位数取决于操作系统
    let uIntN: usize = 1423; // 无符号整型，位数取决于操作系统

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
    let boolValueT: bool = true;
    let boolValueF = false;
    println!("the default value of bool is {boolValueT},{boolValueF}");


    // 字符类型 字符类型中的一个字符是在unioncode编码下的一个字符， 总共四个字节。
    let c = 'z';
    let z: char = 'z';
    let emoji = '😀';

    println!("the value of char is :{c},{z},{emoji}");


    // 元组 元组中的内容类型可以不相同，元组的长度一旦确定下来之后就不能在变化了，如果要获取元组中的内容，可以使用映射的方式来实现。
    let tup: (i32, f32, char) = (12, 12.0, 'c');
    // 将元组解构为三个变量
    let (x, y, z) = tup;
    println!("the value of x is :{x}");
    // 也可以直接寻秩访问
    let one = tup.1;
    println!("the vlaue of .1 is :{one}");


    // 数组 变量名:[(类型/初始化值);长度] = [1,2,3,....]
    let list: [i32; 5] = [1, 2, 3, 4, 5];
    // 数组的长度一旦确定之后就不能在修改，所以当确定使用元素数量不会发变化的时候再使用，否则建议使用向量。
    // 如果是定义期间的数组越界会被编译器了拦截，但是如果在运行时期的数组越界会被抛出异常。
    let firsetElem = list[0];
    println!("the first value of list is :{firsetElem}");

    let test_str = "tsetStr";
    println!("the value of test_str is:{test_str}");

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
fn shadowing() {

    // 在Rust中使用 let 关键字定义变量，但是变量定义好之后默认是不允许修改的
    // 如果想要定义可以被修改的变量，需要再组合关键字 mut  eg. let mut var = 1;

    println!("***************************** the var part *****************************");

    // 定义常量
    const HOUR_OF_SECOND: u32 = 60 * 60 * 60;
    println!("the constatn value is :{HOUR_OF_SECOND}");

    // 阴影变量指的是，定义好一个变量之后，再次定义一个同名的变量，新变量会覆盖历史变量，直达新变量的作用域结束或者新变量再次被阴影变量覆盖。
    // 变量阴影 
    let x = 1;
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


    let mut y = y + 1;
    println!("the value of y is :{y}");
    y = y + 1;
    println!("the value of same y is :{y}");


    // 使用影子变量的一个好处是 如果声明了一个影子变量，就相当于定义了一个新的变量，不仅仅可以进行同类型的数据赋值，也可以使用不同类型的值赋值。这一点mut就做不到。
    let str = "this is a demo String";
    println!("the string of init is :{str}");
    let str = str.len();
    println!("the string of length is :{str}");
}

