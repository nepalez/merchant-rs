use std::marker::PhantomData;

/// The data for completing an external payment.
#[derive(Default)]
pub struct ExternalPaymentData<'a> {
    _marker: PhantomData<&'a ()>,
}
