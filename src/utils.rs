pub type Run = fn();
pub struct Runner {
    pub is_run: bool,
    pub run: Run,
}

pub fn print_start_paragraph() {
    println!("+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++")
}
pub fn print_sub_paragraph_separator() {
    println!("---------------------------------------------------------------------")
}
pub fn print_close_paragraph() {
    println!("+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++\n")
}
pub fn print_topic_title(h: &str) {
    print_start_paragraph();
    println!("{}", h);
    print_sub_paragraph_separator();
}
