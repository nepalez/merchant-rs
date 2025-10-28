/// Provide an unsafe version of the standard `AsRef` trait
/// to inform the developer that he/she deals with the highly sensitive data.
#[allow(dead_code)]
pub trait AsUnsafeRef<T: ?Sized> {
    /// Exposes the sensitive value as a reference to a content of the safe container.
    ///
    /// # SAFETY
    /// When using the method, a developer must ensure that:
    /// 1. The value is not leaking into logs, error messages, etc.;
    /// 2. It is not cloned nor copied outside safe containers that zeroize memory upon a drop.
    unsafe fn as_ref(&self) -> &T;
}
