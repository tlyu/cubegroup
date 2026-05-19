macro_rules! gen_ops {
    ($t:ty) => {
        forward_ref_binop! {
            impl Mul for $t
        }
        forward_ref_binop! {
            impl Mul for $t, Turn
        }
        forward_ref_unop! {
            impl Not, not for $t
        }
        impl<T: AsRef<[Turn]>> Mul<T> for $t {
            type Output = $t;
            fn mul(self, rhs: T) -> Self::Output {
                let mut out = self;
                let rhs = rhs.as_ref();
                for t in rhs {
                    out = out * t;
                }
                out
            }
        }
        // Can't use forward_ref_binop here, because blanket impl conflicts
        impl<T: AsRef<[Turn]>> Mul<T> for &$t {
            type Output = $t;
            fn mul(self, rhs: T) -> Self::Output {
                *self * rhs
            }
        }
    }
}
