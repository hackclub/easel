BUILD_DIR = ./build

.PHONY: all build-win build-mac build-linux

all: build-win build-mac build-linux

build-win: $(BUILD_DIR)
	electron-packager . --platform=win32 --arch=x64 --overwrite --out=$(BUILD_DIR)

build-mac: $(BUILD_DIR)
	electron-packager . --platform=darwin --arch=x64 --overwrite --out=$(BUILD_DIR)

build-linux: $(BUILD_DIR)
	electron-packager . --platform=linux --arch=x64 --overwrite --out=$(BUILD_DIR)

$(BUILD_DIR):
	mkdir -p $(BUILD_DIR)
