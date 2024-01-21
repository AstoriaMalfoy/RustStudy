
// 方法定义
#[derive(Debug)]
struct Rectange{
    width:u32,
    height:u32,
}

impl Rectange {
    fn area(&self) -> u32{
        self.width * self.height
    }
}

impl Rectange {
    fn can_conver(&self,other:&Rectange)->bool{
        self.width >= other.width && self.height >= other.height
    }
}

impl Rectange {
    fn init()->Rectange{
        Rectange{
            width:20,
            height:30,
        }
    }

    fn new(width:u32,height:u32)->Rectange{
        Rectange{
            width,
            height,
        }
    }
}

fn new_rectange(){
    let rec1 = Rectange::init();
    let rec2 = Rectange::new(10, 20);
    dbg!(rec1);
    dbg!(rec2);
}

fn main(){
    new_rectange();
}

fn method_use() {
    let rec = Rectange{
        width:100,
        height:30,
    };
    let area = rec.area();
    println!("the area of rec is :{area}");
}

