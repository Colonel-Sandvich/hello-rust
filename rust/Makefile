EXT := so

all: target/libdouble_input.$(EXT)

target/libdouble_input.$(EXT): src/lib.rs Cargo.toml
	cargo build
	ln -f $(realpath target/debug/libdouble_input.$(EXT)) ../src/main/resources/libdouble_input.$(EXT)

clean:
	rm -rf target