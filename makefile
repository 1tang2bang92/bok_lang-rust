CC = clang
# CFLAGS = -W -Wall
TARGET = main
OBJECTS = main.o output.o

$(TARGET): $(OBJECTS)
	$(CC) -o $@ $^

output.o: test.bs
	cargo run -- --do

run:
	./$(TARGET)