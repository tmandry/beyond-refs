//! Crate to experiment with the API proposed in
//! https://nadrieril.github.io/blog/2025/11/11/truly-first-class-custom-smart-pointers.html .
#![feature(ptr_metadata)]
use std::{
    mem::offset_of,
    ptr::{NonNull, Pointee},
};

mod basic_impls;
mod projection;
pub use projection::*;
mod place_ops;
pub use place_ops::*;

type Ptr<T> = NonNull<T>; // laziness
type Q<T> = NonNull<T>; // laziness
type R<T> = NonNull<T>; // laziness

struct A(u32);
struct Foo {
    a: A,
}

struct FooAProj;
impl Projection for FooAProj {
    type Source = Foo;
    type Target = A;
    fn offset(&self) -> usize {
        offset_of!(Foo, a)
    }
    fn project_metadata(
        &self,
        _: <Self::Source as Pointee>::Metadata,
    ) -> <Self::Target as Pointee>::Metadata {
    }
}

fn double_deref(p: Ptr<Q<Foo>>) {
    unsafe {
        // let a = @R (**p).a;
        let ptr: *const Q<Foo> = NoopProj::default().deref(&raw const p);
        let a: R<A> = FooAProj.borrow(ptr);
    }
}
