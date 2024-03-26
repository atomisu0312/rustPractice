// 関数を定義する。
fn add(x: isize, y: isize) -> isize {
  x + y
}

fn main() {
  // 関数は`名前(引数)`で呼び出せる。
  println!("{}", add(1, 2)); // => 3
  // 関数を変数に束縛できる。
  let f: fn(isize, isize) -> isize = add;
  // 変数に束縛した関数も`名前(引数)`呼び出せる。
  let a = f(1, 2);
  println!("{}", a) // => 3
}