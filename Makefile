RUSTC ?= rustc
RUSTFLAGS ?= -O
TARGET = target/timestamp

SRC = $(wildcard src/hamcrest/*.rs) \
			$(wildcard src/hamcrest/matchers/*.rs)

all: $(TARGET)

$(TARGET): $(SRC)
	mkdir -p target
	$(RUSTC) --crate-type=lib $(RUSTFLAGS) --out-dir target src/hamcrest/lib.rs
	touch $(TARGET)

hamcrest-test: $(SRC)
	$(RUSTC) --test -o hamcrest-test src/hamcrest/lib.rs

test: hamcrest-test
	FOO=bar
	./hamcrest-test

clean:
	rm -rf target
	rm -f hamcrest-test

.PHONY: all test clean
