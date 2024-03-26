/*
 * Stringはそれ自体が文字列の所有者
 * - リードオンリーなら &str
 * - 書き換えたい場合なら &mut String
 * - 値をずっと持っておきたいなら String
 */
fn main() {
  // `&str`は`to_string()`メソッドで`String`にできる。
  let mut a: String = "abc".to_string();
  // 少しややこしいが、`String`に`&str`を足すと`String`ができる。
  // `&str`に`String`を足したり`String`に`String`を足したりはできない。
  a += "def";
  println!("{}", a); // => abcdef

  // `.to_string()`は様々な型に用意されている。
  let x = 1.0.to_string();
  println!("{}", x); // 1

  // `String`を`&str`にするには`as_str()`が使える。
  a += x.as_str();
  println!("{}", a); // => abcdef1
}