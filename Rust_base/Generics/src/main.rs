fn main() {
    let str = "Hello";
    let str2 = "World";

    let num = 100;
    out(&mut Some(num));

    display::<String>(); // 在这里声明泛型Any的类型
    // display_2();
    // display_4();
    // display_5();
    display_6::<i32,6>(23); // 编译器已经知道它的长度是6了，只需设置他的值就行了
    display_7();
}

fn out<T: std::fmt::Debug>(o: &mut Option<T>) {
    match o {
        Some(o) => println!("{:?}",o), // 声明指定泛型T类型
        None => println!("{str}",str="什么都没有"),
        _ => println!("又是什么都没有")
    }
}

/*
 1.解决任意泛型：显示指定泛型的类型
 */
fn display<Any>()
where
    Any: From<String> +
    std::fmt::Display{ // 从这里开始是Any泛型，也就是说这里面的函数都必须是泛型的（如果有声明Any）
    // 因为这里不会自动转换成像JAVA那样的泛型，JAVA是强转换语言，Rust是强类型语言，需要显示转换类型
    // into方法就是为这个而生,会自动推导类型
    let string: Any = String::from("hello world").into();
    println!("string is {}", string);
    // 最后需要在参数声明后进行对泛型Any进行限定，包括后面的输出宏
}

/*
 2.在结构体中使用泛型：解决类型统一问题
 */
fn display_2() -> () {
    // 现在每一个Person都有自己的name，age，email了这样就不会造成统一性类型问题
    #[derive(Debug)]
    struct Person<Every_name, Every_age, Every_email> {
        name: Every_name,
        age: Every_age,
        email: Every_email
    }

    impl<Every_name, Every_age, Every_email> Person<Every_name, Every_age, Every_email>
    where
        Every_name: From<&'static str>,
        Every_age: From<i32>,
        Every_email: From<String> {
        fn new() -> Self{
            Self {
                name: "James".into(),
                age: 24.into(),
                email: "fuckyou@gmail.com".to_string().into()
            }
        }
    }
    let person: Person<&str,i32,String> = Person::new();
    dbg!("name is {}",&person);
}

/*
 3.在结构体中使用泛型
 */
fn display_3<OkType, ErrType> (ok: OkType,err: ErrType) {
    #[derive(Debug)]
    enum InputResult<OkType, ErrType>{
        Ok(OkType),
        Err(ErrType),
    }
}

fn display_4(){
    struct Point<T, U> {
        x: T,
        y: U,
    }

    // 为结构体实现泛型，这样Ponit就不是单纯一个Ponit，而是可以使用泛型<T,U>
    impl<T, U> Point<T, U> {
        fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }
}

/*
 4.对具体的泛型类型实现方法
 */
fn display_5() -> (){
    struct Point<T> {
        x: T,
        y: T
    }

    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    let origin = Point { x: 10.0, y: 20.0 };
    dbg!(Point::distance_from_origin(&origin));
}

/*
 5.const表达式进行编译期的泛型限制
 */
fn display_6<T, const N: usize>(t: T) // 在这里限制编译期const泛型，而不是运行期
where
    T: Copy + std::fmt::Debug,
{
    let array: [T; N] = [t; N];
    println!("array is {:?}", &array);
}

/*
 6.对函数fn进行限制
 */
fn display_7(){
    #[derive(Debug)]
    struct Buffer<const N: usize>{
        data: [usize; N],
    }

    // 限制这个函数，用来计算固定大小
    const fn area(val: usize) -> usize {
        val * val
    }

    const AREA: usize = area(2);
    // let buffer = Buffer{
    //     data:[1,2,3,4,5]
    // };

    let buffer = Buffer::<AREA>{ // 上面那个代码等价于这段代码
        data: [70;AREA]
    };
    println!("buffer is {:?}", &buffer);
}

/*
 7.泛型的单态化（泛型的多态：一对多）
 */
fn display_8(){
    #[derive(Debug)]
    enum Option<T>{
        Some(T),
        None,
    }

    let option_i32 = Option::<i32>::Some(22);// 填充编译期的指定类型
    let option_f64 = Option::<f64>::Some(22.0);// 填充编译期的指定类型
    println!("option_i32 is {:?}",option_i32);
    println!("option_f64 is {:?}",option_f64);
}

