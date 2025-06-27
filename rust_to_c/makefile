# Путь к библиотеке
RUST_LIB_NAME=rust_to_c
RUST_TARGET_DIR=target/release
RUST_LIB=$(RUST_TARGET_DIR)/lib$(RUST_LIB_NAME).a

# Компилятор
CC=gcc
CFLAGS=-I. -L$(RUST_TARGET_DIR) -l$(RUST_LIB_NAME)

# Имя выходного файла
OUT=main

.PHONY: all clean

all: $(RUST_LIB)
	$(CC) c_src/main.c -o $(OUT) $(CFLAGS)

$(RUST_LIB):
	cargo build --release

clean:
	cargo clean
	rm -f $(OUT)

