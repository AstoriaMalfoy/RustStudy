
fn main() {
    let str = String::from("this is a demo string");
    let index = first_word(&str);
    str.clear();
    println!("{index}");
}

fn first_word(str:&String)->&str{
    let bytes = str.as_bytes();
    for (i,&item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &str[..i];
        }
    }
    &str[..]
}
