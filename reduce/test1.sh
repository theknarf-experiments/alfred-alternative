#!/bin/bash

cat <<EndOfMessage | 
#include "wrapper_pre.h"

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

	if( ReceiveNextEvent( 0, 0, kDurationForever, 1, &event ) == noErr )
	{
		SendEventToEventTarget( event, eventTarget );
		ReleaseEvent( event );
	}
}

EndOfMessage
gcc -xc -Wall -framework Carbon -o test_exe - >/dev/null 2>&1 && ./test_exe
