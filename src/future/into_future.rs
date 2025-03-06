/// [std::future::IntoFuture]
pub trait IntoFuture {
    /// [std::future::IntoFuture::Output]
    type Output;
    /// [std::future::IntoFuture::IntoFuture]
    type IntoFuture: Future<Output = Self::Output>;
    /// [std::future::IntoFuture::into_future]
    fn into_future(self) -> Self::IntoFuture;
}
