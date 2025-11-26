mod redirect;

use crate::Error;

pub use redirect::Redirect;

// TODO: Add more variants (Iframe, QRCode, Voucher, BankTransfer, etc.)
/// Customer action required to complete the authorization.
pub enum RequiredAction {
    /// Redirect the customer to an external URL for approval.
    Redirect(Redirect),
}

impl<'a> TryFrom<crate::RequiredAction<'a>> for RequiredAction {
    type Error = Error;

    fn try_from(input: crate::RequiredAction<'a>) -> Result<Self, Self::Error> {
        match input {
            crate::RequiredAction::Redirect { .. } => Ok(Self::Redirect(input.try_into()?)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_input() -> crate::RequiredAction<'static> {
        crate::RequiredAction::Redirect {
            url: "https://gateway.example.com/auth/123",
            return_url: "https://merchant.example.com/callback",
        }
    }

    #[test]
    fn constructs_redirect_variant() {
        let input = valid_input();
        let action = RequiredAction::try_from(input).unwrap();

        match action {
            RequiredAction::Redirect(redirect) => {
                assert_eq!(redirect.url(), "https://gateway.example.com/auth/123");
                assert_eq!(
                    redirect.return_url(),
                    "https://merchant.example.com/callback"
                );
            }
        }
    }
}
