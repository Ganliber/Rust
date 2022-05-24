

fn main() {


/* Test 1 */
    println!("\n@1 variables test!\n");
    let x = 5;
    let x = x + 1;

    {
        let x = x + 2;

        println!("The value of x in the inner scope is :{}",x);
    }


/* Test 2 */
    println!("\n@2 statement and expression test!\n");
    let y = {

        let z = 3;

        z + 1

    };//{...}语句块是表达式(有返回值), 表达式不能包含分号, 否则就会变成一个语句(statement)
    println!("The value of y is: {}", y); // y == z+1 == 4

    //expression若不返回任何值,会隐式地返回一个'()'

    //let ...是一个语句(statement),不能赋值给其他值即不能作为右值
    println!("The value of x is : {}", x);


/* Test 3 */
    println!("\n@3 function test!\n");
    println!("function_1 calls!");
    let res = funtion_1(1,5);
    println!("The result of calling function_1 is:{}\n",res);
    
    println!("function_2 calls!");

    println!("When x>5 return x-5 :(x=6) return->{}", function_2(6));
    println!("When x<=5 return x+5 :(x=3) return->{}", function_2(3));
    
    println!("");

/* Test 4 */
    println!("@4 special return value!\n");

    /* () is a empty tuple 
     * 1. When function with no return value : return ()
     * 2. By the statement returning '()' when its endian is ';'
     */


/* Test 5 */
    println!("@5 Owner test 1!\n");
    let s = String::from("Hello");
    //s.push_str("World!"); ---> error: cannot borrow as mutable
    //println!("{}",s);
    let mut str_changable = String::from("Hey");
    str_changable.push_str(", Guys!");
    println!("{}",str_changable);

    println!("");

    
/* Test 6 */
    println!("@6 Owner test 2!\n");

    let s = String::from("hello");

    onwer_test(s);// 由于传递所有权到函数中而该函数没有返回所有权, 
                  // 因此s已经失效了所以再次使用s会报错

    // println!("{}",s);// error:value borrowed here after move

    println!("");


/* Test 7 */
    
}


fn funtion_1(x: i32, y: i32) -> i32 {
    println!("The value of x+y, x-y, x*y are: {}, {}, {}",x+y,x-y,x*y);
    x * y
}

fn function_2(x: i32) -> i32 {
    if x > 5 {
        return x-5
    }
    x + 5
}

/* 如果没有显示返回空元组(),那么如果函数最后一条语句以';'结尾就会报错 */
use std::fmt::Debug;//注意要有';'

fn report<T: Debug>(item: T) {
    /* This function return a "()" implicitly(隐式地) */
    println!("{:?}", item);
}

fn clear(text: &mut String) -> () {
    *text = String::from("");
}

// 会报错,因为函数最后一行代码以';'结尾那么返回的就是 "()" ,而不是u32
// fn add(x: i32, y:u32) -> u32 {
//     x + y;
// }

fn onwer_test(temp_str: String) {

    println!("{}",temp_str);

}