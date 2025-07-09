use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Workflow {
    pub name: Option<String>,
    pub env: Option<HashMap<String, String>>,
    pub jobs: HashMap<String, Job>,
}

#[derive(Deserialize, Debug)]
pub struct Job {
    pub steps: Vec<Step>,
    pub env: Option<HashMap<String, String>>,
    pub working_directory: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Step {
    pub name: Option<String>,
    pub run: String,
    pub working_directory: Option<String>,
    pub env: Option<HashMap<String, String>>,
}
