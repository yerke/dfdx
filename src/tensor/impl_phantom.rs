use crate::prelude::*;
use std::marker::PhantomData;

#[derive(Clone, Copy)]
pub struct PhantomTensor<T> {
    id: usize,
    marker: PhantomData<*const T>,
}

impl<T> HasUniqueId for PhantomTensor<T> {
    fn id(&self) -> usize {
        self.id
    }
}

impl<T: HasNdArray> HasNdArray for PhantomTensor<T> {
    type ArrayType = T::ArrayType;
    fn data(&self) -> &Self::ArrayType {
        todo!("remove this from HasNdArray")
    }
    fn mut_data(&mut self) -> &mut Self::ArrayType {
        todo!("remove this from HasNdArray")
    }
}

pub trait IntoPhantom: HasNdArray + Sized {
    fn phantom(&self) -> PhantomTensor<Self>;
}

macro_rules! tensor_impl {
    ($typename:ident, [$($Vs:tt),*]) => {
impl<$(const $Vs: usize, )* H> IntoPhantom for $typename<$($Vs, )* H> {
    fn phantom(&self) -> PhantomTensor<Self> {
        PhantomTensor { id: self.id, marker: PhantomData }
    }
}
    };
}

tensor_impl!(Tensor0D, []);
tensor_impl!(Tensor1D, [M]);
tensor_impl!(Tensor2D, [M, N]);
tensor_impl!(Tensor3D, [M, N, O]);
tensor_impl!(Tensor4D, [M, N, O, P]);
