pub fn hash_password(password: &str) -> String {
    use sha2::Digest;

    let mut hasher = sha2::Sha256::new();
    hasher.update(password);

    format!("{:X}", hasher.finalize())
}
