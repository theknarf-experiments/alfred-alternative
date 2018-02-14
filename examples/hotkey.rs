#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate libc;

include!("wrapper.rs");
//include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

extern fn callback() {
    println!("hotkey");
}

extern {
    // fn InstallApplicationEventHandler
    // fn NewEventHandlerUPP
    // struct EventTypeSpec
    // struct EventHotKeyRef
    // struct EventHotKeyID
    // struct EventHandlerCallRef
    // struct EventRef
    // fn RegistertEventHotKey
    // fn GetEventDispatcherTarget
    // fn ReceiveNextEvent
    // fn SendEventToEventTarget
    // fn ReleaseEvent
}

#[cfg(target_os = "macos")]
#[link(name = "Carbon", kind = "framework")]
fn main() {
    let eventType = EventTypeSpec {
        eventClass: kEventClassKeyboard,
        eventKind: kEventHotKeyPressed
    };

    println!("Hello world");
}

