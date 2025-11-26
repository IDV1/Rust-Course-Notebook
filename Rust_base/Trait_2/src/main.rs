fn main() {
    let son1 = Son1{name: "James",age:24};
    println!("{}",son1.pay(5000));
}

trait Daddy<T>{
    // 默认实现
    fn pay(&self,amount: T) -> String{
        "1000".to_string()
    }
}

struct Son1{
    name: &'static str,
    age: u8,
}

impl<T: ToString> Daddy<T> for Son1{
    // 重写方法
    fn pay(&self,amount: T) -> String{
        format!("when {} {} paid {} for his daddy",self.name,self.age,amount.to_string())
    }
}