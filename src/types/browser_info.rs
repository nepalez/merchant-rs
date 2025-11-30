use crate::Error;
use crate::enums::ColorDepth;
use crate::types::Language;

/// Browser information for 3D Secure authentication.
///
/// Per EMVCo 3DS specification, `accept_header` and `user_agent` are required.
/// Other fields are optional (may not be available on mobile devices).
///
/// # Data Protection
/// Browser information is NOT considered PII in any reasonable context,
/// as it represents generic device characteristics that cannot identify individuals.
///
/// Consequently, both `Debug` and `AsRef` are implemented without masking.
#[derive(Clone, Debug)]
pub struct BrowserInfo {
    pub(crate) accept_header: String,
    pub(crate) user_agent: String,
    pub(crate) language: Option<Language>,
    pub(crate) color_depth: Option<ColorDepth>,
    pub(crate) screen_height: Option<u16>,
    pub(crate) screen_width: Option<u16>,
    pub(crate) time_zone_offset: Option<i16>,
    pub(crate) java_enabled: Option<bool>,
    pub(crate) javascript_enabled: Option<bool>,
}

impl BrowserInfo {
    /// HTTP Accept header value from the cardholder's browser.
    #[inline]
    pub fn accept_header(&self) -> &str {
        &self.accept_header
    }

    /// User-Agent header value from the cardholder's browser.
    #[inline]
    pub fn user_agent(&self) -> &str {
        &self.user_agent
    }

    /// Browser language as defined in IETF BCP 47.
    #[inline]
    pub fn language(&self) -> Option<&Language> {
        self.language.as_ref()
    }

    /// Screen color depth in bits per pixel.
    #[inline]
    pub fn color_depth(&self) -> Option<ColorDepth> {
        self.color_depth
    }

    /// Screen height in pixels.
    #[inline]
    pub fn screen_height(&self) -> Option<u16> {
        self.screen_height
    }

    /// Screen width in pixels.
    #[inline]
    pub fn screen_width(&self) -> Option<u16> {
        self.screen_width
    }

    /// Time zone offset from UTC in minutes (negative for west of UTC).
    #[inline]
    pub fn time_zone_offset(&self) -> Option<i16> {
        self.time_zone_offset
    }

    /// Whether Java is enabled in the browser.
    #[inline]
    pub fn java_enabled(&self) -> Option<bool> {
        self.java_enabled
    }

    /// Whether JavaScript is enabled in the browser.
    #[inline]
    pub fn javascript_enabled(&self) -> Option<bool> {
        self.javascript_enabled
    }
}

impl TryFrom<crate::inputs::BrowserInfo<'_>> for BrowserInfo {
    type Error = Error;

    fn try_from(input: crate::inputs::BrowserInfo<'_>) -> Result<Self, Self::Error> {
        Ok(Self {
            accept_header: input.accept_header.to_string(),
            user_agent: input.user_agent.to_string(),
            language: input.language.map(Language::try_from).transpose()?,
            color_depth: input.color_depth,
            screen_height: input.screen_height,
            screen_width: input.screen_width,
            time_zone_offset: input.time_zone_offset,
            java_enabled: input.java_enabled,
            javascript_enabled: input.javascript_enabled,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inputs;

    fn valid_input() -> inputs::BrowserInfo<'static> {
        inputs::BrowserInfo {
            accept_header: "text/html,application/xhtml+xml",
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64)",
            language: Some("en-US"),
            color_depth: Some(ColorDepth::Bits24),
            screen_height: Some(1080),
            screen_width: Some(1920),
            time_zone_offset: Some(-120),
            java_enabled: Some(false),
            javascript_enabled: Some(true),
        }
    }

    #[test]
    fn constructed_from_valid_input() {
        let input = valid_input();
        let info = BrowserInfo::try_from(input).unwrap();

        assert_eq!(info.accept_header(), "text/html,application/xhtml+xml");
        assert_eq!(
            info.user_agent(),
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64)"
        );
        assert_eq!(info.language().map(|l| l.as_ref()), Some("en-US"));
        assert_eq!(info.color_depth(), Some(ColorDepth::Bits24));
        assert_eq!(info.screen_height(), Some(1080));
        assert_eq!(info.screen_width(), Some(1920));
        assert_eq!(info.time_zone_offset(), Some(-120));
        assert_eq!(info.java_enabled(), Some(false));
        assert_eq!(info.javascript_enabled(), Some(true));
    }

    #[test]
    fn constructed_with_optional_fields_none() {
        let input = inputs::BrowserInfo {
            accept_header: "text/html",
            user_agent: "Mozilla/5.0",
            language: None,
            color_depth: None,
            screen_height: None,
            screen_width: None,
            time_zone_offset: None,
            java_enabled: None,
            javascript_enabled: None,
        };
        let info = BrowserInfo::try_from(input).unwrap();

        assert_eq!(info.accept_header(), "text/html");
        assert_eq!(info.user_agent(), "Mozilla/5.0");
        assert!(info.language().is_none());
        assert!(info.color_depth().is_none());
    }

    #[test]
    fn rejects_invalid_language() {
        let mut input = valid_input();
        input.language = Some("invalid");

        let result = BrowserInfo::try_from(input);
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }
}
