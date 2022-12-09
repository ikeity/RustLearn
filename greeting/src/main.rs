fn main() {
    let a = 12;
    println!("Hello, world! a = {0}&&, a={0}", a);
    let mut b = 3;
    b = 4;
    println!("b = {}", b);
    //const
    //Rust 语言中不允许有同名的常量
    const UI_THREAD:i32 = 10;
    //let x_name = 1;
    let x_name = "123str";
    println!("const ui_thread = {0}", UI_THREAD);
    println!("x_name = {0}", x_name);

    //shadowing
    let x = 1;
    let x = x + 1;
    let x = x * 2;
    println!("Shadow x = {0}", x);

    //datatype
    //Rust 语言中每一值都有一个对应的变量，这个变量就成为这个值的 所有者。从某些方面说，定义一个变量就是为这个变量和它存储的数据定义一种所有者管理，声明这个值由这个变量所有。
    //Rust 中，任何特定时刻，一个数据只能有一个所有者。
    //Rust 中，不允许两个变量同时指向同一块内存区域。变量必须指向不同的内存区域。
    //Rust 语言中转让所有权的方式有以下几种：
    //  把一个变量赋值给另一个变量。重要
    //  把变量传递给函数作为参数。
    //  函数中返回一个变量作为返回值。

    // ①
    // 向量 v 拥有堆上数据的所有权
    // 每次只能有一个变量对堆上的数据拥有所有权
    let v = vec![1,2,3]; 
    // 赋值会导致两个变量都对同一个数据拥有所有权
    // 因为两个变量指向了相同的内存块
    let v2 = v; 
    // Rust 会检查两个变量是否同时拥有堆上内存块的所有权。
    // 如果发生所有权竞争，它会自动将所有权判给给新的变量
    // 运行出错，因为 v 不再拥有数据的所有权
    // println!("{:?}",v);

    // ②
    // 将堆中的对象传递给闭包或函数时，值的所有权也会发生变更
    let vdisplay = vec![1,2,3,4];
    let vdisplay2 = vdisplay;
    fun_display(vdisplay2);
    // 运行出错，因为 vdisplay2 不再拥有数据的所有权
    // println!("vdisplay2 = {:?}", vdisplay2);

    // ③
    // 传递给函数的所有权将在函数执行完成时失效
    // 也就是函数的形参获得的所有权将在离开函数后就失效了。失效了数据就再也访问不到的了。
    // 引用默认情况下是只读的，也就是我们不能修改引用的的变量的值


    //tup
    let tup:(i32, f64, u8) = (500, 1.11, 1);
    let (x, y, z) = tup;
    println!("datatype x={0}, y={1}, z={2}", x, y, z);
    let tup = (0.05, 23, "string");
    println!("{:?}", tup);
    println!("integer is :{:?}",tup.0);
    println!("float is :{:?}",tup.1);
    println!("unsigned integer is :{:?}",tup.2);

    //array
    let a = [1, 2, 3];
    println!("datatype array = {0}", a[0]);
    let array:[i32;4] = [1;4];
    println!("array = {:?}", array);
    
    //字符串字面量 &str。它是 Rust 核心内置的数据类型
    //Rust 中的字符串字面量被称之为 字符串切片。因为它的底层实现是 切片。
    let company:&str = "简单教程";
    let user:&str = "测试使用html";
    println!("字符串字面量:{0},{1}", company, user);
    //字符串对象 字符串对象是 Rust 标准库提供的内建类型。
    //与字符串字面量不同的是：字符串对象并不是 Rust 核心内置的数据类型，它只是标准库中的一个 公开 pub 的结构体。
    let empty_string = String::new();
    println!("长度为{}", empty_string.len());
    let content_string = String::from("字符串对象");
    println!("长度为{}", content_string.len());

    let str_to_string = "str_to_string";
    str_to_string.to_string();
    println!("str_to_string = {}", str_to_string);
    let str_replace = str_to_string.replace("string", "替换成功");
    println!("str_to_string  replace = {}", str_replace);
    println!("str_to_string str = {}", str_replace.as_str());
    let mut mut_string = String::from("字符串对象");
    mut_string.push('t');
    println!("mut_string push str = {}", mut_string);
    mut_string.push_str("\t简单\t教程 \r\n简单编程\r\n     ");
    println!("mut_string push str = {}, len = {}", mut_string, mut_string.len());
    println!("mut_string trim str = {}, len = {}", mut_string.trim(), mut_string.trim().len());
    let mut num = 0;
    for item in mut_string.split_whitespace() {
        println!("item {} {}", num, item);
        num += 1;
    }
    let fullname = "李白， 诗仙， 唐朝";
    for token in fullname.split("， ") {
        println!("token is {}", token);
    }
    let token_vec:Vec<&str> = fullname.split("， ").collect();
    for item_token in token_vec {
        println!("姓名 is {}",item_token);
    }
    for item_chars in fullname.chars() {
        println!("full name chars {}", item_chars);
    }
    let str1 = "iphone".to_string();
    let str2 = "plus".to_string();
    let str3 = str1 + &str2;// 需要传递 str2 的引用
    println!("string plus = {}", str3);
    let str_format = format!("{}++++{}", str3, str2);
    println!("str_format {}",str_format);

    // if else match
    if str3 == "iphoneplus" {
        println!("str3 = iphoneplus success");
    }
    let state_code = "MS";
    let state = match state_code {
        "MS" =>{println!("Found match for MH"); "Maharashtra"},
        "KL" => "Kerala",
        "KL1" => "Karnadaka",
        "KL2" => "Goa",
        _ => "default"
    };
    println!("State name is {}",state);

    fun(5, 6);
    println!("func return 8 + 9 = {0}", fun_return(8, 9));

    //没有 return 语句则使用最后一条语句的结果作为返回值
    println!("get pai {}", get_pai());
    println!("get pai no return {}", get_pai_noreturn());
    let mut param_no = 5;
    println!("param_no init = {}", param_no);
    fun_assist(&mut param_no);
    println!("param_no uninit = {}", param_no);

    // slice
    let n1 = "Tutorials".to_string();
    println!("length of string is {}", n1.len());
    let c1 = &n1[4..9];
    println!("slice is {}", c1);
    let data = [10,20,20,30,40];
    fun_use_slice(slice)



    let mut x = 0;
    loop {
        println!("loop");
        x += 1;
        if x > 10 {
            break;
        }
        else {
            println!("continue");
            continue;
        }
        //println!("continue11111111");
    }
}

fn fun(x: i32, y: i32){
    println!("Hello, fun! x = {0}, y = {1}", x, y);
}

fn fun_return(x:i32, y:i32) -> i32{
    println!("fun return x = {0}, y = {1}", x, y);
    return x + y;
}

fn get_pai() -> f64{
    return 22.0/7.0;
}

fn get_pai_noreturn() -> f64{
    22.0/7.0
}

fn fun_assist(param_no:&mut i32){
    *param_no = 10;
}

fn fun_display(v:Vec<i32>){
    println!("inside display{:?}", v);
}

fn fun_use_slice(slice:&[i32]){
    println!("length of slice is {:?}", slice.len());
    println!("content of slice is{:?}", slice);
}