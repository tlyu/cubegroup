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
                let rhs = rhs.as_ref();
                rhs.iter().fold(self, Mul::<&Turn>::mul)
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
