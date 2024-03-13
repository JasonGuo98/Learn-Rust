fn main() {
    // let guess = "42".parse().expect("Not a number!"); // 报错，需要显示说明类型
    let guess: u32 = "42".parse().expect("Not a number!");
    let guess = "42".parse::<i32>().expect("Not a number!");

    let a: u8 = 255;
    let b = a.wrapping_add(20);// 补码溢出+，显示处理溢出
    println!("wrapping_add: {}", b); // 19

    let a: u8 = 255;
    let b = a.overflowing_add(20);// 方法返回该值和一个指示是否存在溢出的布尔值
    println!("overflowing_add: {:?}", b); // (19, true)


    let a: u8 = 255;
    let b = a.checked_add(20);// 方法时发生溢出，则返回 None 值
    println!("checked_add: {:?}", b); // None

    let a: u8 = 255;
    let b = a.saturating_add(20);// 可以限定计算后的结果不超过目标类型的最大值或低于最小值
    println!("saturating_add: {:?}", b); // 255, {:?}是一种格式化输出的占位符，它告诉println!宏将值以调试格式打印出来。调试格式是一种可读性较高的格式，用于显示变量的值以及其类型的详细信息。


    // 浮点数类型

    let x = 2.0; // f64

    let y: f32 = 3.0; // f32, 注意rust的map不能用浮点数作为key，因为浮点数不太好进行大小比较（有误差）

    // assert!(0.1 + 0.2 == 0.3); // 错误
    assert!((0.1_f64 + 0.2 - 0.3).abs() < 1e-5); // OK

    let z = 13.14_f32.round();// 必须说明数据类型才能截断
    let z = y.round();// 必须说明数据类型才能截断
    let z = (x as f64).round();// 必须说明数据类型才能截断
    // let z = x.round();// 错误


    // 处理NaN

    let v = -42.0_f64;
    let x = (v).sqrt();
    if x.is_nan(){
        println!("Undefined sqrt for {}", v);
    }

    // 数学运算

    let twenty = 20;

    let twenty_one: i32 = 21;

    let twenty_two = 22i32;

    // 相同类型才能计算
    let add_v = twenty + twenty_one + twenty_two;
    println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, add_v);

    // 对于较长的数字，可以用_进行分割，提升可读性
    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2));

    let forty_twos = [
        42.0,
        42f32,
        42.0_f32,
    ];

    println!("{:.2}", forty_twos[0]);

    // 位运算

    // 二进制为00000010
    let a:i32 = 2;
    // 二进制为00000011
    let b:i32 = 3;

    println!("(a & b) value is {}", a & b);

    println!("(a | b) value is {}", a | b);

    println!("(a ^ b) value is {}", a ^ b);

    println!("(!b) value is {} ", !b);// 按位非，这个和C不同

    println!("(a << b) value is {}", a << b);

    println!("(a >> b) value is {}", a >> b);

    let mut a = a;
    // 注意这些计算符除了!之外都可以加上=进行赋值 (因为!=要用来判断不等于)
    a <<= b;
    println!("(a << b) value is {}", a);

    // 序列（Range）

    for i in 1..=5 {
        println!("{}", i); // 快速遍历，类似python的列表生成式
    }

    // for i in 'a'..='z' {
    //     println!("{}", i);
    // }

    // 使用包和复数
    use num::complex::Complex;

    let a = Complex {re: 2.1, im: -1.2};
    let b = Complex::new(11.1, 22.2);
    let result = a + b;

    println!("{} + {}i", result.re, result.im);
}
