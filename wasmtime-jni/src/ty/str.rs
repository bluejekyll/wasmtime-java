use jni::objects::JString;
use jni::JNIEnv;
use wasm_bindgen::convert::WasmSlice;
use wasmtime::WasmTy;

pub struct WasmJString<'j> {
    env: JNIEnv<'j>,
    string: JString<'j>,
}

impl<'j> WasmTy for WasmJString<'j> {
    type Abi = WasmSlice;

    #[inline]
    fn compatible_with_store(&self, _store: &Store) -> bool {
        true
    }

    #[inline]
    fn into_abi(self, store: &Store) -> Self::Abi {
        let jstr = self
            .env
            .get_string(&self.string)
            .expect("oh man, it's not a JString");

        let str = Cow::from(jstr);

        unimplemented!();
        // WasmSlice {
        //     pub ptr: u32,
        //     pub len: u32,
        // }
    }

    #[inline]
    unsafe fn from_abi<'a>(abi: Self::Abi, _store: &Store) -> Self {
        unimplemented!();
    }

    fn push(dst: &mut Vec<ValType>) {
        dst.push(ValType::I32); // offset into memory
        dst.push(ValType::I32); // length
    }

    fn matches(mut tys: impl Iterator<Item = ValType>) -> anyhow::Result<()> {
        // offset
        let next = tys.next();
        ensure!(
            next == Some(ValType::I32),
            "Expected offset for memory: {:?}"
            next
        );

        // length
        let next = tys.next();
        ensure!(
            next == Some(ValType::I32),
            "Expected length for memory: {:?}"
            next
        );
    }

    unsafe fn load_from_args(ptr: &mut *const u128) -> Self::Abi {
        unimplemented!();
    }

    unsafe fn store_to_args(abi: Self::Abi, ptr: *mut u128) {
        unimplemented!();
    }
}
