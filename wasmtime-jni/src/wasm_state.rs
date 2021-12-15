use anyhow::{Context, Error};
use jni::JNIEnv;

use wasmtime_wasi::{sync::WasiCtxBuilder, WasiCtx};

/// Store associated data, currently empty
pub struct JavaState {
    wasi: WasiCtx,
}

impl JavaState {
    pub fn new(_env: JNIEnv<'_>) -> Result<Self, Error> {
        // TODO: Security considerations here, we don't want to capture the parent processes env
        //  we probably also want custom filehandles for the stdio of the module as well...
        //
        // see: https://docs.wasmtime.dev/examples-rust-wasi.html
        let wasi = WasiCtxBuilder::new()
            .inherit_stdio()
            .inherit_args()
            .context("failed to establish WASI context")?
            .build();

        Ok(JavaState { wasi })
    }

    pub fn wasi_mut(&mut self) -> &mut WasiCtx {
        &mut self.wasi
    }
}
