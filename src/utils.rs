use once_cell::sync::Lazy;
use regex::Regex;

/// 🔹 Regex compilada apenas uma vez → Melhor performance
static PASSWORD_REGEX: Lazy<Regex> = Lazy::new(|| {
    // Regex para verificar se a senha tem pelo menos uma letra maiúscula, número, caractere especial, e pelo menos 8 caracteres
    Regex::new(r"^(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,}$").unwrap()
});

/// ✅ Verifica se a senha é forte:
/// - Mínimo de 8 caracteres  
/// - Pelo menos 1 letra maiúscula  
/// - Pelo menos 1 número  
/// - Pelo menos 1 caractere especial (@, $, !, %, *, ?, &)  
pub fn is_strong_password(password: &str) -> bool {
    // Removendo look-ahead e implementando manualmente as verificações
    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_digit = password.chars().any(|c| c.is_digit(10));
    let has_special = password.chars().any(|c| r"@$!%*?&".contains(c));
    let min_length = password.len() >= 8;

    has_uppercase && has_digit && has_special && min_length
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_strong_password() {
        assert!(is_strong_password("Test@123")); // ✅ Forte
        assert!(is_strong_password("Password1$")); // ✅ Forte
        assert!(!is_strong_password("weakpass")); // ❌ Falha (sem número, caractere especial e maiúscula)
        assert!(!is_strong_password("12345678")); // ❌ Falha (sem maiúscula, caractere especial)
        assert!(!is_strong_password("NOLOWERCASE123!")); // ❌ Falha (sem letra minúscula)
    }
}
