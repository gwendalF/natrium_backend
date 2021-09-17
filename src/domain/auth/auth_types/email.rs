use lazy_static::lazy_static;
use regex::Regex;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EmailError {
    #[error("Invalid email")]
    InvalidEmail,
    #[error("Email already exist")]
    AlreadyUsedEmail,
}

#[derive(Debug, PartialEq, Eq)]
pub struct EmailAddress(String);

fn validate(email: String) -> Result<String, EmailError> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r#"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#
        )
        .unwrap();
    }
    if RE.is_match(&email) {
        Ok(email)
    } else {
        Err(EmailError::InvalidEmail)
    }
}

impl EmailAddress {
    pub fn new(email_string: String) -> Result<EmailAddress, EmailError> {
        Ok(EmailAddress(validate(email_string)?))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    #[rstest]
    #[case("wrong@.com", false)]
    #[case("good@google.com", true)]
    fn test_email(#[case] email_str: &str, #[case] is_ok: bool) {
        let email_address = EmailAddress::new(email_str.to_owned());
        assert_eq!(email_address.is_ok(), is_ok);
    }
}
