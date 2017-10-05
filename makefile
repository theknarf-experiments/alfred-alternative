all:
	 gcc -Wall -o target/hotkey src/globalhotkey.c -framework Carbon -framework ApplicationServices
