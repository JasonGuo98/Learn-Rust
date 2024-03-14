fn main() {
    // 常规引用是指针类型

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    println!("x: {}, *y: {}, y: {}", x, *y, y); // 这里输出都是5

    // 不可变引用

    let s1 = String::from("hello");

    let len = calculate_length(&s1); // 传入一个引用，实际上是传入一个指向s1的指针

    // 可变引用
    // 同一作用域，特定数据只能有一个可变引用：目的是在编译期避免数据竞争
    // 大括号就是一个作用域，可以解决多个引用的问题

    let mut s = String::from("hello");

    change(&mut s);// 传入可变引用

    // 可变引用和不可变引用不能同时存在

    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    // let r3 = &mut s; // 不行，正在使用不可变引用的用户不希望数据被修改
    // 引用的作用域从创建开始，已知持续到最后一次使用，和变量持续到某个花括号不同
    
    println!("{}, {}", r1, r2);
    
    let r3 = &mut s; // r1,r2生命周期结束，可以创建可变引用
    println!("mut: {}", r3);

    // 悬垂引用（Dangling References）
    // 在 Rust 中编译器可以确保引用永远也不会变成悬垂状态：当你获取数据的引用后，编译器可以确保数据不会在引用结束前被释放，要想释放数据，必须先停止其引用的使用。

    // let ref_to_nothing = dangle();
}

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }// 离开作用域后s被释放


fn calculate_length(s: &String) -> usize { // 传入一个引用
    s.len()
}


fn change(some_string: &mut String){
    some_string.push_str(", world");
}