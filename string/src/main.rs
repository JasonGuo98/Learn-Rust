fn main() {
    // 切片

    let s = String::from("hello world!");

    let hello = &s[0..5];
    let world = &s[6..11]; // 左闭右开区间

    let len = s.len();
    
    let slice = &s[0..2];
    let slice = &s[..2];
    let slice = &s[4..len];
    let slice = &s[4..];
    let slice = &s[0..len];
    let slice = &s[..];

    // 在对字符串使用切片语法时需要格外小心，切片的索引必须落在字符之间的边界位置，也就是 UTF-8 字符的边界，例如中文在 UTF-8 中占用三个字节

    // 字符串切片的类型：&str


    let word = first_word(&s);
    println!("the first word is: {}", word);

    // 其他切片

    let a = [1, 2, 3, 4, 5 ];
    let slice = &a[1..3]; // 数组切片的类型是 &[i32], 持有一个引用指向原始数组的某个元素和长度
    assert_eq!(slice, &[2, 3]);

    // 字符串字面量
    let s = "Hello, world!"; // s 的类型是 &str
    let s: &str = "Hello world!";

    //String 和 &str转换
    // &str -> String
    let s = String::from("hello, world!");
    let s = ("hello, world!").to_string();

    // 将String -> &str
    say_hello(&s);
    say_hello(&s[..]);
    say_hello(s.as_str());

    // 字符串的底层的数据存储格式实际上是[ u8 ]，一个字节数组
    // 不同语言的字符串的长度不同，因此不允许索引操作

    // 追加（Push）

    let mut s = String::from("hello");

    s.push_str(" world!");
    println!("push_str() -> {}", s);
    
    // Insert
    
    s.insert(5, ',');
    println!("insert() -> {}", s);

    // 替换

    let string_replace = String::from("Hello rust, I like rust");
    // 用于 String 和 &str 类型, 返回一个新的String，而不是操作原始的String
    let new_string_replace = string_replace.replace("rust", "RUST");
    // dbg!(string_replace);
    dbg!(new_string_replace);

    // 用于 String 和 &str 类型, replacen：替换n个，同样返回一个新的String
    let new_string_replace = string_replace.replacen("rust", "RUST", 1);
    dbg!(new_string_replace);

    // 仅用于String类型，修改原始字符串的某个范围
    let mut string_replace_range = String::from("I like rust!");
    string_replace_range.replace_range(6..8, "R"); // 可以改变字符串长度
    dbg!(string_replace_range);

    // 删除

}

fn say_hello(s: &str){
    println!("{}", s);
}

fn first_word(s: &String) -> &str{
    &s[..1]
}