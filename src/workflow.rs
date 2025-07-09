use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Workflow {
    pub name: Option<String>,
    pub env: Option<std::collections::HashMap<String, String>>,
    pub jobs: std::collections::HashMap<String, Job>,
}

#[derive(Deserialize, Debug)]
pub struct Job {
    pub steps: Vec<Step>,
}

#[derive(Deserialize, Debug)]
pub struct Step {
    pub name: Option<String>,
    pub run: String,
}
