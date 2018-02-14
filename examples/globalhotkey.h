enum {
    noErr = 0,
    kDurationImmediate = 0,
    kDurationForever = 0x7FFFFFFF,
    kDurationMillisecond = 1,
    kDurationMicrosecond = -1,
    kEventClassMouse = 'mous',
    kEventClassKeyboard = 'keyb',
    kEventClassTextInput = 'text',
    kEventClassApplication = 'appl',
    kEventClassAppleEvent = 'eppc',
    kEventClassMenu = 'menu',
    kEventClassWindow = 'wind',
    kEventClassControl = 'cntl',
    kEventClassCommand = 'cmds',
    kEventClassTablet = 'tblt',
    kEventClassVolume = 'vol ',
    kEventClassAppearance = 'appm',
    kEventClassService = 'serv',
    kEventClassToolbar = 'tbar',
    kEventClassToolbarItem = 'tbit',
    kEventClassToolbarItemView = 'tbiv',
    kEventClassAccessibility = 'acce',
    kEventClassSystem = 'macs',
    kEventClassInk = 'ink ',
    kEventClassTSMDocumentAccess = 'tdac',
    kEventClassGesture = 'gest',
    kEventRawKeyDown = 1,
    kEventRawKeyRepeat = 2,
    kEventRawKeyUp = 3,
    kEventRawKeyModifiersChanged = 4,
    kEventHotKeyPressed = 5,
    kEventHotKeyReleased = 6,
    activeFlagBit = 0,
    btnStateBit = 7,
    cmdKeyBit = 8,
    shiftKeyBit = 9,
    alphaLockBit = 10,
    optionKeyBit = 11,
    controlKeyBit = 12,
    rightShiftKeyBit = 13,
    rightOptionKeyBit = 14,
    rightControlKeyBit = 15
};
enum {  
    activeFlag = 1 << activeFlagBit,
    btnState = 1 << btnStateBit,
    cmdKey = 1 << cmdKeyBit,
    shiftKey = 1 << shiftKeyBit,
    alphaLock = 1 << alphaLockBit,
    optionKey = 1 << optionKeyBit,
    controlKey = 1 << controlKeyBit,
    rightShiftKey = 1 << rightShiftKeyBit,
    rightOptionKey = 1 << rightOptionKeyBit,
    rightControlKey = 1 << rightControlKeyBit
};

typedef signed int SInt32;
typedef unsigned int UInt32;
typedef unsigned long ItemCount;
typedef unsigned char Boolean;
typedef double EventTime;

typedef signed int OSStatus; //typedef SInt32 OSStatus;
typedef unsigned int FourCharCode; //typedef UInt32 FourCharCode;
typedef unsigned int OptionBits; //typedef UInt32 OptionBits;
typedef unsigned int OSType; //typedef FourCharCode OSType;
typedef double EventTimeout; //typedef EventTime EventTimeout;
typedef double EventTimerInterval; //typedef EventTime EventTimerInterval;

typedef struct OpaqueEventTargetRef* EventTargetRef;
typedef struct OpaqueEventRef* EventRef;
typedef struct OpaqueEventHotKeyRef* EventHotKeyRef;
typedef struct OpaqueEventHandlerCallRef* EventHandlerCallRef;
typedef struct OpaqueEventHandlerRef* EventHandlerRef;

extern EventTargetRef GetApplicationEventTarget(void);
extern EventTargetRef GetEventDispatcherTarget(void);

struct EventHotKeyID {
    unsigned int /*OSType*/ signature;
    unsigned int /*UInt32*/ id;
};
typedef struct EventHotKeyID EventHotKeyID;

struct EventTypeSpec {
    unsigned int /*OSType*/ eventClass;
    unsigned int /*UInt32*/ eventKind;
};
typedef struct EventTypeSpec EventTypeSpec;

typedef OSStatus ( * EventHandlerProcPtr)(EventHandlerCallRef inHandlerCallRef, EventRef inEvent, void *inUserData);
typedef EventHandlerProcPtr EventHandlerUPP;

extern OSStatus RegisterEventHotKey(
    UInt32 inHotKeyCode,
    UInt32 inHotKeyModifiers,
    EventHotKeyID inHotKeyID,
    EventTargetRef inTarget,
    OptionBits inOptions,
    EventHotKeyRef* outRef
);

extern OSStatus SendEventToEventTarget(
    EventRef inEvent,
    EventTargetRef inTarget
);

extern OSStatus InstallEventHandler(
    EventTargetRef inTarget,
    EventHandlerUPP inHandler,
    ItemCount inNumTypes,
    const EventTypeSpec* inList,
    void* inUserData,
    EventHandlerRef* outRef
);

extern OSStatus ReceiveNextEvent(
    ItemCount inNumTypes,
    const EventTypeSpec * inList,
    EventTimeout inTimeout,
    Boolean inPullEvent,
    EventRef * outEvent
);

extern void ReleaseEvent(EventRef inEvent);
