# Only works for 32 bit, therefor -arch i386
all:
	 g++ -arch i386 -Wall -o target/hotkey src/globalhotkey.cpp -framework Carbon -framework ApplicationServices
