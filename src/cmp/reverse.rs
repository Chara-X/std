use super::*;
/// [std::cmp::Reverse]
pub struct Reverse<T>(pub T);
impl<T> PartialEq for Reverse<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}
impl<T> Eq for Reverse<T> where T: Eq {}
impl<T> PartialOrd for Reverse<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.0.partial_cmp(&self.0)
    }
}
impl<T> Ord for Reverse<T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0)
    }
}
