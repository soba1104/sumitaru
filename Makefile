compile:
	@cargo +nightly build --features clippy

run:
	@cargo +nightly run --features clippy

clean:
	@cargo clean
