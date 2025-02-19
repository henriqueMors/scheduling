use argon2::{self, Config};
use rand::Rng;

/// Verifica se a senha informada corresponde ao hash armazenado.
pub fn verify_password(hash: &str, password: &str) -> bool {
    argon2::verify_encoded(hash, password.as_bytes()).unwrap_or(false)
}

/// Gera um código SMS aleatório de 6 dígitos.
pub fn generate_sms_code() -> String {
    let code: u32 = rand::thread_rng().gen_range(100000..1000000);
    code.to_string()
}

/// Função para "enviar" o SMS (para testes, imprime no console; em produção, integre um serviço de SMS).
pub fn send_sms(phone: &str, code: &str) {
    println!("Enviando SMS para {}: Seu código é {}", phone, code);
}
