fn main() {
    let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
    println!("Pangram: {}", pangram);
    
    // 各文字に" ,"を追加して、文字列を作成
    let string = pangram.chars().map(|c| format!("{}, ", c)).collect::<String>();
    
    // スペースとカンマを含む文字列スライスを作成
    let chars_to_trim: &[char] = &[' ', ','];
    
    // trim_matchesメソッドに文字列スライスを渡して、スペースとカンマを削除
    let trimmed_str: &str = string.trim_matches(chars_to_trim);
    
    println!("Used characters: {}", trimmed_str);

    let alice = String::from("I like dogs");
    let bob: String = alice.replace("dog", "cat");

    println!("Alice says: {}", alice);
    println!("Bob says: {}", bob);
}