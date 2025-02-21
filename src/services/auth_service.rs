use argon2::{self, Config};
use rand::Rng;

pub fn hash_password(password: &str) -> Result<String, argon2::Error> {
    let mut salt = [0u8; 16];
    rand::thread_rng().fill(&mut salt);
    let config = Config::default();
    argon2::hash_encoded(password.as_bytes(), &salt, &config)
}

// Já existente: verificação de senha e geração de SMS.
pub fn verify_password(hash: &str, password: &str) -> bool {
    use argon2::PasswordVerifier;
    if let Ok(parsed_hash) = argon2::PasswordHash::new(hash) {
        argon2::Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok()
    } else {
        false
    }
}

pub fn generate_sms_code() -> String {
    let code: u32 = rand::thread_rng().gen_range(100000..1000000);
    code.to_string()
}

pub fn send_sms(phone: &str, code: &str) {
    println!("Enviando SMS para {}: Seu código é {}", phone, code);
}
