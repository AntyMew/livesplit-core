BINDIR=livesplit_core
TARGETS=lib/liblivesplit_core.so $(BINDIR)/livesplit_core.py
FLAGS=--release -p cdylib

all: $(TARGETS)

lib:
	mkdir $@

lib/liblivesplit_core.so: lib
	@cargo build $(FLAGS)
	@cp ../../target/release/$(notdir $@) $@

$(BINDIR)/livesplit_core.py:
	@(cd ../bind_gen && cargo run)
	@cp ../bindings/$(notdir $@) $@

clean:
	@cargo clean
	@rm $(TARGETS)