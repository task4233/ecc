ecc: src/main.rs

test: ecc
	cargo build
	./test.sh

clean:
	rm -f target/debug/ecc *.o *~ tmp*

.PHONY: test clean
