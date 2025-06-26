BINARY_NAME=rn
INSTALL_PATH=/usr/bin/$(BINARY_NAME)
PROJECT_DIR=rn
CARGO=/usr/bin/cargo

.PHONY: install uninstall

install:
	cd $(PROJECT_DIR) && $(CARGO) build --release
	cd $(PROJECT_DIR) && sudo cp target/release/$(BINARY_NAME) $(INSTALL_PATH)

uninstall:
	sudo rm -rf $(INSTALL_PATH)
