ifdef SHELL
    SLASH := /
	CP_CMD := cp
else
	ifeq ($(OS),Windows_NT)
		SLASH := \\
		CP_CMD := copy
	else
		SLASH := /
		CP_CMD := cp
	endif
endif

.PHONY: build.win build.lin clean debug release all

# Default target for 'make' with no arguments will be 'debug'
.DEFAULT_GOAL := debug

# This sets up the conditional copy
ifeq ($(BUILD_MODE),release)
	COPY_DIR = ..$(SLASH)net_topo$(SLASH)target$(SLASH)release$(SLASH)
else
	COPY_DIR = ..$(SLASH)net_topo$(SLASH)target$(SLASH)debug$(SLASH)
endif

build.win:
	@echo "Building on windows machine..."
	@gcc -shared -o libdiscovery.dll -Wl,--out-implib,libdiscovery.lib src$(SLASH)discovery.c -Iinclude
	@$(CP_CMD) libdiscovery.dll $(COPY_DIR)

build.lin:
	@echo "Building on linux machine..."
	@gcc -shared -o libdiscovery.so src$(SLASH)discovery.c -Iinclude
	@$(CP_CMD) libdiscovery.so $(COPY_DIR)

debug: build.win

release:
	@gcc -shared -o libdiscovery.dll -Wl,--out-implib,libdiscovery.lib -O2 src$(SLASH)discovery.c -Iinclude
	@$(CP_CMD) libdiscovery.dll $(COPY_DIR)

clean:
	@rm -f *.lib *.dll *.so

all: debug
