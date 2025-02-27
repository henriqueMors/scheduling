use regex::Regex;

/// Verifica se a senha é forte: mínimo 8 caracteres, 1 número, 1 maiúscula e 1 caractere especial.
pub fn is_strong_password(password: &str) -> bool {
    let re = Regex::new(r"^(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,}$").unwrap();
    re.is_match(password)
}
