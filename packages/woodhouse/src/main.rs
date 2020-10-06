// Example copied from: https://github.com/Boscop/web-view
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate libc;
extern crate web_view;
extern crate macos_hotkey;

use web_view::*;
use macos_hotkey::*;

unsafe extern "C" fn callback(
    _inHandlerCallRef: EventHandlerCallRef,
    _inEvent: EventRef,
    _inUserData: *mut ::std::os::raw::c_void
) -> OSStatus {
    println!("hotkey");
    let html_content = include_str!("../../frontend/dist/index.html");

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


    0
}

fn main() {
    println!("Subscribing to hotkey cmd+space");

    unsafe {
        let (event, eventTarget) = SubHotkey(callback);
        while ReceiveNextEvent(
            0,
            0 as *const EventTypeSpec,
            kDurationForever as f64,
            1,
            &event
        ) == noErr {
            SendEventToEventTarget( event, eventTarget );
            ReleaseEvent( event );
        }
    }
}

