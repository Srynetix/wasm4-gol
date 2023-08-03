CART_TITLE := "Game of Life"
CART_DESCRIPTION := "Simple Game of Life"

CART_FILE := "cart.wasm"
CART_DEBUG_PATH := "./target/wasm32-unknown-unknown/debug" / CART_FILE
CART_RELEASE_PATH := "./target/wasm32-unknown-unknown/release" / CART_FILE
WEB_EXPORT_RELEASE_PATH := "./export-release.html"
NATIVE_EXE_EXPORT_RELEASE_PATH := "./export-release.exe"

_default:
	@just --list

# Build the cartridge in debug mode
build-debug:
	cargo build

# Build the cartridge in release mode + strip
build-release:
	cargo build --release
	wasm-strip "{{CART_RELEASE_PATH}}"
	wasm-opt -Oz "{{CART_RELEASE_PATH}}" -o "{{CART_RELEASE_PATH}}"

# Build and run WASM (debug)
run-debug-web:
	@just build-debug
	w4 run --no-open "{{CART_DEBUG_PATH}}"

# Build the cartridge in release mode + strip and run WASM-4 on web
run-release-web:
	@just build-release
	w4 run --no-open "{{CART_RELEASE_PATH}}"

# Build the cartridge in release mode + strip and run WASM-4 on native mode
run-release-native:
	@just build-release
	w4 run-native "{{CART_RELEASE_PATH}}"

# Build the cartridge in release mode + strip and export to HTML file
export-release-web:
	@just build-release
	w4 bundle \
		--title "{{CART_TITLE}}" \
		--description "{{CART_DESCRIPTION}}" \
		--html "{{WEB_EXPORT_RELEASE_PATH}}" \
		"{{CART_RELEASE_PATH}}"

# Build the cartridge in release mode + strip and export to Windows EXE file
export-release-exe:
	@just build-release
	w4 bundle \
		--title "{{CART_TITLE}}" \
		--description "{{CART_DESCRIPTION}}" \
		--windows "{{NATIVE_EXE_EXPORT_RELEASE_PATH}}" \
		"{{CART_RELEASE_PATH}}"

# Build and run WASM-4 in watch mode (release, no-strip)
watch:
	w4 watch --no-open

# Analyze the debug cartridge
analyze-wasm-debug:
	twiggy top "{{CART_DEBUG_PATH}}"

# Analyze the release + strip cartridge
analyze-wasm-release:
	twiggy top "{{CART_RELEASE_PATH}}"

# Format the code
fmt:
	cargo fmt

# Run clippy on the code
lint:
	cargo clippy
