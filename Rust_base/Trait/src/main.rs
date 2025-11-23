fn main() {
    let son1 = Son1{
        name: "Dick",
        age: 25,
        give_daddy_fucking_shit_money: "55".to_string()
    };
    println!("{}", son1.give_money());

    let son2 = Son2{
        name: "Peter",
        age: 55,
        give_daddy_fucking_shit_money: "1000".to_string()
    };
    println!("{}", son2.give_money());
}

pub trait Daddy{
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

impl Daddy for Son1<String> {
    fn give_money(&self) -> String {
        format!("{} was {} who gave his daddy {} dollar money",self.name,self.age,self.give_daddy_fucking_shit_money)
    }
}

impl Daddy for Son2<String> {
    fn give_money(&self) -> String {
        format!("{} was {} who gave his daddy {} dollar money",self.name,self.age,self.give_daddy_fucking_shit_money)
    }
}

