use sha2::{ Sha256, Digest };

const CODE: u32 = 2;
const MASK: u32 = 255;

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginRequest {
    pub success: bool,
    pub message: String,
    pub challenge: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginResponse {
    code: u32,
    algorithm: String,
    challenge: String,
    mask: u32
}

impl LoginResponse {
    pub fn new(password: String, challenge: String) -> Self {
        let mut hasher = Sha256::default();

        hasher.input(challenge.as_bytes());
        hasher.input(password.as_bytes());

        let result = hasher.result();

        LoginResponse {
            code: CODE,
            algorithm: "SHA256".into(),
            challenge: format!("{:x}", result),
            mask: MASK
        }
    }
}
