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

.PHONY: clean
clean:
	cd wasmtime-jni && cargo clean
	mvn clean

target/native:
	@echo building wasmtime-jni for ${PLATFORM} arch ${ARCH}
	cd wasmtime-jni && cargo build ${RELEASE} --lib
	@mkdir -p ${NATIVE_TARGET_DIR}
	@cp -rpf ${WASMTIME_TARGET_DIR}/debug/*.${DYLIB_EXT} ${NATIVE_TARGET_DIR}/

.PHONY: test
test:
	cd wasmtime-jni && cargo test
	rm -rf ${PWD}/target/native
	$(MAKE) mvn-test

.PHONY: mvn-test
mvn-test: target/native
	PLATFORM=${PLATFORM} ARCH=${ARCH} mvn test