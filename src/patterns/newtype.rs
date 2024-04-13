use std::fmt::Display;

pub struct Password(String);

impl Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "*".repeat(16))
    }
}

#[cfg(test)]
mod test {
    use crate::patterns::newtype::Password;

    #[test]
    fn test_password() {
        let unsecured_password = String::from("ThisIsMyPassword");
        let secured_password = Password(unsecured_password.clone());

        assert_eq!(format!("{secured_password}"), "*".repeat(16));
    }
}
