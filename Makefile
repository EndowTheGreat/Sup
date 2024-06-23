build:
	@cargo build --release -Z build-std=std,panic_abort \
	 -Z build-std-features="optimize_for_size" --target x86_64-unknown-linux-gnu