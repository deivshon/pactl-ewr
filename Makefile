INSTALL_PATH = ~/.local/bin

CFLAGS = -Wall -Wextra -O2

all: change-vol-pactl

change-vol-pactl: src/main.rs
	cargo build --release

clean:
	cargo clean

install: all
	mkdir -p $(INSTALL_PATH)
	cp ./target/release/change-vol-pactl $(INSTALL_PATH)
	chmod 755 $(INSTALL_PATH)/change-vol-pactl

uninstall:
	rm -f $(INSTALL_PATH)/change-vol-pactl

.PHONY: all clean install uninstall
