struct MyStruct{
    g: i32
}

fn main(){
    // let x = 5; // wrong
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6; // 原地修改
    println!("The value of x is: {}", x);

    let _x = 5;//被忽略
    let y = 6;// 不被忽略

    let (a, mut b) : (bool, bool) = (true, false);
    // a = true,不可变; b = false，可变
    println!("a = {:?}, b = {:?}", a, b);

    b = true;

    assert_eq!(a, b);


    let (c, d, e, f, g);

    (c, d) = (1, 2);

    [e, .., f, _] = [1, 2, 3, 4, 5];

    MyStruct {g, ..} = MyStruct {g: 5};

    assert_eq!([1, 2, 1, 4, 5], [c, d, e, f, g]);

    // 常量声明

    const MAX_POINTS: u32  = 100_000;

    let x = x + 1;// 变量屏蔽，会申请新的内存

    {
        let x = 2 * x;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces: usize = spaces.len();

    println!("The number of spaces is: {}", spaces);
}