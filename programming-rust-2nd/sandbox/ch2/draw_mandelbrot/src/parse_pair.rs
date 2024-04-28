use std::str::FromStr;

fn parse_pair<T: FromStr>(s: &str, c: char) -> Option<(T,T)> {
	match s.find(c) {
		None => None,
		Some(index) => {
			match (T::from_str(&s[..index]), T::from_str(&s[index + 1 ..])) {
				(Ok(l), Ok(r)) => Some((l,r)),
				_ => None
			}
		}
	}
}

fn main(){
	parse_pair::<i32>("45,67", ',');
}