use argon2::{Argon2, PasswordHash, PasswordVerifier};
use rand::Rng;

pub fn verify_password(hash: &str, password: &str) -> bool {
    // Tenta analisar o hash fornecido
    if let Ok(parsed_hash) = PasswordHash::new(hash) {
        // Verifica a senha usando o Argon2 padrão
        Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok()
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
