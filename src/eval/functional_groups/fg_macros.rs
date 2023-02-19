macro_rules! fg {
    ($T:ty, $field:tt) => {
        use crate::eval::traits::Weighable;
        use std::ops::{Deref, DerefMut};

        impl Deref for $T {
            type Target = Atoms;

            fn deref(&self) -> &Self::Target {
                &self.$field
            }
        }

        impl DerefMut for $T {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$field
            }
        }

        impl Weighable for $T {
            fn atomic_numbers(&self) -> i64 {
                self.atoms()
                    .neighbors(self.head)
                    .map(|neighbor| self.atoms().node_weight(neighbor).unwrap().atomic_numbers())
                    .sum()
            }
        }

        impl Default for $T {
            fn default() -> Self {
                Self::new()
            }
        }
    };
    ($T:ty) => {
        use crate::eval::traits::Weighable;
        use std::ops::{Deref, DerefMut};

        impl Deref for $T {
            type Target = Atoms;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl DerefMut for $T {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl Weighable for $T {
            fn atomic_numbers(&self) -> i64 {
                self.atoms()
                    .neighbors(self.head)
                    .map(|neighbor| self.atoms().node_weight(neighbor).unwrap().atomic_numbers())
                    .sum()
            }
        }

        impl Default for $T {
            fn default() -> Self {
                Self::new()
            }
        }
    };
}

pub(crate) use fg;
