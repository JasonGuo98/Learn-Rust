fn add_with_extra(x: i32, y: i32) -> i32{
    let x = x + 1;
    let y = y+ 2; // 语句，执行产生结果，无法作为返回值
    x + y // 表达式，表示了返回值
}

fn ret_unit_type(){
    let x = 1;
    let y = if x % 2 == 1{
        "odd"
    }else{
        "even"
    };

    let z = if x % 2 == 1 {"odd"} else {"even"};
}


fn main() {
    let y = {
        let x = 3;
        x + 1
    };// 这里的语句块的返回值是x+1的结果

    println!("The value of y is: {}", y);


    assert_eq!(ret_unit_type(),  ()); // 表达式如果不返回任何值，会隐式地返回一个 ()，称之为单元类型
}
