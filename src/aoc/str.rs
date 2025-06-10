use md5;

pub fn md5_hash(word: &String) -> String {
    let digest = md5::compute(word);
    format!("{:x}", digest)
}