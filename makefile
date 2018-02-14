all:
	 gcc -Wall -o target/hotkey examples/globalhotkey.c -framework Carbon -ferror-limit=100 

bindgen:
	RUST_BACKTRACE=1 rustup run stable bindgen examples/globalhotkey.h > examples/wrapper.rs

pre:
	gcc -E -dD examples/wrapper.h > examples/wrapper_pre.h
	wc -l examples/wrapper_pre.h
