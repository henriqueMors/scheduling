use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{SaltString, PasswordHash};
use rand::rngs::OsRng; // Alterado de rand_core::OsRng para rand::rngs::OsRng
use rand::Rng;

/// Gera o hash de uma senha utilizando Argon2.
/// Retorna o hash como String ou um erro se houver problema.
pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    // Gera um salt de forma segura
    let salt = SaltString::generate(&mut OsRng);
    // Utiliza a configuração padrão do Argon2
    let argon2 = Argon2::default();
    // Gera o hash da senha, retornando-o como string
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?.to_string();
    Ok(password_hash)
}

/// Verifica se a senha fornecida corresponde ao hash armazenado.
pub fn verify_password(hash: &str, password: &str) -> bool {
    if let Ok(parsed_hash) = PasswordHash::new(hash) {
        Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok()
    } else {
        false
    }
}

/// Gera um código SMS aleatório de 6 dígitos.
pub fn generate_sms_code() -> String {
    let code: u32 = rand::thread_rng().gen_range(100000..1000000);
    code.to_string()
}

/// Simula o envio de SMS, imprimindo o código no console.
pub fn send_sms(phone: &str, code: &str) {
    println!("Enviando SMS para {}: Seu código é {}", phone, code);
}
