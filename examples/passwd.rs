fn main() {
  let password = b"admin";
  let salt = b"eternalnight";
  let config = argon2::Config {
    variant: argon2::Variant::Argon2i,
    version: argon2::Version::Version13,
    mem_cost: 1024, // 降低内存成本
    time_cost: 2,   // 降低时间成本
    lanes: 16,      // 增加并行度
    secret: &[],
    ad: &[],
    hash_length: 16, // 减少哈希长度
  };
  let hash = argon2::hash_encoded(password, salt, &config).unwrap();
  let matches = argon2::verify_encoded(&hash, password).unwrap();
  println!("hash -> {hash}");
  assert!(matches);
}
