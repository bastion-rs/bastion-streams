use std::marker::PhantomData as marker;

#[derive(Copy, Clone)]
pub struct Inlet<'a, T> {
    pub id: usize,
    pub name: &'a str,
    _private: marker<T>,
}

impl<'a, T> Inlet<'a, T> {
    pub fn new(id: usize, name: &'a str) -> Self {
        Inlet {
            id,
            name,
            _private: marker,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Outlet<'a, T> {
    pub id: usize,
    pub name: &'a str,
    _private: marker<T>,
}

impl<'a, T> Outlet<'a, T> {
    pub fn new(id: usize, name: &'a str) -> Self {
        Outlet {
            id,
            name,
            _private: marker,
        }
    }
}

//impl<T> From<Outlet<T>> for Vec<_> {
//    fn from(out: Outlet<T>) -> Self {
//        CliError::IoError(error)
//    }
//}
