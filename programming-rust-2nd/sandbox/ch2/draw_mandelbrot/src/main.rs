use num::Complex;

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

fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re,im)) => Some(Complex{re, im}),
        None => None
    }
}

#[test]
fn test_parse_complex() {
    assert_eq!(parse_complex("1.25,-0.0625"), Some(Complex{re: 1.25, im:-0.0625}))
}

/**
 * ピクセル情報を複素数に変換する関数である
 */
fn pixel_to_point(bounds: (usize, usize),
                    pixel: (usize, usize),
                    upper_left: Complex<f64>,
                    lower_right: Complex<f64>) -> Complex<f64> {
        let (width, height) = (lower_right.re - upper_left.re,
                                upper_left.im - lower_right.im);
        Complex{
            re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
            im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64
        }
    }

#[test]
fn test_pixel_to_point() {
    assert_eq!(pixel_to_point(
                (100, 200), (25, 175),
                Complex { re: -1.0, im:  1.0 },
                Complex { re:  1.0, im: -1.0 }),
                    Complex { re: -0.5, im: -0.75 });
}

fn main() {
    println!("Hello, world!");
}
