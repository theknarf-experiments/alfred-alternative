#include <stdio.h>
#include <stdlib.h>

// forward decl

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
    kEventHotKeyReleased = 6
};

enum {    activeFlagBit = 0,   btnStateBit = 7,   cmdKeyBit = 8,   shiftKeyBit = 9,   alphaLockBit = 10,   optionKeyBit = 11,   controlKeyBit = 12,   rightShiftKeyBit = 13,   rightOptionKeyBit = 14,   rightControlKeyBit = 15 };
enum {    activeFlag = 1 << activeFlagBit,   btnState = 1 << btnStateBit,   cmdKey = 1 << cmdKeyBit,   shiftKey = 1 << shiftKeyBit,   alphaLock = 1 << alphaLockBit,   optionKey = 1 << optionKeyBit,   controlKey = 1 << controlKeyBit,   rightShiftKey = 1 << rightShiftKeyBit,   rightOptionKey = 1 << rightOptionKeyBit,   rightControlKey = 1 << rightControlKeyBit };

typedef signed int SInt32;
typedef unsigned int UInt32;
typedef SInt32 OSStatus;
typedef struct OpaqueEventTargetRef* EventTargetRef;
typedef struct OpaqueEventRef* EventRef;
typedef UInt32 FourCharCode;
typedef FourCharCode OSType;
typedef UInt32 OptionBits;
typedef unsigned long ItemCount;
typedef unsigned char Boolean;
typedef double EventTime;
typedef EventTime EventTimeout;
typedef EventTime EventTimerInterval;

typedef struct OpaqueEventHotKeyRef* EventHotKeyRef;
typedef struct OpaqueEventHandlerCallRef* EventHandlerCallRef;

typedef struct OpaqueEventHandlerRef* EventHandlerRef;

extern EventTargetRef GetApplicationEventTarget(void);
extern EventTargetRef GetEventDispatcherTarget(void);

struct EventHotKeyID {    OSType signature;    UInt32 id;  };
typedef struct EventHotKeyID EventHotKeyID;

struct EventTypeSpec {    OSType eventClass;    UInt32 eventKind;  };
typedef struct EventTypeSpec EventTypeSpec;

typedef OSStatus ( * EventHandlerProcPtr)(EventHandlerCallRef inHandlerCallRef, EventRef inEvent, void *inUserData);
typedef EventHandlerProcPtr EventHandlerUPP;

extern OSStatus RegisterEventHotKey(   UInt32 inHotKeyCode,   UInt32 inHotKeyModifiers,   EventHotKeyID inHotKeyID,   EventTargetRef inTarget,   OptionBits inOptions,   EventHotKeyRef * outRef) ;

extern OSStatus SendEventToEventTarget(   EventRef inEvent,   EventTargetRef inTarget) ;

extern OSStatus InstallEventHandler(   EventTargetRef inTarget,   EventHandlerUPP inHandler,   ItemCount inNumTypes,   const EventTypeSpec * inList,   void * inUserData,   EventHandlerRef * outRef) ;

extern OSStatus ReceiveNextEvent(   ItemCount inNumTypes,   const EventTypeSpec * inList,   EventTimeout inTimeout,   Boolean inPullEvent,   EventRef * outEvent) ;

extern void ReleaseEvent(EventRef inEvent) ;

// app

static OSStatus MyHotKeyHandler(EventHandlerCallRef nextHandler, EventRef theEvent, void *userData)
{
	printf("Hotkey press\n");
	return noErr;
}
int main(void)
{
	EventTypeSpec eventType;
	eventType.eventClass=kEventClassKeyboard;
	eventType.eventKind=kEventHotKeyPressed;

	OSStatus err = InstallEventHandler(
			GetApplicationEventTarget(),
			(EventHandlerUPP)MyHotKeyHandler,
			1,&eventType,0,0
	);

	if(err != noErr){
		exit(1);
	}

	EventHotKeyRef gMyHotKeyRef;
	EventHotKeyID gMyHotKeyID;
	gMyHotKeyID.signature='htk1';
	gMyHotKeyID.id=1;

	RegisterEventHotKey(49, cmdKey, gMyHotKeyID, GetEventDispatcherTarget(), 0, &gMyHotKeyRef);

	EventRef event;
	EventTargetRef eventTarget = GetEventDispatcherTarget();

	while( ReceiveNextEvent( 0, 0, kDurationForever, 1, &event ) == noErr )
	{
		SendEventToEventTarget( event, eventTarget );
		ReleaseEvent( event );
	}
}
