use anyhow::{anyhow, ensure};
use wasmtime::{Val, ValType, WeakStore};
pub(crate) trait ComplexTy {
    type Abi: Abi;

    fn compatible_with_store<'a>(&self, _store: WeakStore<'a>) -> bool;
}

pub(crate) trait Abi: Copy {
    fn push_arg_tys(args: &mut Vec<ValType>);
    fn store_to_args(self, args: &mut Vec<Val>);
    fn load_from_args(args: impl Iterator<Item = Val>) -> Result<Self, anyhow::Error>;
    fn matches_arg_tys(tys: impl Iterator<Item = ValType>) -> anyhow::Result<()>;
}

pub(crate) trait IntoAbi {
    type Abi: Abi;

    fn into_abi(self) -> Self::Abi;
}

pub(crate) trait FromAbi<A: Abi> {
    unsafe fn from_abi(abi: A) -> Self;
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

        impl ComplexTy for $t {
            type Abi = Self;

            fn compatible_with_store<'a>(&self, _store: WeakStore<'a>) -> bool {
                true
            }
        }
    };
}

direct_complex_ty!(i32, ValType::I32);
direct_complex_ty!(i64, ValType::I64);
direct_complex_ty!(f32, ValType::F32);
direct_complex_ty!(f64, ValType::F64);

// macro_rules! float_complex_ty {
//     ($t:ident, $a:ident, $v:path) => {
//         impl Abi for $a {
//             fn push_arg_tys(args: &mut Vec<ValType>) {
//                 args.push($v);
//             }

//             fn store_to_args(self, args: &mut Vec<Val>) {
//                 args.push(Val::from(self));
//             }

//             fn load_from_args(mut args: impl Iterator<Item = Val>) -> Result<Self, anyhow::Error> {
//                 let val = args
//                     .next()
//                     .ok_or_else(|| anyhow!("next argument missing, expected: {:?}", $v))?;
//                 val.$a().ok_or_else(|| {
//                     anyhow!(
//                         "incorrect value for argument, expected: {:?}, got: {:?}",
//                         $v,
//                         val
//                     )
//                 })
//             }

//             fn matches_arg_tys(mut tys: impl Iterator<Item = ValType>) -> anyhow::Result<()> {
//                 let next = tys.next();
//                 ensure!(
//                     next == Some($v),
//                     "Expected ty for next arg: {:?} got: {:?}",
//                     $v,
//                     next
//                 );

//                 Ok(())
//             }
//         }

//         impl IntoAbi for $t {
//             type Abi = $a;

//             fn into_abi<'a>(self) -> Self::Abi {
//                 self.to_bits()
//             }
//         }

//         impl FromAbi<$a> for $t {
//             unsafe fn from_abi<'a>(abi: $a) -> Self {
//                 $t::from_bits(abi)
//             }
//         }

//         impl ComplexTy for $t {
//             type Abi = $a;

//             fn compatible_with_store<'a>(&self, _store: WeakStore<'a>) -> bool {
//                 true
//             }
//         }
//     };
// }

// float_complex_ty!(f32, u32, ValType::F32);
// float_complex_ty!(f64, u64, ValType::F64);
