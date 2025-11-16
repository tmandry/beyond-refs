use crate::*;

pub trait HasPlace {
    type Target: ?Sized;
}

/// Borrow a subplace.
pub unsafe trait PlaceBorrow<'a, P: Projection, X>: HasPlace<Target = P::Source>
where
    X: HasPlace<Target = P::Target>,
{
    /// Tells the borrow-checker what other simultaneous and subsequent
    /// borrows are allowed.
    const BORROW_KIND: BorrowKind;

    unsafe fn borrow(ptr: *const Self, p: &P) -> X;
}

#[non_exhaustive]
pub enum BorrowKind {
    /// Other borrows are allowed (like `*mut T` and `RcRef<T>`).
    Untracked,
    /// Other `Shared` simultaneous borrows are allowed (like `&T`).
    Shared,
    /// No other simultaneous tracked borrows are allowed (like `&mut T`).
    Unique,
    /// No other simultaneous or subsequent borrows are allowed (like `&own T`).
    Owning,
    /// No other simultaneous tracked borrows are allowed and `drop` must be
    /// called before the underlying memory is reclaimed (like `&pin mut T`).
    UniquePinning,
    // maybe other things?
}

/// Read a value from a subplace.
pub unsafe trait PlaceRead<P: Projection>: HasPlace<Target = P::Source> {
    unsafe fn read(ptr: *const Self, p: &P) -> P::Target
    where
        P::Target: Sized;
}

/// Write to a subplace.
pub unsafe trait PlaceWrite<P: Projection>: HasPlace<Target = P::Source> {
    unsafe fn write(ptr: *mut Self, p: &P, x: P::Target)
    where
        P::Target: Sized;
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
