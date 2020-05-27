// Example copied from: https://github.com/Boscop/web-view
extern crate web_view;
use web_view::*;

fn main() {
    let html_content = "<html><body><h1>Hello, World!</h1></body></html>";

    web_view::builder()
        .content(Content::Html(html_content))
        .size(320, 480)
        .resizable(false)
        .frameless(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
}
