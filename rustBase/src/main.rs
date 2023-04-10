
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

    println!("the value is :{int8},{uint8},{int16},{uint16},{int32},{uInt32},{int64},{uint64},{intN},{uIntN},{value10},{value16},{value8},{value2},{charVlaue}");

}