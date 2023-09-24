.PHONY: build.win build.lin clean debug release all

# Default target for 'make' with no arguments will be 'debug'
.DEFAULT_GOAL := debug

# This sets up the conditional copy
ifeq ($(BUILD_MODE),release)
	COPY_DIR = ../net_topo/target/release/
else
	COPY_DIR = ../net_topo/target/debug/
endif

build.win:
	@gcc -shared -o libdiscovery.dll -Wl,--out-implib,libdiscovery.lib src/discovery.c -Iinclude
	@cp libdiscovery.dll $(COPY_DIR)

build.lin:
	@gcc -shared -o libdiscovery.so src/discovery.c -Iinclude
	@cp libdiscovery.so $(COPY_DIR)

debug: build.win

release:
	@gcc -shared -o libdiscovery.dll -Wl,--out-implib,libdiscovery.lib -O2 src/discovery.c -Iinclude
	@cp libdiscovery.dll $(COPY_DIR)

clean:
	@rm -f *.lib *.dll *.so

all: debug
