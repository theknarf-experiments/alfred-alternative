all:
	yarn
	yarn build:frontend
	cargo build

run: all
	cargo run
