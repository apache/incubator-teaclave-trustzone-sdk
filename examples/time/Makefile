all:
	@cd host && cargo build --target aarch64-unknown-linux-gnu
	@make -s -C ta

clean:
	@cd host && cargo clean
	@make -s -C ta clean
