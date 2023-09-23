INSTALL_PATH = ~/.local/bin

CFLAGS = -Wall -Wextra -O2

all: pactl-ewr

pactl-ewr: src/main.rs
	cargo build --release

clean:
	cargo clean

install: all
	mkdir -p $(INSTALL_PATH)
	cp ./target/release/pactl-ewr $(INSTALL_PATH)
	chmod 755 $(INSTALL_PATH)/pactl-ewr

uninstall:
	rm -f $(INSTALL_PATH)/pactl-ewr

.PHONY: all clean install uninstall
