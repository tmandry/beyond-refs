use crate::*;

impl<T: ?Sized> HasPlace for *const T {
    type Target = T;
}
impl<T: ?Sized> HasPlace for *mut T {
    type Target = T;
}
impl<T: ?Sized> HasPlace for NonNull<T> {
    type Target = T;
}

// The two basic impls everything else is derived from.
unsafe impl<'a, P: Projection> PlaceBorrow<'a, P, *const P::Target> for *const P::Source {
    unsafe fn borrow(ptr: *const Self, p: &P) -> *const P::Target {
        unsafe {
            let (ptr, meta) = (*ptr).to_raw_parts();
            let ptr = ptr.offset(p.offset() as isize);
            let meta = p.project_metadata(meta);
            std::ptr::from_raw_parts(ptr, meta)
        }
    }
}
unsafe impl<'a, P: Projection> PlaceBorrow<'a, P, *mut P::Target> for *mut P::Source {
    unsafe fn borrow(ptr: *const Self, p: &P) -> *mut P::Target {
        unsafe {
            let (ptr, meta) = (*ptr).to_raw_parts();
            let ptr = ptr.offset(p.offset() as isize);
            let meta = p.project_metadata(meta);
            std::ptr::from_raw_parts_mut(ptr, meta)
        }
    }
}

unsafe impl<'a, P: Projection> PlaceBorrow<'a, P, NonNull<P::Target>> for NonNull<P::Source> {
    unsafe fn borrow(ptr: *const Self, p: &P) -> NonNull<P::Target> {
        unsafe {
            let ptr: *mut _ = (*ptr).as_ptr();
            NonNull::new_unchecked(p.borrow::<*mut _, *mut _>(&raw const ptr))
        }
    }
}
unsafe impl<'a, P: Projection> PlaceBorrow<'a, P, *const P::Target> for NonNull<P::Source> {
    unsafe fn borrow(ptr: *const Self, p: &P) -> *const P::Target {
        unsafe { p.borrow::<NonNull<_>, NonNull<_>>(ptr).as_ptr() }
    }
}

unsafe impl<P: Projection> PlaceDeref<P> for NonNull<P::Source> where P::Target: HasPlace {}
