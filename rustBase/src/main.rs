use std::io;


fn main() {
  shadowing();

  dataStructor();
}

// shardowing变量
fn shadowing(){
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

// 数据类型
fn dataStructor(){
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

    let mut inputValue = String::new();

    io::stdin()
        .read_line(&mut inputValue)
        .expect("input the wrong value");

    let inputIndex:usize =  inputValue
        .trim()
        .parse()
        .expect("input value is not a number");
    
    let sanncerIndexValue =  list[inputIndex];
    println!("the value of input index is :{sanncerIndexValue}");



}