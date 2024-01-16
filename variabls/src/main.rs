use std::usize;
use std::io;

mod function_call;
use self::function_call::*;

fn main() {
    variable_mut();
    rust_constant();
    type_infer();
    println!("Hello, world!");
}

/**
 *  part0 rust变量可变性
 * 使用let关键字声明变量，但是声明的变量是不可变的。
 * 如果想要变量可变，需要同时使用mut关键字进行修饰
 */
fn variable_mut(){

    let var_a = 10;

  // 下面代码在使用cargo run命令运行的时候就报错
  //  var_a = 20;
  //  ^^^^^^^^^^ cannot assign twice to immutable variable
    let mut var_b = 10;
    var_b = 20;
}


/**
 * 在rust中可以定义常量，使用关键字const
 * 通常情况下，常量全部使用大写字母表示，并且必须指定类型
 */
fn rust_constant(){
    const MAX_LENGTH : usize = 100_000;
}

/**
* 强制指定类型：当类型推断能推断出多种类型的时候，就必须现示指定类型
*/
fn type_infer(){
    let str = "42";
    let number : usize = str.parse().expect("the string str is not a number");
}

fn read_line(){
}
