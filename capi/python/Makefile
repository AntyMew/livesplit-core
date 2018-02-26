BINDIR=livesplit_core
TARGETS=$(BINDIR)/lib/liblivesplit_core.so $(BINDIR)/livesplit_core.py
FLAGS=--release -p cdylib

all: $(TARGETS)

$(BINDIR)/lib:
	mkdir $@

$(BINDIR)/lib/liblivesplit_core.so: $(BINDIR)/lib
	@cargo build $(FLAGS)
	@cp ../../target/release/$(notdir $@) $@

$(BINDIR)/livesplit_core.py:
	@(cd ../bind_gen && cargo run)
	@cp ../bindings/$(notdir $@) $@

clean:
	@cargo clean
	@rm $(TARGETS)

install: $(TARGETS)
	@pip3 install .

uninstall:
	@pip3 uninstall livesplit-core