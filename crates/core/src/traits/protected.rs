use crate::internal::Exposed;

pub trait Protected
where
    Self: Exposed,
{
    /// Exposes the sensitive data as references to zeroized values.
    ///
    /// It enforces the use of a closure to control the lifetime
    /// of the returned value and thus minimize the security risks.
    ///
    /// # Safety
    ///
    /// The method is **unsafe** because it exposes secrets
    /// (either PII or SAD) as references to zeroized values.
    /// Those must be used with special care to leaking prevention.
    ///
    /// Caller MUST ensure they are in a PCI-compliant environment where
    /// exposed sensitive data will be:
    /// 1. Used only for immediate gateway communication
    ///    or for immediate processing of gateway responses;
    /// 2. Not logged, stored, or transmitted insecurely.
    /// 3. When copied to other values (like requests),
    ///    the allocated memory is dropped after use.
    unsafe fn expose_secrets<'a, T, F>(&'a self, f: F) -> T
    where
        F: FnOnce(<Self as Exposed>::Output<'a>) -> T,
    {
        f(self.expose())
    }
}
