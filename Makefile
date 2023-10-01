.PHONY: dev test clean

dev:
	@echo "Starting up in dev env..."

	@cd net_topo && cargo check -q && cd ..

	@make -C discovery_module -s build.win

	@cd net_topo && cargo update -q && cargo run -q

test:
	@cd net_topo &&	cargo test -q -- --nocapture

clean:
	@cd net_topo && cargo clean
	@cd discovery_module && make -s clean
