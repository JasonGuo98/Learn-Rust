fn main() {
    let s = String::from("hello"); //栈上

    let mut s = String::from("hello"); // 堆上，::是一种调用操作符

    s.push_str(", world!"); // 堆上，可修改

    println!("{}", s);


    let x = 5;
    let y = x; // 两个变量都在栈上，自动拷贝赋值


    let s1 = String::from("hello"); // s1拥有所有权
    let s2 = s1; // String不是基本类型，字符串存储在堆上，而变量存在栈上，有堆指针，字符串长度，字符串容量组成。
    // 这里的拷贝仅拷贝了指针，因此发生所有权转移，s1失效
    // 这里的操作也被称为变量转移

    // println!("{}", s1); // 错误，已经失效的变量

    let x: &str = "hello, world!";
    let y = x; // x, y同时引用同一个字符串
    println!("{}, {}", x, y); // 这里的x仅引用了存储在二进制中的字符串，没有持有所有权

    // 克隆（深拷贝）
    // rust永远不会自动创建数据的深拷贝
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("{}, {}", s1, s2);// 克隆进行完整的内存拷贝，性能较低

    // 拷贝（浅拷贝）
    // 仅发生在栈上
    let x = 5;
    let y = x;
    println!("{}, {}", x, y);// 基本类型在编译时已知，因此可以存储在栈上


    // 函数传值与返回

    let s = String::from("hello");// s进入作用域

    takes_ownership(s);
    //之后s失效
    // println!("{}", s);

    let x = 5;

    makes_copy(x);

    // x仍然有效
    println!("{}", x);

    //函数返回值的所有权

    let s1 = give_ownership(); // 获得所有权

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);

}

fn takes_ownership(some_str: String){
    println!("{}", some_str);
}// 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_int: i32){
    println!("{}", some_int);
}// 这里，some_integer 移出作用域。不会有特殊操作

fn give_ownership() -> String{
    let some_string = String::from("hello");

    some_string
}

fn takes_and_gives_back(a_str: String) -> String{
    // 给出所有权，然后返回所有权
    return a_str;
}