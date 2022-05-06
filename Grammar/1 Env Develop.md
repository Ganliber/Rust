# Note-1

[TOC]



# hello-world

## Cargo new

> See link : [How to build a Rust Project](https://code.visualstudio.com/docs/languages/rust#_cargo-new)

A good way to create your first Rust program is to use Cargo to scaffold a new project by typing `cargo new`. This will create a simple Hello World program along with a default `Cargo.tml` dependency file. You pass `cargo new` the folder where you'd like to create the project.

Let's create Hello World. Navigate to a folder where you'd like to create your project and type:

```
cargo new hello_world   
## 在本地某处建一个文件夹hello_world,同时也是一个rust project
## 它还初始化了一个新的 Git 存储库以及一个.gitignore文件。cargo new如果您在现有的 Git 存储库中运行，则不会生成 Git 文件；您可以使用cargo new --vcs=git.
## cargo new如果您在现有的 Git 存储库中运行，则不会生成 Git 文件；您可以使用cargo new --vcs=git.
```

To open your new project in VS Code, navigate into the new folder and launch VS Code via `code .`:

```
cd hello_world
code .
```

> **Note**: Enable [Workspace Trust](https://code.visualstudio.com/docs/editor/workspace-trust) for the new folder as you are the author. You can enable Workspace Trust for your entire project folder parent to avoid being prompted when you create new projects by checking the option to **Trust the authors of all the files in parent folder 'my_projects`**.

`cargo new` creates a simple Hello World project with a `main.rs` source code file and `Cargo.toml` [Cargo manifest](https://doc.rust-lang.org/cargo/reference/manifest.html) file.

```
src\
    main.rs
.gitignore
Cargo.toml
```

`main.rs` has the program's entry function `main()` and prints "Hello, world!" to the console using `println!`.

```
fn main() {
    println!("Hello, world!");
}
```

This simple Hello World program doesn't have any dependencies but you would add Rust package (crate) references under `[dependencies]`.





## Cargo build[#](https://code.visualstudio.com/docs/languages/rust#_cargo-build)

Cargo can be used to build your Rust project. Open a new VS Code [integrated terminal](https://code.visualstudio.com/docs/editor/integrated-terminal) (Ctrl+Shift+\`) and type `cargo build`.

```
cargo build
```

You will now have `target\debug` folder with build output include an executable called `hello_world.exe`.

Cargo also provides a command called `cargo check`. This command quickly checks your code to make sure it compiles but doesn’t produce an executable:

```console
$ cargo check
   Checking hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
```



## Running Hello World

1. Cargo can also be used to run your Rust project via `cargo run`.

   ```
   cargo run
   ```

2. You can also run `hello_world.exe` manually in the terminal by typing `.\target\debug\hello_world`.

3. `cargo.toml`

   ```json
   [package]
   name = "hello_cargo"
   version = "0.1.0"
   edition = "2021"
   
   [dependencies]
   ```

   > 该文件采用[*TOML*](https://toml.io/)（*Tom's Obvious, Minimal Language*）格式，这是 Cargo 的配置格式。

   * Cargo 生成了一个“Hello, world!” 为您准备的程序，就像我们在清单 1-1 中编写的程序一样！到目前为止，我们之前的项目与 Cargo 生成的项目的不同之处在于 Cargo 将代码放在了*src* 目录中，而我们在顶层目录中有一个*Cargo.toml*配置文件。
   * Cargo 希望您的源文件位于*src*目录中。顶级项目目录仅用于 README 文件、许可证信息、配置文件以及与您的代码无关的任何其他内容。使用 Cargo 可以帮助您组织项目。一切都有一个地方，一切都在它的位置。









## Code navigation

Code navigation features are available in the context menu in the editor.

- **Go To Definition** `F12` - Go to the source code of the type definition.
- **Peek Definition** `Alt+F12 `- Bring up a Peek window with the type definition.
- **Go to References** `Shift+F12` - Show all references for the type.
- **Show Call Hierarchy** `Shift+Alt+H` - Show all calls from or to a function.

You can navigate via symbol search using the **Go to Symbol** commands from the **Command Palette** (`Ctrl+Shift+P`).

- Go to Symbol in File - `Ctrl+Shift+O`
- Go to Symbol in Workspace - `Ctrl+T`





## Summary

* `Cargo`
  1. We can build a project using `cargo build`.
  2. We can build and run a project in one step using `cargo run`.
  3. We can build a project without producing a binary to check for errors using `cargo check`.
  4. Instead of saving the result of the build in the same directory as our code, Cargo stores it in the *target/debug* directory.
* 







