fn main() {
    let son1 = Son1{
        name: "Kiva",
        age: 20,
    };
    notify(&son1,100);
}

/*
 使用特征作为默认参数
 */
trait Daddy<T: ToString>{
    fn pay(&self,amount: T) -> (){
        println!("{:?}",amount.to_string())
    }
}

// 显式为Son1实现
impl<T: std::fmt::Display> Daddy<T> for Son1 {}
struct Son1{
    name: &'static str,
    age: u8,
}

// 为参数item实现特征，就可以使用Daddy所有方法
fn notify(item: &impl Daddy<i32> ,amount: i32) -> (){
    item.pay(amount)
}
