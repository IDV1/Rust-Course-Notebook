fn main() {
    let num = 100_i32;
    transfor_(Box::new(num));
}

trait Write{
    fn write(&self);
}

// 只有实现了i32才能使用Write特征
impl Write for i32{
    fn write(&self) {
        println!("{} 是一个被所有权String转换的i32位", self.to_string());
    }
}

impl Write for u8 {
    fn write(&self) {
        println!("{} 是一个被所有权String转换的u8位", self.to_string());
    }
}

// 这里的参数是dyn它代表trait特征类型，告诉编译器是特征类型
fn transfor_(x: Box<dyn Write>) {
     x.write();
}