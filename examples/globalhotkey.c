#include "wrapper_reduced.h"

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
		printf("Error: Could not install carbon event hook for input!\n");
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
