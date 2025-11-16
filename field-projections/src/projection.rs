use std::{marker::PhantomData, ptr::Pointee};

use crate::*;

pub trait Projection: Sized {
    type Source: ?Sized;
    type Target: ?Sized;

    fn offset(&self) -> usize;
    fn project_metadata(
        &self,
        src_meta: <Self::Source as Pointee>::Metadata,
    ) -> <Self::Target as Pointee>::Metadata;

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
    /// Convenience method that simply calls the corresponding PlaceDeref method.
    unsafe fn deref<X>(&self, ptr: *const X) -> *const Self::Target
    where
        X: HasPlace<Target = Self::Source>,
        X: PlaceDeref<Self>,
        Self::Target: HasPlace,
    {
        unsafe { self.borrow(ptr) }
    }

    // TODO: compose
    // TODO: dyn-compat
    // TODO: inspect projections
    // fn compose<Q>(&self, other: Q) -> impl Projection<Source=Self::Source, Target=Q::Target>
    // where Q: Projection<Source=Self::Target>;
}

pub struct NoopProj<T>(PhantomData<T>);
impl<T> Default for NoopProj<T> {
    fn default() -> Self {
        Self(Default::default())
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
