use std::str::FromStr;
/**
 * <T:FromStr> Tはジェネリック型で、FromStr(trait)を継承しているものであると示す
 * 引数についてはいつも通り、参照わたししているだけ
 * Optionalは型Tである。OKならSome, NGならNo
 * s.findはOptional型を返すものである
 * T::from_str(&s[..index]), T::from_str(&s[index + 1..])については覚えておく
 * 
 */
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T,T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])){
                (Ok(l), Ok(r)) => Some((l,r)),
                _ => None
            }
        }
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10,20)));
    

}

fn main() {
    println!("Hello, world!");
}
