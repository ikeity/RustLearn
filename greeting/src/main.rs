fn main() {
    let a = 12;
    println!("Hello, world! a = {0}, a={0}", a);
    let mut b = 3;
    b = 4;
    println!("b = {0}", b);
    //const
    //Rust 语言中不允许有同名的常量
    const UI_THREAD:i32 = 10;
    let x_name = 1;
    let x_name = "123str";
    println!("const ui_thread = {0}", UI_THREAD);
    println!("x_name = {0}", x_name);

    //shadowing
    let x = 1;
    let x = x + 1;
    let x = x * 2;
    println!("Shadow x = {0}", x);

    //datatype
    //tup
    let tup:(i32, f64, u8) = (500, 1.11, 1);
    let (x, y, z) = tup;
    println!("datatype x={0}, y={1}, z={2}", x, y, z);
    //array
    let a = [1, 2, 3];
    println!("datatype array = {0}", a[0]);

    //字符串字面量 &str。它是 Rust 核心内置的数据类型
    //Rust 中的字符串字面量被称之为 字符串切片。因为它的底层实现是 切片。
    let company:&str = "简单教程";
    let user:&str = "测试使用html";
    println!("字符串字面量:{0},{1}", company, user);
    
    fun(5, 6);
    println!("func return 8 + 9 = {0}", fun_return(8, 9));
}

fn fun(x: i32, y: i32){
    println!("Hello, fun! x = {0}, y = {1}", x, y);
}

fn fun_return(x:i32, y:i32) -> i32{
    println!("fun return x = {0}, y = {1}", x, y);
    return x + y;
}