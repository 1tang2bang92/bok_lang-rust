CC = clang
# CFLAGS = -W -Wall
TARGET = main
OBJECTS = main.o output.o

$(TARGET): $(OBJECTS)
	$(CC) -o $@ $^

run:
	./$(TARGET)