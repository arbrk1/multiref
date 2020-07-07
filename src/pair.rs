/// A wrapper for a pair of references.
///
///
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
    pub fn new<'x:'a>( from: &'a (&'x A, &'x B) ) -> &'a Self {
        from.into()
    }

    pub fn fst(&'a self) -> &'a A {
        unsafe { &*self.pair.0 }
    }
    
    pub fn snd(&'a self) -> &'a B {
        unsafe { &*self.pair.1 }
    }

    pub fn as_ref(&'a self) -> &'a (&'a A, &'a B) {
        unsafe { &*(self as *const _ as *const _) }
    }
    
    pub fn new_mut<'x:'a>( from: &'a mut (&'x mut A, &'x mut B) ) -> &'a mut Self {
        from.into()
    }

    pub fn fst_mut(&'a mut self) -> &'a mut A {
        unsafe { &mut *(self.pair.0 as *mut _) }
    }
    
    pub fn snd_mut(&'a mut self) -> &'a mut B {
        unsafe { &mut *(self.pair.1 as *mut _) }
    }

    pub fn as_mut(&'a mut self) -> &'a mut (&'a mut A, &'a mut B) {
        unsafe { &mut *(self as *mut _ as *mut _) }
    }
}





