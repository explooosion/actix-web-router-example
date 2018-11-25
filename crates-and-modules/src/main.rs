// extern crate test;
extern crate test;

// 如果 cargo.toml 名字包含 -
// 例如: my-name
// 則會自動轉換成底線
// 使用: extern crate my_name

// 從模組中提取函數使用
use test::chinese;
use test::japanese::greetings;

// 模組解構
// use test::chinese::farewells::goodbye;
use test::japanese::{farewells::goodbye, greetings::hello};

// 模組別名
use test::chinese::farewells::goodbye as exit;

fn main() {
    println!("Hello in English: {}", test::english::greetings::hello());
    println!("Hello in Chinese: {}", chinese::greetings::hello());
    println!("Hello in English: {}", greetings::hello());
    println!("Hello in Japanese: {}", hello());
    println!("Goodbye in Japanese: {}", goodbye());
    println!("Goodbye in Chinese: {}", exit());

    // println!("------------------------------");
    // println!("Hello in English: {}", test::english::greetings::hello());
    // println!("Goodbye in English: {}", test::english::farewells::goodbye());
    // println!("------------------------------");
    // println!("Hello in Chinese: {}", test::chinese::greetings::hello());
    // println!("Goodbye in Chinese: {}", test::chinese::farewells::goodbye());
    // println!("------------------------------");
    // println!("Hello in Japanese: {}", test::japanese::greetings::hello());
    // println!("Goodbye in Japanese: {}", test::japanese::farewells::goodbye());
    // println!("------------------------------");
}
