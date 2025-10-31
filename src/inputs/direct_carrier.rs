use crate::inputs::Metadata;

pub struct DirectCarrier<'a> {
    /// User phone number (primary payment identifier)
    pub phone: &'a str,
    /// Carrier-specific extensions
    pub metadata: Option<Metadata<'a>>,
}
