#include <iostream>
#include <Carbon/Carbon.h>
#include <CoreFoundation/CoreFoundation.h>

/* For reasons obscure to me RunApplicationEventLoop is not defined in */
/*  the mac header files if we are in 64 bit mode. Strangely it seems to */
/*  be in the libraries and functional */
#if __LP64__
extern void RunApplicationEventLoop(void);
#endif

static OSStatus MyHotKeyHandler(EventHandlerCallRef nextHandler, EventRef theEvent, void *userData)
{
	std::cout << "Hotkey press" << std::endl;
	return noErr;
}
int main(void)
{
	std::cout << "Starting app" << std::endl;
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

	RegisterEventHotKey(49, cmdKey, gMyHotKeyID, GetApplicationEventTarget(), 0, &gMyHotKeyRef);

	RunApplicationEventLoop();
}
