use crate::Error;

/// Data for a redirect action.
///
/// Contains URLs for redirecting the customer to an external service
/// and returning them after the action is completed.
#[derive(Clone, Debug)]
pub struct Redirect {
    url: String,
    return_url: String,
}

impl Redirect {
    /// The URL to redirect the customer to.
    #[inline]
    pub fn url(&self) -> &str {
        &self.url
    }

    /// The URL to return to after the customer completes the action.
    #[inline]
    pub fn return_url(&self) -> &str {
        &self.return_url
    }
}

impl<'a> TryFrom<crate::RequiredAction<'a>> for Redirect {
    type Error = Error;

    fn try_from(input: crate::RequiredAction<'a>) -> Result<Self, Self::Error> {
        match input {
            crate::RequiredAction::Redirect { url, return_url } => Ok(Self {
                url: url.to_string(),
                return_url: return_url.to_string(),
            }),
            #[allow(unreachable_patterns)]
            _ => Err(Error::InvalidInput(String::from(
                "invalid required action type",
            ))),
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
    fn constructs_from_required_action() {
        let input = valid_input();
        let redirect = Redirect::try_from(input).unwrap();

        assert_eq!(redirect.url(), "https://gateway.example.com/auth/123");
        assert_eq!(
            redirect.return_url(),
            "https://merchant.example.com/callback"
        );
    }
}
