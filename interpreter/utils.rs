use crate::error::Error;
use crate::token::Token;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub enum PrintType {
    Success,
    Error,
}

pub fn print_errors(errors: &Vec<Error>) {
    for err in errors {
        let Error { token, error_type } = err;
        let Token {
            token_type,
            line,
            start,
            end,
        } = token;

        print(
            &format!("{}.{}-{} : {}", line, start, end, error_type),
            PrintType::Error,
        );
    }
}

pub fn print(s: &str, print_type: PrintType) {
    let window = web_sys::window().expect("global window does not exist");
    let document = window.document().expect("document does not exist");
    let console = document
        .get_element_by_id("console")
        .expect("console element does not exist");
    let result_element = document
        .create_element("div")
        .expect("Failed to create element");

    let class = match print_type {
        PrintType::Success => "success",
        PrintType::Error => "error",
    };

    result_element.set_class_name(class);
    result_element.set_text_content(Some(s));
    console
        .append_child(&result_element)
        .expect("Failed to append element");
    console.set_scroll_top(console.client_height());
}
