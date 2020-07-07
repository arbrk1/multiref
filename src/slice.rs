/// A wrapper for a slice of references.
///
/// Available only through a (possibly mutable) reference.
///
/// Can be created from (a (possibly mutable) reference to) a slice 
/// of (possibly mutable) references by means of the `From` trait or 
/// with the help of [`new`](#method.new) and [`new_mut`](#method.new_mut) 
/// functions.
///
/// The current version of the crate provides only the distributive laws.
/// Thus to get a concrete reference (or a sublice) out of the `Slice` 
/// one has to write
///
/// ```
/// # use multiref::Slice;
/// let (mut a, mut b, mut c) = (1, 2, 3);
/// let mut array = [&mut a, &mut b, &mut c];
/// let slice = Slice::new_mut(&mut array[..]);
///
/// // Very clumsy but works!
/// *slice.as_mut()[0] = 4;
/// *((&mut slice.as_mut()[1..3])[0]) = 5;
///
/// assert!(a == 4);
/// assert!(b == 5);
/// assert!(c == 3);
/// ```
///
/// Next versions of the crate are expected to fix this by providing 
/// an interface analogous to the slice one (unfortunately, `Deref` 
/// can't be implemented, because of the necessary &-head of the type). 
#[repr(transparent)]
pub struct Slice<T: ?Sized> {
    _slice: [*const T],
}

impl<'a, 'x: 'a, T> From<&'a [&'x T]> for &'a Slice<T> where
    T: ?Sized,
{
    fn from(slice: &'a [&'x T]) -> Self {
        unsafe { &*(slice as *const _ as *const _) }
    }
}

impl<'a, 'x: 'a, T> From<&'a mut [&'x mut T]> for &'a mut Slice<T> where
    T: ?Sized,
{
    fn from(slice: &'a mut [&'x mut T]) -> Self {
        unsafe { &mut *(slice as *mut _ as *mut _) }
    }
}

impl<'a, T: ?Sized> Slice<T> {
    /// The same as `slice.into()`.
    pub fn new<'x:'a>( slice: &'a [&'x T] ) -> &'a Self {
        slice.into()
    }

    /// The original slice.
    pub fn as_ref(&'a self) -> &'a [&'a T] {
        unsafe { &*(self as *const _ as *const _) }
    }
    
    /// The same as `pair_ref.into()`.
    pub fn new_mut<'x:'a>( slice: &'a mut [&'x mut T] ) -> &'a mut Self {
        slice.into()
    }

    /// The original slice, mutable version.
    pub fn as_mut(&'a mut self) -> &'a mut [&'a mut T] {
        unsafe { &mut *(self as *mut _ as *mut _) }
    }
}
