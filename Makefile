WASMTIME_TARGET_DIR := ${PWD}/target
NATIVE_TARGET_DIR := ${PWD}/target/native

.PHONY: clean
clean:
	cd wasmtime-jni && cargo clean
	mvn clean

target/native:
	@echo building wasmtime-jni
	cd wasmtime-jni && cargo build ${RELEASE} --lib
	@mkdir -p ${NATIVE_TARGET_DIR}
	@cp -rpf ${WASMTIME_TARGET_DIR}/debug/*.dll ${NATIVE_TARGET_DIR}/ || true
	@cp -rpf ${WASMTIME_TARGET_DIR}/debug/*.dylib ${NATIVE_TARGET_DIR}/ || true
	@cp -rpf ${WASMTIME_TARGET_DIR}/debug/*.so ${NATIVE_TARGET_DIR}/ || true

.PHONY: test
test:
	cd wasmtime-jni && cargo test
	rm -rf ${NATIVE_TARGET_DIR}
	$(MAKE) mvn-test

.PHONY: mvn-test
mvn-test: target/native
	mvn test