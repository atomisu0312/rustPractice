use std::rc::Rc;

fn main() {
    let s: Rc<Vec<String>> = Rc::new(vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()]);
    let t: Rc<Vec<String>> = s.clone();
    let u: Rc<Vec<String>> = s.clone();
}
