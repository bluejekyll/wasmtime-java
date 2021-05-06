ifeq (${OS}, Windows_NT)
    PLATFORM = Windows
	ifeq ($(PROCESSOR_ARCHITEW6432), AMD64)
        ARCH = x86_64
    else
	    ifeq ($(PROCESSOR_ARCHITECTURE), AMD64)
            ARCH = x86_64
        endif
		ifeq ($(PROCESSOR_ARCHITECTURE), x86)
            ARCH = i386
        endif
	endif
else
    PLATFORM = $(shell uname -s)
	ARCH = $(shell uname -m)
endif

ifeq (${PLATFORM}, Windows)
    DYLIB_EXT = dll
endif
 
ifeq (${PLATFORM}, Darwin)
    DYLIB_EXT = dylib
endif

ifeq (${PLATFORM}, Linux)
    DYLIB_EXT = so
endif

WASMTIME_TARGET_DIR := ${PWD}/target
NATIVE_TARGET_DIR := ${PWD}/target/native/${PLATFORM}/${ARCH}
WASM_TESTS := $(wildcard tests/*/Cargo.toml)
RUSTV := "+stable"

## This can be changed to the different wasm targets
# WASM_TARGET := wasm32-unknown-unknown
WASM_TARGET := wasm32-wasi

.PHONY: init
init:
	@echo "====> Testing for all tools"
	@mvn -version || (echo maven is required, e.g. 'brew install maven' && mvn -version)
	@cargo ${RUSTV} --version || (echo rust is required, e.g. 'curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh' && cargo --version)
	rustup target add ${WASM_TARGET}

.PHONY: clean
clean:
	@echo "====> Cleaning"
	cd wasmtime-jni && cargo clean
	mvn clean

target/native:
	@echo "====> Building wasmtime-jni for ${PLATFORM} arch ${ARCH}"
	cd wasmtime-jni && cargo ${RUSTV} build ${RELEASE} --lib
	@mkdir -p ${NATIVE_TARGET_DIR}
	@cp -rpf ${WASMTIME_TARGET_DIR}/debug/*.${DYLIB_EXT} ${NATIVE_TARGET_DIR}/

.PHONY: build
build:
	@echo "====> Building"
	cd wasmtime-jni && cargo ${RUSTV} build
	$(MAKE) ${WASM_TESTS}

	rm -rf ${PWD}/target/native
	$(MAKE) mvn-compile

.PHONY: ${WASM_TESTS}
${WASM_TESTS}:
	@echo "====> Building $(dir $@)"
	cd $(dir $@) && cargo ${RUSTV} build --target ${WASM_TARGET}

.PHONY: test
test: build
	@echo "====> Testing"
	cd wasmtime-jni && cargo ${RUSTV} test
	$(MAKE) mvn-test

.PHONY: mvn-test
mvn-test: target/native
	PLATFORM=${PLATFORM} ARCH=${ARCH} mvn verify

.PHONY: mvn-compile
mvn-compile: target/native
	PLATFORM=${PLATFORM} ARCH=${ARCH} mvn compile

.PHONY: package
package: build
	PLATFORM=${PLATFORM} ARCH=${ARCH} mvn package

.PHONY: cleanliness
cleanliness:
	cargo ${RUSTV} clean -p wasmtime-jni -p wasmtime-jni-exports -p math -p slices -p strings
	cargo ${RUSTV} clippy -- -D warnings
	cargo ${RUSTV} fmt -- --check