




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
    let new_s3 = s3.

}
