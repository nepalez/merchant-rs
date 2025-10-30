use std::marker::PhantomData;

/// The data for completing an external payment.
pub struct ExternalPaymentData<'a> {
    _marker: PhantomData<&'a ()>,
}
