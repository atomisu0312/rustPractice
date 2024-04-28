```rust
fn parse_pair()
```
```rs
use std::str::FromStr;
fn parse_pair<T:FromStr>(s: &str, c: char) -> Optional(T,T) {
  match(s,c)
}
```

```rs
use std::str::FromStr;
fn parse_pair<T:FromStr>(s: &str, c: char) -> Option<T,T> {
  match()
}
```

```rs
use std::str::FromStr;
fn parse_pair<T:FromStr>(string: &str, separator: char) -> Option<T,T> {
  match string.find(separator) {
    None => return None, 
    Some(index) => {
      match (T::FromStr(&s[..index]), T::FromStr(&s[index+1..])){
        (Ok(k), Ok(l)) => Some((k,l))
        _ => None,
      }
    }
  }
}
```


```rs
use std::str::FromStr;
fn parse_pair<T::FromStr>(string: &str, separator: char)<>
```

```rs
use std::str::FromStr;
fn parse_pair<T::FromStr>(string: &str, separator: char) -> Option<T,T> {
  match string.find(separator){
    Ng
  }
}
```

```rs
use std::str::FromStr;
fn parse_pair<T::FromStr>(string: &str, separator: char)-> Option<T,T> {
  match string.find(separator) {
    None => None,
    Some(index) => {
      match (T::FromStr(string[..index]), T::FromStr(string[index+1..])) {
        (Ok(l), Ok(r)) => Some((l,r)), 
        _ => None
      }
    }
  }
}
// 正解
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
```

```rs
use std::
fn parse_pair
```

```rust
use std::str::FromStr;
fn parse_pair<T::FromStr>(s: &str, separator: char) -> Option<(T,T)>{
  match s.find(separator) {
    None => None,
    Some(index) => {
      match (T::FromStr(s[..index]), T::FromStr(s[index+1..])){
        (Ok(l), Ok(r)) => Some((l,r)),
        _ => None
      }
    }
  }
}
```

```rs
use std::str::FromStr;
fn parse_pair<T: FromStr> (s: &str, separator: char) -> Option<(T,T)>{
  match s.find(separator) {
    None => None,
    Some(index) => {
      match (T::from_str(&s[..index]), T::from_str($s[index+1..])) {
        (Ok(l), Ok(r)) => Some((l,r)),
        _ => None
      }
    }
  }
}
```

```rs
use std::str::Fromstr;

fn parse_pair<T: FromStr>(s:&str, separator : char)  -> Option<(T,T)>{
  match s.find(separator) {
    None => None,
    Some(index) => {
      match (T::from_str(&s[..index]), T::from_str(&s[index+1..])) => {
        (Ok(l), Ok(r)) => Some((l,r)),
        _ => None
      }
    }
  }
}
```

```rs
use std::str::FromStr;

fn parse_pair<T: FromStr> (s: &str, separator : char) -> Option<(T,T)> {
  match s.find(separator) {
    None => None,
    Some(index) => {
      match (T::from_str(&s[..index]), T::from_str(&s[index + 1 ..])) {
        (Ok(l), Ok(r)) => Some((l,r)),
        _ => None
      }
    } 
  }
}
```