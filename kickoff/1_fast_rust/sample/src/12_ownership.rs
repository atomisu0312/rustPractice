fn print_string(s: String) {
  println!("{}", s);
  // sはこの関数の終わりで消滅する。
  // このタイミングでsのメモリも自動で解放される。
}

fn main() {
  let s = "this is a resource".to_string();
  // 以下の行で、`s`が束縛されている文字列の所有権が`t`に移る。以後`s`は使えない。
  let t = s;
  // 以下の行で、文字列の所有権が`t`から`print_string`に移る。以後`t`は使えない。
  print_string(t);
  // もう一度`t`を使おうとしてもエラー。
  // print_string(t); // error[E0382]: use of moved value: `t`
  // 同じくsを使おうとしてもエラー。
  // print_string(s); // error[E0382]: use of moved value: `s`
}
