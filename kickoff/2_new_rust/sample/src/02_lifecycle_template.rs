// orig に prefix が含まれていればそれを削除した orig の一部を返し、
// orig に prefix が含まれていなければ orig を返す関数です。
// 'a は寿命を表し、orig と返り値は同じ寿命であることを表します。
fn maybe_remove_prefix<'a>(orig: &'a [i32], prefix: &[i32]) -> &'a [i32] {
  if orig.starts_with(prefix) {
    return orig.split_at(prefix.len()).1;
  } else {
    return orig;
  }
}

fn main() {
  let a = [1, 2, 3];
  let suffix;
  {
    let b = [1, 2];
    suffix = maybe_remove_prefix(&a, &b);
    // suffix = maybe_remove_prefix(&b, &a);
    // a = [4, 5, 6];
  }
  println!("{:?}", suffix);  // => [3]
}