fn main() {
    a_func(1, 3.2);

    let x = plus_5(5);

    println!{"The value of x is: {}", x};

    let x = plus_or_minus(6);

    println!{"The value of x is: {}", x};
}

fn a_func(x: i32, y: f64){
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn plus_5(x: i32) -> i32{
    x + 5
}


fn plus_or_minus(x: i32) -> i32{
    if x > 5{
        return x - 5; // 这里分号可有可没有
    }

    x + 5
}


use std::fmt::Debug;

// 隐式返回()
fn report<T: Debug>(item: T){
    println!("{:?}", item); 
}

// 显示返回()
fn clear(text: &mut String) -> (){
    * text = String::from("");
}

// 永远不返回的发散函数

fn dead_end() -> !{
    panic!("Never return func!");
}

// 死循环也不返回，因此也是发散函数
fn forever() -> !{
    loop{
        //...
    };
}