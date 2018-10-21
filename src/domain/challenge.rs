#[derive(Serialize, Deserialize, Debug)]
pub struct Challenge {
    pub version: u32,
    pub challenge: String
}
