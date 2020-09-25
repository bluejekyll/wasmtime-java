WASMTIME_TARGET_DIR := ${PWD}/target/wasmtime/target
NATIVE_TARGET_DIR := ${PWD}/target/native

.PHONY: clean
clean:
	rm -rf target

target/wasmtime:
	mkdir -p target
	cd target && git clone https://github.com/bytecodealliance/wasmtime.git
	cd target/wasmtime && git submodule update --init

target/native: target/wasmtime
	@echo fetching and building wasmtime
	cd target/wasmtime && (\
		cargo build --release --manifest-path crates/c-api/Cargo.toml --lib --features=jitdump,wat,cache \
	)
	@mkdir -p ${NATIVE_TARGET_DIR}
	@cp -rpf ${WASMTIME_TARGET_DIR}/release/*.dll ${NATIVE_TARGET_DIR}/ || true
	@cp -rpf ${WASMTIME_TARGET_DIR}/release/*.dylib ${NATIVE_TARGET_DIR}/ || true
	@cp -rpf ${WASMTIME_TARGET_DIR}/release/*.so ${NATIVE_TARGET_DIR}/ || true

.PHONY: test
test: target/native
	mvn test