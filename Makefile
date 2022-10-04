INSTALL_PATH = /usr/local/bin

CFLAGS = -Wall -Wextra -O2

all: change-vol-pactl

change-vol-pactl: change-vol-pactl.c config.h
	gcc $(CFLAGS) -o $@ $<

clean:
	rm -f change-vol-pactl *.o

install: all
	mkdir -p $(INSTALL_PATH)
	cp change-vol-pactl $(INSTALL_PATH)
	chmod 755 $(INSTALL_PATH)/change-vol-pactl

uninstall:
	rm -f $(INSTALL_PATH)/change-vol-pactl

.PHONY: all clean install uninstall
