fn ref_string(s: &String) {
  println!("{}", s);
}

fn refmut_string(s: &mut String) {
  // ここでsに対して変更を加えるなどの操作も可能。
  println!("{}", s);
}

fn main() {
  let s = "this is a resource".to_string();
  // 参照1つめ。
  let _t = &s;
  // 参照2つめ。同時に2つ存在できる。
  ref_string(&s);

  // ミュータブルな参照を使う場合の話、ミュータブルなものを複数スレッドからアクセスできると面倒なので
  // 参照は同時に２つ以上は作られない
  let mut s = "this is a resource".to_string();
  // ミュータブルな参照1つめ。
  let _t = &mut s;
  // ミュータブルな参照2つめはエラー。
  // refmut_string(&s); // error[E0499]: cannot borrow `s` as mutable more than once at a time

}