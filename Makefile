.PHONY: dev clean

dev:
	@cd net_topo && cargo check -q && cd ..

	@cd discovery_module && make -s build.win & cd ..

	@cd net_topo && cargo run -q

clean:
	@cd net_topo && cargo clean
	@cd discovery_module && make -s clean
