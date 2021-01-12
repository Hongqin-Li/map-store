gen:
	cargo run --bin data-generator -- -n 100000 -o datasets/test.txt

run-%:
	cargo run --bin $* -- datasets/test.txt -o target/tmp

