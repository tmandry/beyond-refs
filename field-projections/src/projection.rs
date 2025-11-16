use std::{marker::PhantomData, ptr::Pointee};

use crate::*;

// TODO: inspect projections
pub trait Projection: Sized {
    type Source: ?Sized;
    type Target: ?Sized;

    fn offset(&self) -> usize;
    fn project_metadata(
        &self,
        meta: <Self::Source as Pointee>::Metadata,
    ) -> <Self::Target as Pointee>::Metadata;
}

/// Extension trait so that `Projection` stays dyn-compatible.
impl<P: Projection> ProjectionExt for P {}
pub trait ProjectionExt: Projection {
    /// Convenience method that simply calls the corresponding PlaceBorrow method.
    unsafe fn borrow<'a, X, Y>(&self, ptr: *const X) -> Y
    where
        X: HasPlace<Target = Self::Source>,
        Y: HasPlace<Target = Self::Target>,
        X: PlaceBorrow<'a, Self, Y>,
    {
        unsafe { PlaceBorrow::borrow(ptr, self) }
    }
    /// Convenience method that simply calls the corresponding PlaceRead method.
    unsafe fn read<X>(&self, ptr: *const X) -> Self::Target
    where
        X: PlaceRead<Self>,
        Self::Target: Sized,
    {
        unsafe { PlaceRead::read(ptr, self) }
    }
    /// Convenience method that simply calls the corresponding PlaceRead method.
    unsafe fn write<X>(&self, ptr: *const X, val: Self::Target)
    where
        X: PlaceWrite<Self>,
        Self::Target: Sized,
    {
        unsafe { PlaceWrite::write(ptr.cast_mut(), self, val) }
    }
    /// Convenience method that simply calls the corresponding PlaceDeref method.
    unsafe fn deref<X>(&self, ptr: *const X) -> *const Self::Target
    where
        X: HasPlace<Target = Self::Source>,
        X: PlaceDeref<Self>,
        Self::Target: HasPlace,
    {
        unsafe { self.borrow(ptr) }
    }

    fn compose<Q>(self, other: Q) -> ComposeProj<Self, Q>
    where
        Q: Projection<Source = Self::Target>,
    {
        ComposeProj(self, other)
    }
}

pub struct NoopProj<T>(PhantomData<T>);
impl<T> Default for NoopProj<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}
impl<T> Clone for NoopProj<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<T> Projection for NoopProj<T> {
    type Source = T;
    type Target = T;
    fn offset(&self) -> usize {
        0
    }
    fn project_metadata(
        &self,
        m: <Self::Source as Pointee>::Metadata,
    ) -> <Self::Target as Pointee>::Metadata {
        m
    }
}

/// Projection `P` followed by `Q`.
#[derive(Clone)]
pub struct ComposeProj<P, Q>(P, Q);
impl<P, Q> Projection for ComposeProj<P, Q>
where
    P: Projection,
    Q: Projection<Source = P::Target>,
{
    type Source = P::Source;
    type Target = Q::Target;
    fn offset(&self) -> usize {
        self.0.offset() + self.1.offset()
    }
    fn project_metadata(
        &self,
        meta: <Self::Source as Pointee>::Metadata,
    ) -> <Self::Target as Pointee>::Metadata {
        self.1.project_metadata(self.0.project_metadata(meta))
    }
}
