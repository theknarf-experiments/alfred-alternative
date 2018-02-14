
/* automatically generated by rust-bindgen */

pub const noErr: _bindgen_ty_1 = 0;
pub const kDurationImmediate: _bindgen_ty_1 = 0;
pub const kDurationForever: _bindgen_ty_1 = 2147483647;
pub const kDurationMillisecond: _bindgen_ty_1 = 1;
pub const kDurationMicrosecond: _bindgen_ty_1 = -1;
pub const kEventClassMouse: _bindgen_ty_1 = 1836021107;
pub const kEventClassKeyboard: _bindgen_ty_1 = 1801812322;
pub const kEventClassTextInput: _bindgen_ty_1 = 1952807028;
pub const kEventClassApplication: _bindgen_ty_1 = 1634758764;
pub const kEventClassAppleEvent: _bindgen_ty_1 = 1701867619;
pub const kEventClassMenu: _bindgen_ty_1 = 1835363957;
pub const kEventClassWindow: _bindgen_ty_1 = 2003398244;
pub const kEventClassControl: _bindgen_ty_1 = 1668183148;
pub const kEventClassCommand: _bindgen_ty_1 = 1668113523;
pub const kEventClassTablet: _bindgen_ty_1 = 1952607348;
pub const kEventClassVolume: _bindgen_ty_1 = 1987013664;
pub const kEventClassAppearance: _bindgen_ty_1 = 1634758765;
pub const kEventClassService: _bindgen_ty_1 = 1936028278;
pub const kEventClassToolbar: _bindgen_ty_1 = 1952604530;
pub const kEventClassToolbarItem: _bindgen_ty_1 = 1952606580;
pub const kEventClassToolbarItemView: _bindgen_ty_1 = 1952606582;
pub const kEventClassAccessibility: _bindgen_ty_1 = 1633903461;
pub const kEventClassSystem: _bindgen_ty_1 = 1835098995;
pub const kEventClassInk: _bindgen_ty_1 = 1768844064;
pub const kEventClassTSMDocumentAccess: _bindgen_ty_1 = 1952735587;
pub const kEventClassGesture: _bindgen_ty_1 = 1734701940;
pub const kEventRawKeyDown: _bindgen_ty_1 = 1;
pub const kEventRawKeyRepeat: _bindgen_ty_1 = 2;
pub const kEventRawKeyUp: _bindgen_ty_1 = 3;
pub const kEventRawKeyModifiersChanged: _bindgen_ty_1 = 4;
pub const kEventHotKeyPressed: _bindgen_ty_1 = 5;
pub const kEventHotKeyReleased: _bindgen_ty_1 = 6;
pub const activeFlagBit: _bindgen_ty_1 = 0;
pub const btnStateBit: _bindgen_ty_1 = 7;
pub const cmdKeyBit: _bindgen_ty_1 = 8;
pub const shiftKeyBit: _bindgen_ty_1 = 9;
pub const alphaLockBit: _bindgen_ty_1 = 10;
pub const optionKeyBit: _bindgen_ty_1 = 11;
pub const controlKeyBit: _bindgen_ty_1 = 12;
pub const rightShiftKeyBit: _bindgen_ty_1 = 13;
pub const rightOptionKeyBit: _bindgen_ty_1 = 14;
pub const rightControlKeyBit: _bindgen_ty_1 = 15;
pub type _bindgen_ty_1 = i32;
pub const activeFlag: _bindgen_ty_2 = 1;
pub const btnState: _bindgen_ty_2 = 128;
pub const cmdKey: _bindgen_ty_2 = 256;
pub const shiftKey: _bindgen_ty_2 = 512;
pub const alphaLock: _bindgen_ty_2 = 1024;
pub const optionKey: _bindgen_ty_2 = 2048;
pub const controlKey: _bindgen_ty_2 = 4096;
pub const rightShiftKey: _bindgen_ty_2 = 8192;
pub const rightOptionKey: _bindgen_ty_2 = 16384;
pub const rightControlKey: _bindgen_ty_2 = 32768;
pub type _bindgen_ty_2 = u32;
pub type SInt32 = ::std::os::raw::c_int;
pub type UInt32 = ::std::os::raw::c_uint;
pub type ItemCount = ::std::os::raw::c_ulong;
pub type Boolean = ::std::os::raw::c_uchar;
pub type EventTime = f64;
pub type OSStatus = ::std::os::raw::c_int;
pub type FourCharCode = ::std::os::raw::c_uint;
pub type OptionBits = ::std::os::raw::c_uint;
pub type OSType = ::std::os::raw::c_uint;
pub type EventTimeout = f64;
pub type EventTimerInterval = f64;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueEventTargetRef {
    _unused: [u8; 0],
}
pub type EventTargetRef = *mut OpaqueEventTargetRef;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueEventRef {
    _unused: [u8; 0],
}
pub type EventRef = *mut OpaqueEventRef;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueEventHotKeyRef {
    _unused: [u8; 0],
}
pub type EventHotKeyRef = *mut OpaqueEventHotKeyRef;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueEventHandlerCallRef {
    _unused: [u8; 0],
}
pub type EventHandlerCallRef = *mut OpaqueEventHandlerCallRef;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueEventHandlerRef {
    _unused: [u8; 0],
}
pub type EventHandlerRef = *mut OpaqueEventHandlerRef;
#[link(name = "Carbon", kind = "framework")]
extern "C" {
    #[link_name = "\u{1}_GetApplicationEventTarget"]
    pub fn GetApplicationEventTarget() -> EventTargetRef;

    #[link_name = "\u{1}_GetEventDispatcherTarget"]
    pub fn GetEventDispatcherTarget() -> EventTargetRef;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct EventHotKeyID {
    pub signature: ::std::os::raw::c_uint,
    pub id: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_EventHotKeyID() {
    assert_eq!(
        ::std::mem::size_of::<EventHotKeyID>(),
        8usize,
        concat!("Size of: ", stringify!(EventHotKeyID))
    );
    assert_eq!(
        ::std::mem::align_of::<EventHotKeyID>(),
        4usize,
        concat!("Alignment of ", stringify!(EventHotKeyID))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<EventHotKeyID>())).signature as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(EventHotKeyID),
            "::",
            stringify!(signature)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<EventHotKeyID>())).id as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(EventHotKeyID),
            "::",
            stringify!(id)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct EventTypeSpec {
    pub eventClass: ::std::os::raw::c_uint,
    pub eventKind: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_EventTypeSpec() {
    assert_eq!(
        ::std::mem::size_of::<EventTypeSpec>(),
        8usize,
        concat!("Size of: ", stringify!(EventTypeSpec))
    );
    assert_eq!(
        ::std::mem::align_of::<EventTypeSpec>(),
        4usize,
        concat!("Alignment of ", stringify!(EventTypeSpec))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<EventTypeSpec>())).eventClass as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(EventTypeSpec),
            "::",
            stringify!(eventClass)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<EventTypeSpec>())).eventKind as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(EventTypeSpec),
            "::",
            stringify!(eventKind)
        )
    );
}
pub type EventHandlerProcPtr = ::std::option::Option<
    unsafe extern "C" fn(inHandlerCallRef: EventHandlerCallRef, inEvent: EventRef, inUserData: *mut ::std::os::raw::c_void) -> OSStatus,
>;
pub type EventHandlerUPP = EventHandlerProcPtr;

#[link(name = "Carbon", kind = "framework")]
extern "C" {
    #[link_name = "\u{1}_RegisterEventHotKey"]
    pub fn RegisterEventHotKey(
        inHotKeyCode: UInt32,
        inHotKeyModifiers: UInt32,
        inHotKeyID: EventHotKeyID,
        inTarget: EventTargetRef,
        inOptions: OptionBits,
        outRef: &EventHotKeyRef,
    ) -> OSStatus;

    #[link_name = "\u{1}_SendEventToEventTarget"]
    pub fn SendEventToEventTarget(inEvent: EventRef, inTarget: EventTargetRef) -> OSStatus;

    #[link_name = "\u{1}_InstallEventHandler"]
    pub fn InstallEventHandler(
        inTarget: EventTargetRef,
        inHandler: EventHandlerUPP,
        inNumTypes: ItemCount,
        inList: *const EventTypeSpec,
        inUserData: *mut ::std::os::raw::c_void,
        outRef: *mut EventHandlerRef,
    ) -> OSStatus;

    #[link_name = "\u{1}_ReceiveNextEvent"]
    pub fn ReceiveNextEvent(
        inNumTypes: ItemCount,
        inList: *const EventTypeSpec,
        inTimeout: EventTimeout,
        inPullEvent: Boolean,
        outEvent: &EventRef,
    ) -> OSStatus;

    #[link_name = "\u{1}_ReleaseEvent"]
    pub fn ReleaseEvent(inEvent: EventRef);
}
