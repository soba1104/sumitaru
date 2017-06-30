HOST=127.0.0.1
PORT=3456

compile:
	@cargo +nightly clippy
	@cargo build

run:
	@cargo run ${HOST} ${PORT}

clean:
	@cargo clean
