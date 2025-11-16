use crate::*;

pub trait HasPlace {
    type Target: ?Sized;
}

/// Reborrow a subplace.
pub unsafe trait PlaceBorrow<'a, P: Projection, X>: HasPlace<Target = P::Source>
where
    X: HasPlace<Target = P::Target>,
{
    unsafe fn borrow(ptr: *const Self, p: &P) -> X;
}

/// Read a subplace.
pub unsafe trait PlaceRead<P: Projection>: HasPlace<Target = P::Source> {
    unsafe fn read(ptr: *const Self, p: &P) -> P::Target
    where
        P::Target: Sized;
}

/// Write to a subplace.
pub unsafe trait PlaceWrite<P: Projection>: HasPlace<Target = P::Source> {
    unsafe fn write(ptr: *mut Self, p: &P, x: P::Target);
}

/// Allows moving a value out of a subplace. This uses `PlaceRead::read` to read the value.
pub unsafe trait PlaceMove<P: Projection>: PlaceRead<P> {}

/// Allows dereferencing a subplace that contains a pointer. This piggy-backs on
/// `*const`-reborrowing. `PlaceRead` is insufficient since the pointer may not be `Copy`.
pub unsafe trait PlaceDeref<P: Projection>: HasPlace<Target = P::Source>
where
    P::Target: HasPlace,
    Self: for<'a> PlaceBorrow<'a, P, *const P::Target>,
{
}

/// Allows dropping a subplace. This piggy-backs on `*mut`-reborrowing.
pub unsafe trait PlaceDrop<P: Projection>: HasPlace<Target = P::Source>
where
    P::Target: HasPlace,
    Self: for<'a> PlaceBorrow<'a, P, *mut P::Target>,
{
}

/// Clean up a pointer whose contained place has been moved out of/dropped.
pub unsafe trait DropHusk: HasPlace {
    /// Drop the pointer but not the contents of the place (borrowck takes care of that).
    unsafe fn drop_husk(ptr: *mut Self);
}
