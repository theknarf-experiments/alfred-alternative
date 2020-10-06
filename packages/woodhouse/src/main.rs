#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate libc;
extern crate web_view;
extern crate macos_hotkey;

use web_view::*;
use macos_hotkey::*;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Copy)]
#[derive(Clone)]
enum ViewVisible {
    Visible,
    Hidden,
}

static mut view_status : ViewVisible = ViewVisible::Hidden;
static html_content : &str = include_str!("../../frontend/dist/index.html");

unsafe extern "C" fn callback(
    _inHandlerCallRef: EventHandlerCallRef,
    _inEvent: EventRef,
    _inUserData: *mut ::std::os::raw::c_void
) -> OSStatus {
    match view_status {
        ViewVisible::Visible => view_status = ViewVisible::Hidden,
        ViewVisible::Hidden => view_status = ViewVisible::Visible,
    }
    println!("Hotkey {:?}", view_status);

    0
}

fn main() {
    println!("Subscribing to hotkey cmd+space");

    let mut prev_view_status = ViewVisible::Hidden;

    let mut view : Option<web_view::WebView<_>> = None;

    unsafe {
        let (event, eventTarget) = SubHotkey(callback);
        loop {
            if ReceiveNextEvent(
                0,
                0 as *const EventTypeSpec,
                kDurationForever as f64,
                1,
                &event
            ) == noErr {
                SendEventToEventTarget( event, eventTarget );
                ReleaseEvent( event );
            }

            if prev_view_status != view_status {
                match view_status {
                    ViewVisible::Hidden => {
                        match view {
                            Some(ref mut iView) => iView.exit(),
                            None => (),
                        };
                    },
                    ViewVisible::Visible => {
                        view = Some(web_view::builder()
                            .content(Content::Html(html_content))
                            .size(320, 480)
                            .resizable(false)
                            .frameless(true)
                            .debug(true)
                            .user_data(())
                            .invoke_handler(|_webview, _arg| Ok(()))
                            .build()
                            .unwrap());
                    },
                }
            }

            prev_view_status = view_status;

            if view_status == ViewVisible::Visible {
                match view {
                    Some(ref mut iView) => {
                        iView.step();
                    },
                    None => (),
                };
            }
        }
    }
}

