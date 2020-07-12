/// A wrapper for a slice of references.
///
/// Available only through a (possibly mutable) reference.
///
/// Can be created from (a (possibly mutable) reference to) a slice 
/// of (possibly mutable) references by means of the `From` trait or 
/// with the help of [`new`](#method.new) and [`new_mut`](#method.new_mut) 
/// functions.
///
/// The current version of the crate provides only a minimal viable interface: 
/// the distributive laws and the [`modify`](#method.modify) method.
///
/// To get a concrete reference (or a sublice) out of the `Slice` 
/// you can write
///
/// ```
/// # use multiref::Slice;
/// let (mut a, mut b, mut c, mut d) = (1, 2, 3, 4);
/// let mut array = [&mut a, &mut b, &mut c, &mut d];
/// let slice = Slice::new_mut(&mut array[..]);
///
/// // Very clumsy but works!
/// *slice.as_mut()[0] = 4;
/// *((&mut slice.as_mut()[1..3])[0]) = 5;
///
/// // Continuation-passing style is a little more convenient:
/// let forty_two = slice.modify(|real_slice| { *real_slice[2] = 6; 42 });
/// assert!(forty_two == 42);
///
/// // Modifications can be chained:
/// slice.modify(|real_slice| {
///     *real_slice[3] += 1;
///     Slice::new_mut(real_slice)
/// }).modify(|real_slice| { 
///     *real_slice[3] += 2; 
/// });
///
/// assert!(a == 4);
/// assert!(b == 5);
/// assert!(c == 6);
/// assert!(d == 7);
/// ```
///
/// Next versions of the crate are expected to provide
/// an interface analogous to the one of standard slices (unfortunately, 
/// the lazy solution, i.e. implementing the `Deref` trait,
/// can't be used, because of the necessary &-head of the type). 
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

    /// Provides an access to the underlying slice of references via CPS.
    pub fn modify<R, F>(&'a mut self, f: F) -> R where
        F: FnOnce(&'a mut[&'a mut T]) -> R
    {
        f(self.as_mut())
    }
}
