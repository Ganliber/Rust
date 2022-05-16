/***********************
 * 所有权原则
 *      1. Rust 每一个 value ---> variable(owner of value)
 *      2. 一个 value 同时只能被一个变量所拥有
 *      3. 所有者(owner)离开作用域范围时候,该值被抛弃(drop)
 * 
 * 基本类型 (on stack) : Copy
 * 特殊类型 (on heap) : 所有权转移(move)
 *      string : heap_pointer(on stack), string_len, string_volumn(容量)
 *               字符串的堆指针是固定大小存在堆上
 * 
 * move 和 shallow copy :
 *      1. same : copy pointer,length,volumn
 *      2. diff : move->被拷贝者变为无效引用, 但 shallow copy 不会
 * 
 * Rust 永远也不会自动创建数据的“深拷贝” (deep copy)
 * 
 * i32 等基本类型是 copy
 * 对于一些长度可变的如 string ,则采取传递所有权的形式
 *      可变引用 &mut T 是不可以 copy 的
 *      不可变引用 &T 是可以 copy 的
*/


fn takes_ownership(some_string: String) {

    println!("{}",some_string);
}


fn makes_copy(somer_integer: i32) {
    println!("{}",somer_integer);
}


fn owner_main() {

    let s = String::from("hello");

    takes_ownership(s); // s 的值移动到函数里 ...
                        // ... 所以到此处不再有效
    let x = 5;

    makes_copy(x);  // x应该移动函数里,
                    // 但 i32是copy的
}