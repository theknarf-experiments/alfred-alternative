#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

extern crate libc;

include!("wrapper.rs");

pub type CallbackT = unsafe extern "C" fn(*mut OpaqueEventHandlerCallRef, *mut OpaqueEventRef, *mut libc::c_void) -> i32;

pub unsafe fn SubHotkey(callback: CallbackT) -> (*mut OpaqueEventRef, *mut OpaqueEventTargetRef) {
    let eventType = EventTypeSpec {
        eventClass: kEventClassKeyboard as u32,
        eventKind: kEventHotKeyPressed as u32
    };

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

    return (event, eventTarget);
}
