/// A wrapper for a pair of references.
///
/// Available only through a (possibly mutable) reference.
///
/// Can be created from (a (possibly mutable) reference to) a pair 
/// of (possibly mutable) references by means of the `From` trait or 
/// with the help of [`new`](#method.new) and [`new_mut`](#method.new_mut) 
/// functions.
///
/// Pairs can be combined to form longer tuples:
///
/// ```
/// # use multiref::Pair;
/// let (mut a, mut b, mut c) = (1, 2, 3);
/// let mut bc = (&mut b, &mut c);
/// let mut a_pbc = (&mut a, Pair::new_mut(&mut bc));
/// let pabc = Pair::new_mut(&mut a_pbc);
///
/// *pabc.snd_mut().snd_mut() = 4;
///
/// // an alternative (more clumsy) way to do the same (modulo indices) thing
/// *pabc.as_mut().1.as_mut().0 = 5;
///
/// // and in continuation-passing style:
/// pabc.modify(|a_pbc| { *a_pbc.0 = 6; });
///
/// assert!(a == 6);
/// assert!(b == 5);
/// assert!(c == 4);
/// ```
///
/// A solution facilitating working with more than two values is due to appear 
/// in one of the next versions of the crate.
#[repr(transparent)]
pub struct Pair<A: ?Sized, B: ?Sized> {
    pair: (*const A, *const B),
}

impl<'a, 'x: 'a, A, B> From<&'a (&'x A, &'x B)> for &'a Pair<A, B> where
    A: ?Sized,
    B: ?Sized,
{
    fn from(pair: &'a (&'x A, &'x B)) -> Self {
        unsafe { &*(pair as *const _ as *const _) }
    }
}

impl<'a, 'x: 'a, A, B> From<&'a mut (&'x mut A, &'x mut B)> for &'a mut Pair<A, B> where
    A: ?Sized,
    B: ?Sized,
{
    fn from(pair: &'a mut (&'x mut A, &'x mut B)) -> Self {
        unsafe { &mut *(pair as *mut _ as *mut _) }
    }
}


impl<'a, A: ?Sized, B: ?Sized> Pair<A, B> {
    /// The same as `pair_ref.into()`.
    pub fn new<'x:'a>( pair_ref: &'a (&'x A, &'x B) ) -> &'a Self {
        pair_ref.into()
    }

    /// The first component.
    pub fn fst(&'a self) -> &'a A {
        unsafe { &*self.pair.0 }
    }
    
    /// The second component.
    pub fn snd(&'a self) -> &'a B {
        unsafe { &*self.pair.1 }
    }

    /// Both components.
    ///
    /// A little more efficient than `(pair.fst(), pair.snd())` because it 
    /// returns the original place where the wrapped references are situated.
    pub fn as_ref(&'a self) -> &'a (&'a A, &'a B) {
        unsafe { &*(self as *const _ as *const _) }
    }
    
    /// The same as `pair_ref.into()`.
    pub fn new_mut<'x:'a>( pair_ref: &'a mut (&'x mut A, &'x mut B) ) -> &'a mut Self {
        pair_ref.into()
    }

    /// The first mutable component.
    pub fn fst_mut(&'a mut self) -> &'a mut A {
        unsafe { &mut *(self.pair.0 as *mut _) }
    }
    
    /// The second mutable component.
    pub fn snd_mut(&'a mut self) -> &'a mut B {
        unsafe { &mut *(self.pair.1 as *mut _) }
    }

    /// Both components.
    ///
    /// Obviously, you can't simply make `(pair.fst_mut(), pair.snd_mut())`.
    /// Thus this method is much more useful than [`as_ref`](#method.as_ref).
    pub fn as_mut(&'a mut self) -> &'a mut (&'a mut A, &'a mut B) {
        unsafe { &mut *(self as *mut _ as *mut _) }
    }

    /// Provides an access to the underlying pair of references via CPS.
    pub fn modify<R, F>(&'a mut self, f: F) -> R where
        F: FnOnce(&'a mut (&'a mut A, &'a mut B)) -> R
    {
        f(self.as_mut())
    }
}
