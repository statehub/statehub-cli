#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Helm {
    pub repo: String,
    pub chart: String,
    pub version: String,
    pub parameters: HashMap<String, String>,
}
