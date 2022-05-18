fn main() {

/* 字符串操作: Operations On String */

/* 1.追加 */
    let mut s1 = String::from("Hello ");
    s1.push('r');
    println!("追加字符 push() -> {}", s1);

    s1.push_str("ust!");
    println!("追加字符串 push_str() -> {}", s1);

/* 2.插入 */
    let mut s2 = String::from("Hello rust!");
    s2.insert(5, ',');
    println!("插入字符 insert() -> {}", s2);
    
    s2.insert_str(6, "I like");
    println!("插入字符串 insert_str() -> {}",s2);

/* 3.替换 */
    let s3 = String::from("I like rust, Learning rust is my favorite!");
    let new_s3 = s3.replace("rust", "RUST");
    dbg!(new_s3);

    let s4 = "I like rust. learning rust is my favorite!";
    let new_s4 = s4.replacen("rust", "RUST", 1);// 最后一个参数是替换的个数
    dbg!(new_s4);

    let mut s5 = String::from("I like rust!");
    s5.replace_range(7..8, "R");
    dbg!(s5);

/* 4. 删除 */

//pop()
    let mut string_pop = String::from("rust pop 中文!");
    let p1 = string_pop.pop();
    let p2 = string_pop.pop();
    dbg!(p1);
    dbg!(p2);
    dbg!(string_pop);

//remove()
    let mut string_remove = String::from("测试remove方法");
    println!(
        "string_remove 占 {} 个字节",
        std::mem::size_of_val(string_remove.as_str())
    );
    //删除第一个汉字
    string_remove.remove(0);

    /* 错误代码 --- begin */
    //string_remove.remove(1);// UTF-8编码中汉字占 3 字节
    /* 错误代码 --- end */

    string_remove.remove(3);//直接删除第二个汉字(不过如果第一个汉字删除之后删除的就是'r'了)

    dbg!(string_remove);

//truncate
    let mut string_truncate = String::from("测试truncate");
    string_truncate.truncate(3);
    dbg!(string_truncate);

// clear
    let mut string_clear = String::from("Hello world");
    string_clear.clear();
    dbg!(string_clear);

//catenate
    // +/+= 第二个参数必须是切片引用(Slice)类型
    // +/+= 相当于 std::string standard library 里的 add() 方法
    // fn add(self, s:&str) -> String
    // 传入的参数 s 在 add() 结束时释放,因此所有权交给函数 add() 后就不返回了
    let string_append = String::from("hello ");

    let string_rust = String::from("rust");

    // &string_rust会自动解引用为&str
    let result = string_append + &string_rust;

    let mut result = result + "!";

    result += "!!!";

    println!("字符串连接 + -> {}", result);

    // format!
    let string_format_1 = "hello";

    let string_format_2 = String::from("rust");

    let string_format = format!("{} {}!", string_format_1, string_format_2);

    println!("{}",string_format);
    //print!("{}",string_format); --> 不换行
    //println!("");

    //Escape character
    let byte_escape = "I'm writing \x52\x75\x73\x74!";
    println!("1. Escape on ASCII: {} \n2. Two \'\\\' means no escape: \\x3F means \x3F",byte_escape);
    println!("");

    let unicode_escape = "\u{211D}";
    println!("3. Escape on Unicode: {} is called \"Double-Struck capital R\"",unicode_escape);
    println!("");

    //No escape by 'r', 当引号只有一对时,不必加 ##
    let str_raw = r"No escape by r: \x3F \u{211D}";
    let str_raw2 = r#"no escape by r: "\x3F \u{211D}""#;
    println!("{}", str_raw);
    println!("{}\n", str_raw2);

// operate on utf-8
    println!("按照unicode字符遍历");
    for c in "中国人".chars() {
        print!("{}",c);
    }
    println!("\n");

    println!("按照byte遍历");
    for b in "中国人".bytes() {
        println!("{}",b);
    }
    println!("");

/* 元组测试 --- begin */

    tuple_test();

/* 元祖测试 --- end */

/* 结构体测试 --- begin */

    struct_test();

/* 结构体测试 --- end */

}



fn tuple_test() {
    let tup1: (i32, f64, u8) = (500, 6.4, 1);
    println!("{}", tup1.0);

    let tup2 = (300, 2.4, 6);

    let (x,y,z) = tup2;//模式匹配

    println!("The value of y={}, x={}, z={}", y,x,z);

    let multiple_ret = String::from("Hello");
    let (s, len) = cal_len(multiple_ret);
    println!("The length of string \"{}\" is {}.", s, len);

}


fn cal_len(s: String) -> (String, usize) {
    let len = s.len();
    (s,len) //返回ownership
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// 构建函数,简化结构体的实现
fn build_user(email:String, username:String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    } // 结尾没有';',且当函数参数和字段同名时,可以直接使用缩略的方式进行初始化,
      // 这一点和 TypeScript 一致
}

fn struct_test() {
    /* Initailization 
     * 1.每个字段都要初始化
     * 2.顺序不需要和结构体定义时一致 */
    let mut user1 = User {
        email: String::from("2468982667@qq.com"),
        username: String::from("Ganliber"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("u202011061@gmail.com");

    println!("{} and {}", user1.username,user1.email);

    let user2 = build_user(String::from("2468982667@qq.com"), String::from("Ganliber"));

    println!("{} and {}", user2.username, user2.email);

}