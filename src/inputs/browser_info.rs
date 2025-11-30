use crate::enums::ColorDepth;

/// Browser information for 3D Secure authentication.
///
/// Per EMVCo 3DS specification, `accept_header` and `user_agent` are required.
/// Other fields are optional (may not be available on mobile devices).
///
/// ```skip
/// let browser_info = BrowserInfo {
///     accept_header: "text/html,application/xhtml+xml",
///     user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64)",
///     language: Some("en-US"),
///     color_depth: Some(ColorDepth::Bits24),
///     screen_height: Some(1080),
///     screen_width: Some(1920),
///     time_zone_offset: Some(-120),
///     java_enabled: Some(false),
///     javascript_enabled: Some(true),
/// }.try_into()?;
/// ```
pub struct BrowserInfo<'a> {
    /// HTTP Accept header value from the cardholder's browser.
    pub accept_header: &'a str,

    /// User-Agent header value from the cardholder's browser.
    pub user_agent: &'a str,

    /// Browser language as defined in IETF BCP 47 (e.g., "en-US", "pt-BR").
    pub language: Option<&'a str>,

    /// Screen color depth in bits per pixel.
    pub color_depth: Option<ColorDepth>,

    /// Screen height in pixels.
    pub screen_height: Option<u16>,

    /// Screen width in pixels.
    pub screen_width: Option<u16>,

    /// Time zone offset from UTC in minutes (negative for west of UTC).
    pub time_zone_offset: Option<i16>,

    /// Whether Java is enabled in the browser.
    pub java_enabled: Option<bool>,

    /// Whether JavaScript is enabled in the browser.
    pub javascript_enabled: Option<bool>,
}
