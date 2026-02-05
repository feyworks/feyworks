use crate::{
    Num, impl_approx, impl_bytemuck, impl_casts, impl_neg, impl_op, impl_op_scalar, impl_serde,
    impl_tuple_arr,
};

pub type ProjectionF = Projection<f32>;

/// Represents the projection of a 2D shape on an axis.
///
/// This primitive does not contain the axis itself, merely
/// the start and end bounds of the projection. This is used
/// in overlap checks for convex shapes.
#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Projection<T> {
    pub min: T,
    pub max: T,
}

impl_bytemuck!(Projection);

impl_tuple_arr!(
    NAME = Projection
    LEN = 2
    FIELDS = (min, max)
    TUPLE = (T, T)
);

impl_approx!(
    NAME = Projection
    FIELDS = (min, max)
);

impl_serde!(
    NAME = Projection
    FIELDS = (min, max)
);

impl_casts!(
    NAME = Projection
    FIELDS = (min, max)
);

impl_op!(Projection Add add AddAssign add_assign min max);
impl_op!(Projection Sub sub SubAssign sub_assign min max);
impl_op!(Projection Mul mul MulAssign mul_assign min max);
impl_op!(Projection Div div DivAssign div_assign min max);
impl_op!(Projection Rem rem RemAssign rem_assign min max);
impl_op_scalar!(Projection Add add AddAssign add_assign min max);
impl_op_scalar!(Projection Sub sub SubAssign sub_assign min max);
impl_op_scalar!(Projection Mul mul MulAssign mul_assign min max);
impl_op_scalar!(Projection Div div DivAssign div_assign min max);
impl_op_scalar!(Projection Rem rem RemAssign rem_assign min max);
impl_neg!(Projection min max);

impl<T> Projection<T> {
    /// Create a new projection.
    #[inline]
    pub const fn new(min: T, max: T) -> Self {
        Self { min, max }
    }
}

/// Create a new projection.
#[inline]
pub const fn projection<T>(min: T, max: T) -> Projection<T> {
    Projection { min, max }
}

impl<T: Num> Projection<T> {
    /// A projection of range `[0, 0]`.
    pub const ZERO: Self = Self::new(T::ZERO, T::ZERO);

    #[inline]
    pub fn contains(&self, value: T) -> bool {
        value >= self.min && value <= self.max
    }

    /// Returns true if this projection overlaps the other.
    #[inline]
    pub fn overlaps(&self, other: Projection<T>) -> bool {
        self.min < other.max && self.max > other.min
    }

    /// If this projection overlaps the other, returns the amount
    /// which it overlaps.
    #[inline]
    pub fn overlap(&self, other: Projection<T>) -> Option<T> {
        (self.min < other.max && self.max > other.min).then(|| self.max - other.min)
    }

    /// Length of the projection.
    #[inline]
    pub fn len(&self) -> T {
        self.max - self.min
    }
}
