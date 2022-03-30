ecc: src/main.rs

test: ecc
	./test.sh

clean:
	rm -f target/debug/ecc *.o *~ tmp*

.PHONY: test clean
