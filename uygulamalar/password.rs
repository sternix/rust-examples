// https://github.com/Sirneij/rust-auth/blob/main/backend/src/utils/auth/password.rs

use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

#[tracing::instrument(name = "Hashing user password", skip(password))]
pub async fn hash(password: &[u8]) -> String {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password, &salt)
        .expect("Unable to hash password.")
        .to_string()
}

#[tracing::instrument(name = "Verifying user password", skip(password, hash))]
pub fn verify_password(hash: &str, password: &[u8]) -> Result<(), argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(hash)?;
    Argon2::default().verify_password(password, &parsed_hash)
}
