use termsize;

#[allow(dead_code)]
pub fn divide() {
    let term_size = termsize::get().unwrap();
    let term_width = term_size.cols as usize;
    let divide_line = String::from('-').repeat(term_width);
    println!("{divide_line}");
}
