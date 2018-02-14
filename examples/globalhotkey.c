#include <stdio.h>
#include <stdlib.h>
#include "globalhotkey.h"

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
