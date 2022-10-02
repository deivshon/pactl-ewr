CFLAGS = -Wall -Wextra -O2

all: change-vol-pactl

change-vol-pactl: change-vol-pactl.c config.h
	gcc $(CFLAGS) -o $@ $<

clean:
	rm -f change-vol-pactl *.o
