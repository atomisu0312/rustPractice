use std::env;
// 引数の解釈
fn process_args() -> Result<Vec<String>, String> {
  let args: Vec<String> = env::args().collect();

  if args.len() != 2 {
    println!("How to Use");
    println!("{} value\n", args[0]);
    return Err("引数の数が正しくありません。".to_string());
  }

  Ok(args)
}

// メイン
fn main() {
  let args: Result<Vec<String>, String> = process_args();

  match args {
    Ok(a) => {
        let string_value = &a[1]; //a[1]とするとエラーが出る
        match string_value.parse::<i64>() {
            Ok(x) if x < 0 => println!("負の整数: {}", x),
            Ok(x) => println!("非負整数: {}", x), // ちなみにこの行を削除してもエラーが出る
            Err(e) => println!("整数への変換に失敗しました: {}", e),
        }
    },
    Err(e) => println!("{}", e),
  }
}