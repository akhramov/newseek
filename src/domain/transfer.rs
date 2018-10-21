#[derive(Serialize, Deserialize, Debug)]
pub struct Transfer {
    upload: bool,
    user: String,
    path: String,
    place: u32,
    state: u32,
    error: String,
    progress: u64,
    file_size: u64,
    rate: u32
}
