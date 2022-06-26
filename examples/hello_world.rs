//! Hello world server.
//!
//! A simple client that connects to a mini-redis server, sets key "hello" with value "world",
//! and gets it from the server after
//!
//! You can test this out by running:
//!
//!     cargo run --bin mini-redis-server
//!
//! And then in another terminal run:
//!
//!     cargo run --example hello_world

#![warn(rust_2018_idioms)]

use mini_redis::{client, Result};

/// main方法
/// 
/// ## 关于```async```和```await```
/// ### 相关依赖：
/// ```
/// [dependencies]
//  futures = "0.3"
/// ```
/// ### 使用async fn语法来创建的异步函数返回的值是一个Future
/// ```
/// async fn do_something() {
///   println!("go go go!");
/// }
/// fn main() {
///   do_something();// 这里返回的是一个Future，它不会调用具体的执行  
/// }
/// ```
/// ### 可以使用futures::executor::block_on来阻塞当前线程直到Future被执行完成
/// ```
/// let future = do_something();
/// block_on(future);
/// ```
/// ### 在async fn函数中使用.await可以等待另一个异步调用的完成。
/// ** 但是与block_on不同，.await并不会阻塞当前的线程，而是异步的等待Future A的完成，在等待的过程中，该线程还可以继续执行其它的Future B，最终实现了并发处理的效果。 **
/// ( 《Rust语言圣经》 /async-rust/async/getting-started.html )
/// 
/// ## 关于```#[tokio::main]```
/// ```
/// #[tokio::main]
/// async fn main() {
///     println!("hello");
/// }
/// ```
/// 相当于：
/// ```
/// fn main() {
///   let mut rt = tokio::runtime::Runtime::new().unwrap();
///   tr.block_on(async {
///      println!("hello");
///   });
/// }
/// ```
/// * .wait只能在async函数中使用
/// * #[tokio::main] 宏在将 async fn main 隐式的转换为 fn main 的同时还对整个异步运行时进行了初始化。
/// 
/// （ 《Rust语言圣经》/async-rust/tokio/getting-startted.html ）
/// 
/// ## 关于 main 方法的返回结果为Result<()> 以及 ? 符号的使用
///   常规的错误处理：
///   ```
///     let f = File::open("hello.txt");
///     let mut f = match f {
///        Ok(file) => file,
///        Err(e) => return Err(e),
///     }
///   ```
///   通过?来实现错误传播的简写:
///   ```
///     let mut s = String::new();
///     File::open("hello.txt")?.read_to_string(&mut s)?;
///   ```
///   当使用?时：如果 Result 的值是 Ok，这个表达式将会返回 Ok 中的值而程序将继续执行。
///   如果值是 Err，Err 中的值将作为整个函数的返回值，就好像使用了 return 关键字一样，这样错误值就被传播给了调用者。
///   另外
///     1. 使用?后可以将错误类型自动转换为返回类型指定的错误类型（当然，转换需要错误类型实现了From trait）
///     2. 可以链式调用
///   （ 文档：《Rust 程序设计语言》 /ch09-02-recoverable-errors-with-result.html ）
/// 
/// 另一个相关问题：client::connect返回类型是crate::Result<Client>，但是main函数返回的是Result<()>，而且std里的Result定义是两个结果参数的Result<T, E> ？
/// 看Rust语言圣经里的一个例子：
/// ```
/// use std::error::Error;
/// use std::fs::File;
///
/// fn main() -> Result<(), Box<dyn Error>> {
///     let f = File::open("hello.txt")?;
///
///     Ok(())
/// }
/// ```
/// std::error:Error 是 Rust 中抽象层次最高的错误，其它标准库中的错误都实现了该特征，
/// 因此我们可以用该特征对象代表一切错误，就算 main 函数中调用任何标准库函数发生错误，
/// 都可以通过 Box<dyn Error> 这个特征对象进行返回.
/// 我们可以看到这个例子里的Result实际是mini-redis的Result，而`mini-redis`里的Result实际是进行了重新定义的，具体见src/lib.rs
/// 
#[tokio::main]
pub async fn main() -> Result<()> {
    // Open a connection to the mini-redis address.
    let mut client = client::connect("127.0.0.1:6379").await?;

    // Set the key "hello" with value "world"
    client.set("hello", "world".into()).await?;

    // Get key "hello"
    let result = client.get("hello").await?;

    println!("got value from the server; success={:?}", result.is_some());

    Ok(())
}
