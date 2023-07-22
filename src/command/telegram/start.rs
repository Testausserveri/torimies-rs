use teloxide::prelude::*;

pub async fn run() -> ResponseResult<String> {
    Ok(String::from(
        "Get started by adding a Vahti. Use /help for a list of commands",
    ))
}
