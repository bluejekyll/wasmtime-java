use anyhow::{anyhow, ensure, Error};
use wasmtime::{Store, Val, ValType};

use crate::{
    ty::{WasmAlloc, WasmSliceWrapper},
    wasm_state::JavaState,
};

pub(crate) trait ComplexTy {
    type Abi: Abi;

    fn compatible_with_store(&self, _store: &Store<JavaState>) -> bool;
}

pub(crate) trait Abi: Copy {
    /// Place the necessary type signature in the type list
    fn push_arg_tys(args: &mut Vec<ValType>);

    /// Place the values in the argument list
    fn store_to_args(self, args: &mut Vec<Val>);

    /// Load from the argument list
    fn load_from_args(args: impl Iterator<Item = Val>) -> Result<Self, anyhow::Error>;

    /// matches the arg tys
    fn matches_arg_tys(tys: impl Iterator<Item = ValType>) -> anyhow::Result<()>;
}

pub(crate) trait ReturnAbi: Abi {
    /// Place the necessary type signature in the type list
    #[allow(unused)]
    fn return_or_push_arg_tys(args: &mut Vec<ValType>) -> Option<ValType>;

    /// Matches the return type or the arg tys
    fn matches_return_or_arg_tys(
        ret: Option<ValType>,
        arg_tys: impl Iterator<Item = ValType>,
    ) -> Result<(), anyhow::Error>;

    fn get_return_by_ref_arg(args: impl Iterator<Item = Val>) -> Option<i32>;

    /// Place the values in the argument list
    #[allow(unused)]
    fn return_or_store_to_arg<'w>(
        args: &mut Vec<Val>,
        wasm_alloc: Option<&'w WasmAlloc>,
        store: &mut Store<JavaState>,
    ) -> Result<Option<WasmSliceWrapper<'w>>, Error>;

    /// Load from the argument list
    fn return_or_load_or_from_args(
        ret: Option<&Val>,
        ret_by_ref_ptr: Option<WasmSliceWrapper<'_>>,
        wasm_alloc: Option<&WasmAlloc>,
        store: &mut Store<JavaState>,
    ) -> Result<Self, anyhow::Error>;
}

impl<T: Abi + IntoValType + FromVal + MatchesValType> ReturnAbi for T {
    /// Place the necessary type signature in the type list
    #[allow(unused)]
    fn return_or_push_arg_tys(args: &mut Vec<ValType>) -> Option<ValType> {
        Some(Self::into_val_type())
    }

    /// Place the values in the argument list
    #[allow(unused)]
    fn return_or_store_to_arg<'w>(
        args: &mut Vec<Val>,
        wasm_alloc: Option<&'w WasmAlloc>,
        store: &mut Store<JavaState>,
    ) -> Result<Option<WasmSliceWrapper<'w>>, Error> {
        Ok(None)
    }

    fn get_return_by_ref_arg(_args: impl Iterator<Item = Val>) -> Option<i32> {
        None
    }

    /// Load from the argument list
    fn return_or_load_or_from_args(
        mut ret: Option<&Val>,
        _ret_by_ref_ptr: Option<WasmSliceWrapper<'_>>,
        _wasm_alloc: Option<&WasmAlloc>,
        _store: &mut Store<JavaState>,
    ) -> Result<Self, anyhow::Error> {
        ret.take()
            .cloned()
            .map(Self::from_val)
            .ok_or_else(|| anyhow!("Return Val not present"))
    }

    /// matches the arg tys
    fn matches_return_or_arg_tys(
        mut ret: Option<ValType>,
        _arg_tys: impl Iterator<Item = ValType>,
    ) -> Result<(), anyhow::Error> {
        let ty = ret
            .take()
            .ok_or_else(|| anyhow!("expected return type to compare"))?;

        ensure!(
            Self::matches_val_type(ty.clone()),
            "Expected {} but was {:?}",
            std::any::type_name::<Self>(),
            ty
        );

        Ok(())
    }
}

pub(crate) trait IntoValType {
    fn into_val_type() -> ValType;
}

pub(crate) trait FromVal: Sized {
    fn from_val(v: Val) -> Self;
}

pub(crate) trait IntoAbi {
    type Abi: Abi;

    fn into_abi(self) -> Self::Abi;
}

pub(crate) trait FromAbi<A: Abi> {
    unsafe fn from_abi(abi: A) -> Self;
}

pub(crate) trait MatchesValType {
    fn matches_val_type(ty: ValType) -> bool;
}

macro_rules! direct_complex_ty {
    ($t:ident, $v:path) => {
        impl Abi for $t {
            fn push_arg_tys(args: &mut Vec<ValType>) {
                args.push($v);
            }

            fn store_to_args(self, args: &mut Vec<Val>) {
                args.push(Val::from(self));
            }

            fn load_from_args(mut args: impl Iterator<Item = Val>) -> Result<Self, anyhow::Error> {
                let val = args
                    .next()
                    .ok_or_else(|| anyhow!("next argument missing, expected: {:?}", $v))?;
                val.$t().ok_or_else(|| {
                    anyhow!(
                        "incorrect value for argument, expected: {:?}, got: {:?}",
                        $v,
                        val
                    )
                })
            }

            fn matches_arg_tys(mut tys: impl Iterator<Item = ValType>) -> anyhow::Result<()> {
                let next = tys.next();
                ensure!(
                    next == Some($v),
                    "Expected ty for next arg: {:?} got: {:?}",
                    $v,
                    next
                );

                Ok(())
            }
        }

        impl IntoAbi for $t {
            type Abi = Self;

            fn into_abi<'a>(self) -> Self::Abi {
                self
            }
        }

        impl FromAbi<$t> for $t {
            unsafe fn from_abi<'a>(abi: Self) -> Self {
                abi
            }
        }

        impl IntoValType for $t {
            fn into_val_type() -> ValType {
                $v
            }
        }

        impl FromVal for $t {
            fn from_val(val: Val) -> Self {
                val.$t().expect("Should always succeed")
            }
        }

        impl MatchesValType for $t {
            fn matches_val_type(ty: ValType) -> bool {
                ty == $v
            }
        }

        impl ComplexTy for $t {
            type Abi = Self;

            fn compatible_with_store(&self, _store: &Store<JavaState>) -> bool {
                true
            }
        }
    };
}

direct_complex_ty!(i32, ValType::I32);
direct_complex_ty!(i64, ValType::I64);
direct_complex_ty!(f32, ValType::F32);
direct_complex_ty!(f64, ValType::F64);
