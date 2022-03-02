.PHONY: all
all:
	cargo build --release --target=x86_64-apple-darwin
	mkdir -p dest/alfred
	cp target/x86_64-apple-darwin/release/ipinfo dest/alfred/info
	cp main.py dest/alfred/main.py
	cp alfred/info.plist dest/alfred/info.plist
	zip -r dest/alfred.alfredworkflow dest/alfred