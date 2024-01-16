fn main() {
    let s1 = get_string();

    let s2 = String::from("yours");

    let s3 = take_and_give_back(s2);

}

fn get_string()->String{
    String::from("hello world")
}

fn take_and_give_back(a_string:String)->String{
    a_string
}
