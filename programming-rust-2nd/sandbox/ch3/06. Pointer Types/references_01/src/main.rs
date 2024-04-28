fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // 不変参照を作成
    let r2 = &s; // 複数の不変参照を作成可能
    // let r3 = &mut s; // この行はコンパイルエラーになります

    println!("{} and {}", r1, r2);
}
