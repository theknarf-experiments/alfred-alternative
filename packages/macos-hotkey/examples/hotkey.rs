#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate libc;
extern crate macos_hotkey;

use macos_hotkey::*;

unsafe extern "C" fn callback(
    _inHandlerCallRef: EventHandlerCallRef,
    _inEvent: EventRef,
    _inUserData: *mut ::std::os::raw::c_void
) -> OSStatus {
    println!("hotkey");
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

