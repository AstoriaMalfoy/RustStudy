
struct User{
    active:bool,
    username:String,
    email:String,
    sing_in_count : u64,
}


fn build_user(email:String,username:String)->User{
    User {
        active:true,
        username:username,
        email:email,
        sing_in_count:1,
    }
}


fn build_user_short(email:String,username:String)->User{
    User {
        active:true,
        username,
        email,
        sing_in_count:1,
    }
}

fn copy_user(){

    let user = User{
        active:true,
        username:String::from("someusername"),
        email:String::from("someemail@example.com"),
        sing_in_count:1,
    };


    let user2 = User{
        active:user.active,
        username:user.username,
        email:user.email,
        sing_in_count:user.sing_in_count,
    };

}


fn copy_user_v2(){

    let user = User{
        active:true,
        username:String::from("someusername"),
        email:String::from("someemail@example.com"),
        sing_in_count:1,
    };


    let user2 = User{
        username:String::from("user name"),
        ..user
    };
}

struct Color(i32,i32,i32);
struct Point(i32,i32,i32);

// fn main() {
//     let black= Color(0,0,0);
//     let origin = Point(0,0,0);
// }
//
//
//
//

struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
