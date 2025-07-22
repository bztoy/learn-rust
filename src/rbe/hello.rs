use crate::utils::{print_close_paragraph, print_topic_title};

pub fn formatted_print() {
    print_topic_title("Formatted print ...");

    const PI: f32 = 3.1415;
    const PI_PORTION: &str = "22/7";
    let what_pi_is: String = String::from(format!(
        "PI can be written in {} or decimal point format {}",
        PI, PI_PORTION
    ));
    println!("...{}...", what_pi_is);
    print_close_paragraph();
}
