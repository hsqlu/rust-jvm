// Rust 里面有两种字符串类型。String 和 str。
// &str
// str 类型基本上不怎么使用，通常使用 &str 类型，它其实是 [u8] 类型的切片形式 &[u8]。这是一种固定大小的字符串类型。 常见的的字符串字面值就是 &'static str 类型。这是一种带有 'static 生命周期的 &str 类型。

// 字符串字面值
let hello = "Hello, world!";

// 附带显式类型标识
let hello: &'static str = "Hello, world!";


// 创建一个空的字符串
let mut s = String::new();
// 从 `&str` 类型转化成 `String` 类型
let mut hello = String::from("Hello, ");
// 压入字符和压入字符串切片
hello.push('w');
hello.push_str("orld!");

// 弹出字符。
let mut s = String::from("foo");
assert_eq!(s.pop(), Some('o'));
assert_eq!(s.pop(), Some('o'));
assert_eq!(s.pop(), Some('f'));
assert_eq!(s.pop(), None);