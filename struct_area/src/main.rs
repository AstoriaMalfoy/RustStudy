
#[derive(Debug)]
struct Rectangle{
    widht:i32,
    heihgt:i32,
}

fn main() {
    let width = 30;
    let height = 30;
    // println!("the area of the rectangle is {} square pixels",area(width, height));


    let rec1 = (30,30);
    // println!("the area of the rectangle is {} square pixels",area_v1(rec1));

    let rec2 = Rectangle{
        widht:30,
        heihgt:30,
    };
    println!("rec2 is {:?}",rec2);
    // println!("the area of the rectangle is {} square pixels",area_v2(rec2));
}


fn area(width:i32,height:i32)->i32{
    width * height
}

fn area_v1(rec:(u32,u32))->u32{
    rec.0 * rec.1
}

fn area_v2(rec:Rectangle)->i32{
    rec.widht * rec.heihgt
}
