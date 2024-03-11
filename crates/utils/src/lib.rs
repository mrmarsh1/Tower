pub fn log(message: &str) {
    web_sys::console::log_1(&message.into());
}