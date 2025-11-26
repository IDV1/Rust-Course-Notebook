use std::fmt::{Debug, Display};

fn main() {
    let james = James{
        name: "James".to_string(),
        age: 20,
    };

    let mario = Mario{
        name: "Marios".to_string(),
        age: 20,
    };

    // notify(&james,&mario); 不能用两个不一样的结构体
    notify(&james,&james);
    // result:
    // James pay for 1000 millions
    // James pay for 1200 millions

    notify_2(&james,&mario); // 这样就解决了一致性问题
    // result:
    // James pay for 1000 millions
    // Mario pay for 1200 millions
}

trait Daddy {
    fn pay(&self,amount: i32);
}

// 使用了特征进行约束，只能是James
#[derive(Debug)]
struct James{
    name: String,
    age: u8,
}

#[derive(Debug)]
struct Mario{
    name: String,
    age: u8,
}

impl Daddy for Mario {
    fn pay(&self,amount: i32) {
        println!("{} was {} pay for {} millions",self.name,self.age,amount);
    }
}

impl Daddy for James {
    fn pay(&self,amount: i32) {
        println!("{} was {} pay for {} millions",self.name,self.age,amount);
    }
}

// 使用特征进行约束<T: xxx>
fn notify<T: Daddy + Debug>(james: &T,mario: &T){
    let james_ = james.pay(1000);
    let mario_ = mario.pay(1200);
    format!("{:?}{:?}",james_,mario); // 无效输出的还是 james
}

fn notify_2<T>(james: &T,mario: &(impl Daddy + Debug))
where T: Daddy + Debug // 或者使用where子句进行特征的约束
{ // 让参数Daddy实现Debug输出
    let james_ = james.pay(1000);
    let mario_ = mario.pay(1200);
    format!("{:?}{:?}",james_,mario);
}