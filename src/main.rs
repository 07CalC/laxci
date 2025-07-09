mod workflow;
use workflow::*;

use anyhow::{Context, Result};
use clap::Parser;
use console::{Emoji, style};
use std::fs;
use std::io::{self, BufRead, BufReader};
use std::process::{Command, Stdio};

#[derive(Parser)]
struct Cli {
    #[arg(short, long, default_value = "laxci.yml")]
    file: String,
}

fn main() -> Result<()> {
    let emoji_run = Emoji("▶", ">");
    let emoji_job = Emoji("🔨", ">");
    let emoji_step = Emoji("⚙️ ", "*");
    let emoji_fail = Emoji("❌", "X");
    let emoji_success = Emoji("✅", "V");

    let cli = Cli::parse();

    let yaml = fs::read_to_string(&cli.file)
        .with_context(|| format!("Failed to read file: {}", cli.file))?;

    let wf: Workflow = serde_yaml::from_str(&yaml).with_context(|| {
        format!(
            "invalid workflow format, not a valid YAML file: {}",
            cli.file
        )
    })?;

    print!("\x1B[2J\x1B[H");

    println!(
        "{} Running workflow: {}",
        emoji_run,
        style(wf.name.unwrap_or("Unnamed".into()))
            .bold()
            .underlined()
    );

    for (job_name, job) in wf.jobs {
        println!("\n{} Job: {}", emoji_job, style(job_name).bold().cyan());

        for step in job.steps {
            if let Some(name) = &step.name {
                println!("\n{} {}", emoji_step, style(name).bold().blue());
            }

            println!("{} {}", style("$").dim(), style(&step.run).dim());

            let mut child = Command::new("sh")
                .arg("-c")
                .arg(&step.run)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .with_context(|| format!("Failed to spawn command: {}", step.run))?;

            let stdout = child.stdout.take().unwrap();
            let stderr = child.stderr.take().unwrap();

            let stdout_thread = std::thread::spawn(move || {
                let reader = BufReader::new(stdout);
                for line in reader.lines() {
                    eprintln!("{}", style(line.unwrap_or_default()).white());
                }
            });

            let stderr_thread = std::thread::spawn(move || {
                let reader = BufReader::new(stderr);
                for line in reader.lines() {
                    eprintln!("{}", style(line.unwrap_or_default()).red());
                }
            });

            let status = child.wait()?;

            stdout_thread.join().unwrap();
            stderr_thread.join().unwrap();

            if !status.success() {
                println!(
                    "{} {}",
                    emoji_fail,
                    style(format!("Step failed with status: {}", status))
                        .bold()
                        .red()
                );
                return Ok(());
            }
            println!(
                "{} {}",
                emoji_success,
                style("Step completed successfully").bold().green(),
            );
        }
    }
    println!("\n{} Workflow completed successfully!", emoji_success);
    Ok(())
}
