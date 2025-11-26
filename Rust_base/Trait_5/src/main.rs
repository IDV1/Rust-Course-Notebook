fn main() {
    let pair_1 = Pair {
        x: 3,
        y: 1
    };
    pair_1.cmp_display(); // 如果里面不存在Display和比较则不可使用

    let pair_2 = Pair::new(5,10);
    println!("{:?}",&pair_2);
}

use std::fmt::Display;

#[derive(Debug)]
struct Pair<T> {
    x: T,
    y: T,
}

// 给任何类型 T 的 Pair<T> 实现一些方法
impl<T> Pair<T> {
    // 里面的函数
    fn new(x: T, y: T) -> Self { // 这里面实现泛型T
        Self {
            x,
            y,
        }
    }
}

// 给任何 T约束是 xxx和xxx的Pair<T>实现一些方法
impl<T: Display + PartialOrd> Pair<T> {
    // 里面的方法
    fn cmp_display(&self) {
        // 比较PartiallOrd
        if self.x >= self.y {
            // 和Display，这两个要同时实现才可以使用这个Pair<T>里面的所有关联函数和方法
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
