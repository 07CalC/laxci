use std::{fs, path::Path};

use anyhow::Result;

pub fn init_workflow() -> Result<()> {
    let path = Path::new("laxci.yml");

    if path.exists() {
        println!("⚠️  'laxci.yml' already exists. Aborting.");
        return Ok(());
    }

    let default_yaml = r#"
name: Example Workflow

jobs:
  hello:
    steps:
      - name: Say hello
        run: echo "👋 Hello from LaxCI"
"#;

    fs::write(path, default_yaml.trim_start())?;
    println!("✅ Created starter workflow file: laxci.yml");
    Ok(())
}
