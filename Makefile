.PHONY: build-linux
build-linux:
	cargo build --target x86_64-unknown-linux-musl --release --locked
	strip target/x86_64-unknown-linux-musl/release/dumbhttp
	upx target/x86_64-unknown-linux-musl/release/dumbhttp

.PHONY: build-win
build-win:
	RUSTFLAGS="-C linker=x86_64-w64-mingw32-gcc" cargo build --target x86_64-pc-windows-gnu --release --locked
	strip target/x86_64-pc-windows-gnu/release/dumbhttp.exe
	upx target/x86_64-pc-windows-gnu/release/dumbhttp.exe

.PHONY: build-apple
build-apple:
	cargo build --target x86_64-apple-darwin --release --locked
	strip target/x86_64-apple-darwin/release/dumbhttp
	upx target/x86_64-apple-darwin/release/dumbhttp
