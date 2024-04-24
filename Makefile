FILE_NAME = xhodakj00_xkratom00_xmatejj00_xottmat00
REPO = mockup plan profiling src tests Cargo.lock .gitignore Cargo.toml debugging.png hodnoceni.txt LICENSE Makefile README.md skutecnost.txt screenshot.png

all:
	cargo build --all-targets --release
	mkdir -p build
	cp target/release/main build/
	cp target/release/stddev build/

pack:
	mkdir -p $(FILE_NAME)/doc $(FILE_NAME)/install $(FILE_NAME)/repo
	cargo doc --no-deps
	cp -r target/doc/ $(FILE_NAME)/
	cargo clean
	rm -rf build 
	cp -r $(REPO) $(FILE_NAME)/repo
	zip -r $(FILE_NAME).zip $(FILE_NAME)
	rm -rf $(FILE_NAME)

clean:
	cargo clean

test:
	cargo test --doc

doc:
	cargo doc

run:
	cargo run --release --bin main
	mkdir -p build
	cp target/release/main build/

profile:
	cargo run --bin stddev.rs
	mkdir -p build
	cp target/release/stddev build/
