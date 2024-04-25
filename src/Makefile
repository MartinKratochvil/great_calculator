FILE_NAME = ../xhodakj00_xkratom00_xmatejj00_xottmat00
REPO = ../mockup ../plan ../profiling ../src ../tests ../Cargo.lock ../.gitignore ../Cargo.toml ../debugging.png ../hodnoceni.txt ../LICENSE ../README.md ../skutecnost.txt ../screenshot.png

all: dependencies
	cargo build --all-targets --release
	mkdir -p ../build
	cp ../target/release/main ../build/
	cp ../target/release/stddev ../build/

pack: dependencies
	mkdir -p $(FILE_NAME)/doc $(FILE_NAME)/install $(FILE_NAME)/repo
	cargo doc --no-deps
	cp -r ../target/doc/ $(FILE_NAME)/
	cargo clean
	rm -rf ../build 
	cp -r $(REPO) $(FILE_NAME)/repo 
	cp Makefile $(FILE_NAME)/repo/src/
#tady přidat packige do $(FILE_NAME)/install packige pro main a stddev
	zip -r $(FILE_NAME).zip $(FILE_NAME)
	rm -rf $(FILE_NAME)
 
clean: dependencies
	cargo clean

test: dependencies
	cargo test --doc

doc: dependencies
	cargo doc --no-deps

run: dependencies
	cargo run --release --bin main
	mkdir -p ../build
	cp ../target/release/main ../build/

profile: dependencies
	cargo build --release --bin stddev
	mkdir -p ../build
	cp ../target/release/stddev ../build/

install: dependencies
	cargo build --release --bin main
	sudo cp ../target/release/main /bin/ggc-bin
	chmod +x ggc.sh
	sudo cp ggc.sh /bin/ggc

dependencies:
	sudo apt install cargo

	

