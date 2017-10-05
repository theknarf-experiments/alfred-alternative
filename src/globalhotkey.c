#include <stdio.h>
#include <Carbon/Carbon.h>
#include <CoreFoundation/CoreFoundation.h>

static OSStatus MyHotKeyHandler(EventHandlerCallRef nextHandler, EventRef theEvent, void *userData)
{
	printf("Hotkey press\n");
	return noErr;
}
int main(void)
{
	OSStatus err = noErr;

	EventHotKeyRef gMyHotKeyRef;
	EventHotKeyID gMyHotKeyID;
	EventTypeSpec eventType;

	eventType.eventClass=kEventClassKeyboard;
	eventType.eventKind=kEventHotKeyPressed;
	err = InstallApplicationEventHandler(NewEventHandlerUPP(MyHotKeyHandler),1,&eventType,NULL,NULL);

	if(err != noErr){

		printf("Error: Could not install carbon event hook for input!\n");
		exit(0);
	}

	gMyHotKeyID.signature='htk1';
	gMyHotKeyID.id=1;

	RegisterEventHotKey(49, cmdKey, gMyHotKeyID, GetEventDispatcherTarget(), 0, &gMyHotKeyRef);

	EventRef        event;
	EventTargetRef    eventTarget;
	eventTarget = GetEventDispatcherTarget();

	while( ReceiveNextEvent( 0, NULL, kDurationForever, TRUE, &event ) == noErr )
	{
		SendEventToEventTarget( event, eventTarget );
		ReleaseEvent( event );
	}
}
