const CODE: u32 = 770;

#[derive(Serialize, Deserialize, Debug)]
pub struct PrivateMessageRequest {
    direction: u32,
    timestamp: u32,
    user: String,
    message: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PrivateMessageResponse {
    code: u32,
    user: String,
    message: String
}

impl PrivateMessageResponse {
    pub fn new(recipient: String, message: String) -> Self {
        Self {
            code: CODE,
            user: recipient,
            message
        }
    }
}
