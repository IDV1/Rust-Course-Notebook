fn main() {
    // 载入数据
    let son1 = Son1{
        name: "Dick",
        age: 25,
        give_daddy_fucking_shit_money: "55".to_string()
    };
    
    // 调用实现成功的give_money方法
    println!("{}", son1.give_money());

    // 载入数据
    let son2 = Son2{
        name: "Peter",
        age: 55,
        give_daddy_fucking_shit_money: "1000".to_string()
    };

    // 调用实现成功的give_money方法
    println!("{}", son2.give_money());
}

// 定义特征Daddy
pub trait Daddy{
    // 未实现
    fn give_money(&self) -> String;
}

struct Son1<T>{
    name: &'static str,
    age: u8,
    give_daddy_fucking_shit_money: T
}

struct Son2<T>{
    name: &'static str,
    age: u8,
    give_daddy_fucking_shit_money: T
}

// 为Son1实现daddy
impl Daddy for Son1<String> {
    // 被son1实现
    fn give_money(&self) -> String {
        format!("{} was {} who gave his daddy {} dollar money",self.name,self.age,self.give_daddy_fucking_shit_money)
    }
}

// 为Son2实现daddy
impl Daddy for Son2<String> {
    // 被son2实现
    fn give_money(&self) -> String {
        format!("{} was {} who gave his daddy {} dollar money",self.name,self.age,self.give_daddy_fucking_shit_money)
    }
}

