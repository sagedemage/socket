//! Message Color Functions

#[must_use]
pub fn red_message(msg: &str) -> String {
    /* Red message */
    let msg: String = format!("{}{}{}", "\x1b[31m", msg, "\x1b[0m");
    msg
}

#[must_use]
pub fn green_message(msg: &str) -> String {
    /* Green message */
    let msg: String = format!("{}{}{}", "\x1b[32m", msg, "\x1b[0m");
    msg
}

#[must_use]
pub fn blue_message(msg: &str) -> String {
    /* Blue message */
    let msg: String = format!("{}{}{}", "\x1b[34m", msg, "\x1b[0m");
    msg
}



