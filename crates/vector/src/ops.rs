//! Overloaded operators for a `Vector`.

use std::ops;

use crate::Vector;

macro_rules! impl_add {
    ($lhs:ty, $rhs:ty, $output:ty) => {
        impl ops::Add<$rhs> for $lhs {
            type Output = $output;

            fn add(self, other: $rhs) -> Self::Output {
                Self::Output {
                    x: self.x + other.x,
                    y: self.y + other.y,
                }
            }
        }
    };
}

impl_add!(Vector, Vector, Vector);
impl_add!(Vector, &Vector, Vector);
impl_add!(&Vector, &Vector, Vector);

macro_rules! impl_sub {
    ($lhs:ty, $rhs:ty, $output:ty) => {
        impl ops::Sub<$rhs> for $lhs {
            type Output = $output;

            fn sub(self, other: $rhs) -> Self::Output {
                Self::Output {
                    x: self.x - other.x,
                    y: self.y - other.y,
                }
            }
        }
    };
}

impl_sub!(Vector, Vector, Vector);
impl_sub!(Vector, &Vector, Vector);
impl_sub!(&Vector, &Vector, Vector);

macro_rules! impl_mul {
    ($lhs:ty, $rhs:ty, $output:ty) => {
        impl ops::Mul<$rhs> for $lhs {
            type Output = $output;

            fn mul(self, other: $rhs) -> Self::Output {
                Self::Output {
                    x: self.x * other,
                    y: self.y * other,
                }
            }
        }
    };
}

impl_mul!(Vector, i64, Vector);
impl_mul!(Vector, &i64, Vector);
impl_mul!(&Vector, i64, Vector);
impl_mul!(&Vector, &i64, Vector);

macro_rules! impl_add_assign {
    ($self:ty, $other:ty) => {
        impl ops::AddAssign<$other> for $self {
            fn add_assign(&mut self, other: $other) {
                self.x += other.x;
                self.y += other.y;
            }
        }
    };
}

impl_add_assign!(Vector, Vector);
impl_add_assign!(Vector, &Vector);

macro_rules! impl_sub_assign {
    ($self:ty, $other:ty) => {
        impl ops::SubAssign<$other> for $self {
            fn sub_assign(&mut self, other: $other) {
                self.x -= other.x;
                self.y -= other.y;
            }
        }
    };
}

impl_sub_assign!(Vector, Vector);
impl_sub_assign!(Vector, &Vector);
