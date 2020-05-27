// Example copied from: https://github.com/Boscop/web-view
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate libc;
extern crate web_view;

use web_view::*;
include!("wrapper.rs");

unsafe extern "C" fn callback(
    _inHandlerCallRef: EventHandlerCallRef,
    _inEvent: EventRef,
    _inUserData: *mut ::std::os::raw::c_void
) -> OSStatus {
    println!("hotkey");
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


    0
}

fn main() {
    let eventType = EventTypeSpec {
        eventClass: kEventClassKeyboard as u32,
        eventKind: kEventHotKeyPressed as u32
    };

    unsafe {
        let err = InstallEventHandler(
            GetApplicationEventTarget(),
            Some(callback),
            1,
            &eventType,
            0 as *mut std::os::raw::c_void,
            0 as *mut *mut OpaqueEventHandlerRef 
        );

        if err != noErr {
            panic!("aah error");
        }

        let gMyHotKeyRef_box : Box<OpaqueEventHotKeyRef> = Box::new(OpaqueEventHotKeyRef {
            _unused: []
        });
        let gMyHotKeyRef : EventHotKeyRef = Box::into_raw(gMyHotKeyRef_box);
        let gMyHotKeyID  = EventHotKeyID {
            signature: 101,
            id: 1
        };

        RegisterEventHotKey(49, cmdKey, gMyHotKeyID, GetEventDispatcherTarget(), 0, &gMyHotKeyRef);

        let event_box : Box<OpaqueEventRef>= Box::new(OpaqueEventRef {
            _unused: [] 
        });
        let event : EventRef = Box::into_raw(event_box);

        let eventTarget : EventTargetRef = GetEventDispatcherTarget();

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

