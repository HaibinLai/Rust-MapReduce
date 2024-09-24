all:
	cargo run --release

debug:
    cargo run --debug

clean:
	rm -rf mapreduce
