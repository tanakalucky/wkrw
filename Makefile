CC=gcc
CFLAGS=-O -Wall

all: wkrw

wkrw: wkrw.c wkrw.h
	$(CC) $(CFLAGS) -o wkrw wkrw.c -lncurses

clean:
	rm -f wkrw
