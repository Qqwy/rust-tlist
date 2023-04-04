use typenum::{Unsigned, Bit, UInt, UTerm, B0, B1};
pub trait BitExt: Bit {
    type UIntSucc<U: UnsignedExt>: UnsignedExt;
}

impl BitExt for B0 {
    type UIntSucc<U: UnsignedExt> = UInt<U, B1>;
}

impl BitExt for B1 {
    type UIntSucc<U: UnsignedExt> = UInt<U::Succ, B0>;
}

pub trait UnsignedExt: Unsigned {
    type Succ: UnsignedExt;
}

impl UnsignedExt for UTerm {
    type Succ = UInt<UTerm, B1>;
}

impl<U: UnsignedExt, B: BitExt> UnsignedExt for UInt<U, B> {
    type Succ = B::UIntSucc<U>;
}
