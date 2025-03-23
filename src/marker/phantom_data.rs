use std::marker;
/// [marker::PhantomData]
pub struct PhantomData<T: ?Sized> {
    _data: marker::PhantomData<T>,
}
