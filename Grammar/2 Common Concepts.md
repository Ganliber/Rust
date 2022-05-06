# Common Programming Concepts

[TOC]



## Variables and Mutability

* immutability error

  ```rust
  fn main() {
      // immutability error
      let x = 5;
      println!("The value of x is : {}", x);
      x = 6;
      println!("The value of x is : {}", x);
  }
  ```

  error output

  > `error[E0384]: cannot assign twice to immutable variable x `

  ```
     Compiling variables v0.1.0 (D:\github\Rust\Demo\variables)
  error[E0384]: cannot assign twice to immutable variable `x`    
   --> src\main.rs:5:5
    |
  3 |     let x = 5;
    |         -
    |         |
    |         first assignment to `x`
    |         help: consider making this binding mutable: `mut x`
  4 |     println!("The value of x is : {}", x);
  5 |     x = 6;
    |     ^^^^^ cannot assign twice to immutable variable
  
  For more information about this error, try `rustc --explain E0384`.
  error: could not compile `variables` due to previous error
  ```

  `Variables are immutable only by default. `

  ```rust
  fn main() {
      let mut x = 5;
      println!("The value of x is : {}", x);
      x = 6;
      println!("The value of x is : {}", x);
  }
  ```

  Summary :

  > Clarity !!! 

  * `Large data structures` : mutating an instance is faster than copying and returning newly allocated instances.
  * `Small data stuctures ` : creating new instances and writing in a more functional programming style may be easier to think through.



### Constants

* Difference between `constants` and `immutable variables (by default) `

  * `mut` is not allowed
  * `const` is needed instead of `let`
  * `data type` must be annotated (声明，注释)

* Example

  ```rust
  const THREE_HOURS_IN_SECTIONS : u32 = 60 * 60 * 3
  ```



### Shadowing

> You can declare a new variable with **the same name** as a previous variable.
>
> ---> `The first variable is shadowed by the second`

* Example

  ```rust
  fn main() {
      let x = 5;
      let x = x + 1; <--- it shadows `x` by repeating `let x=`
  
      {
          let x = x + 2;
          println!("The value of x in the inner scope is :{}",x);
      }
  
      println!("The value of x is : {}",x);
  }
  ```

  Output

  ```
  The value of x in the inner scope is :8
  The value of x is : 6
  ```

* Attention

  * `shadowing` is **different from** marking a variable as `mut`

    > 尤其是当第二次覆盖的值的类型与第一次不一致时候

    ```rust
    let spaces = "   ";
    let spaces = spaces.len();  --> No-Error
    -------------------------------
    let mut spaces = "   ";
    spaces = spaces.len();      --> Error[E0308]:mismatched types
    ```





## Data Types

> Two data type subsets : `scalar` and `compound`

* `Rust` is a statically typed language, which means that it must know the types of all variables at a compile time.

* `Compiler` can `infer` the type.

* `But` in cases when many types are possible, we must add a type annotation.

  ```rust
  let guess : u32 = "42".parse().expect("Not a number");
  ```

  > `error[E0282]:type annotation needed` 



### Scalar Types

* Rust has four scalar types : 
  * integers
  * floating-point numbers
  * Booleans
  * characters
* 













