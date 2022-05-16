/**********************************
 * Reference and borrowing 
 * > 此处引用和借用是一个意思
 * > 定义: 获取变量的引用,称之为借用(borrowing)
 *   1. 对于符号 & ,其允许你使用 value, 但不允许你获取所有权
 *   2. 当引用离开作用域后, 其指向的值也不会被丢弃
 *   3. 借用(borrowing)默认不可变
 *   4. 同一作用域, 特定数据只能有一个可变引用: 避免数据竞争
 *   5. 可变引用(mutable borrowing)和不可变引用(immutable borrowing)不能同时存在
 *   6. a). 引用(reference)的作用域:从 s 创建开始持续到最后一次使用的地方
 *      b). 变量(variable)的作用域:持续到某一个'}'
 *   7. NLL ( Non-Lexical Lifetimes ) : 专门用于找到某个引用在作用域'}'结束前就不再被使用的代码位置
 * 
 * > Summary:
 *   1. 同一时刻, 你只能拥有要么一个 mutable borrowing, 要么任意多个 immutable borrowing
 *   2. 引用必须总是有效的
 * */


fn refr_brw_main() {

/* referencing and dereferencing */
    let x = 5;
    let y = &x; // type of y is the reference of i32
    assert_eq!(5, x);
    assert_eq!(5, *y);

/* 不可变引用  */
    let s1 = String::from("Hello");

    let len = calculate_len(&s1);//把 s1的引用传给函数, 而不是把s1的所有权转移给函数

    println!("The length of '{}' is {}.", s1, len);


/* 可变引用 */

    let mut s = String::from("hello");

    change(&mut s);

/* 冲突 1 :  */
    let mut a = String::from("hey");
    let r1 = &a;
    let r2 = &a;// 此时还没有问题

    // let r3 = &mut s;//此时出现问题
    // println!("{}, {}, and {}", r1, r2, r3);
/* 冲突 2 :  */
}

fn calculate_len(s : &String) -> usize {
    /* &String 表明擦书 s 的类型是一个引用 */
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}