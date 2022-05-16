/**********************************
 * Reference and borrowing 
 *   1. 对于符号 & ,其允许你使用 value, 但不允许你获取所有权
 *   2. 当引用离开作用域后, 其指向的值也不会被丢弃
 *   3. 借用(borrowing)默认不可变
 *   4. 同一作用域, 特定数据只能有一个可变引用
 * */



fn refr_brw_main() {

/* referencing and dereferencing */
    let x = 5;
    let y = &x; // type of y is the reference of i32
    assert_eq!(5, x);
    assert_eq!(5, *y);

/* 不可变引用 */
    let s1 = String::from("Hello");

    let len = calculate_len(&s1);//把 s1的引用传给函数, 而不是把s1的所有权转移给函数

    println!("The length of '{}' is {}.", s1, len);


/* 可变引用 */

    let mut s = String::from("hello");

    change(&mut s);
}

fn calculate_len(s : &String) -> usize {
    /* &String 表明擦书 s 的类型是一个引用 */
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}