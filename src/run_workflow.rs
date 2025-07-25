use anyhow::Context;
use console::{Emoji, style};
use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Command, Stdio};

use crate::sort_jobs::sort_jobs;
use crate::workflow::Workflow;

pub fn run_workflow(file: String) -> anyhow::Result<()> {
    let emoji_run = Emoji("▶", ">");
    let emoji_job = Emoji("🔨", ">");
    let emoji_step = Emoji("⚙️ ", "*");
    let emoji_fail = Emoji("❌", "X");
    let emoji_success = Emoji("✅", "V");

    let yaml =
        fs::read_to_string(&file).with_context(|| format!("Failed to read file: {}", file))?;

    let wf: Workflow = serde_yaml::from_str(&yaml)
        .with_context(|| format!("invalid workflow format, not a valid YAML file: {}", file))?;

    print!("\x1B[2J\x1B[H");

    println!(
        "{} Running workflow: {}",
        emoji_run,
        style(wf.name.unwrap_or("Unnamed".into()))
            .bold()
            .underlined()
    );

    let job_order = sort_jobs(&wf.jobs)?;

    for job_name in job_order {
        println!(
            "\n{} Job: {}",
            emoji_job,
            style(job_name.clone()).bold().cyan()
        );

        let job = &wf.jobs[&job_name];
        for step in &job.steps {
            if let Some(name) = &step.name {
                println!("\n{}{}", emoji_step, style(name).bold().blue());
            }

            let mut cmd = if cfg!(windows) {
                let mut c = Command::new("cmd");
                c.arg("/C").arg(&step.run);
                c
            } else {
                let mut c = Command::new("sh");
                c.arg("-c").arg(&step.run);
                c
            };
            cmd.stdout(Stdio::piped());
            cmd.stderr(Stdio::piped());

            let mut merged_env = HashMap::new();

            if let Some(wf_env) = &wf.env {
                merged_env.extend(wf_env.clone());
            }
            if let Some(job_env) = &job.env {
                merged_env.extend(job_env.clone());
            }
            if let Some(step_env) = &step.env {
                merged_env.extend(step_env.clone());
            }
            for (key, value) in merged_env {
                cmd.env(key, value);
            }

            let working_dir = if let Some(dir) = &step.working_directory {
                Some(dir)
            } else if let Some(dir) = &job.working_directory {
                Some(dir)
            } else {
                None
            };

            if let Some(dir) = working_dir {
                let path = Path::new(dir);
                if !path.exists() || !path.is_dir() {
                    println!(
                        "{} {}",
                        emoji_fail,
                        style(format!(
                            "Working directory '{}' does not exist or is not a directory",
                            dir
                        ))
                        .red()
                        .bold()
                    );
                    return Ok(());
                }

                println!(
                    "{} {} {}",
                    style("📁").dim(),
                    style("Working directory:").dim(),
                    style(path.display()).cyan()
                );
                cmd.current_dir(path);
            }

            println!("{} {}", style("$").dim(), style(&step.run).dim());

            let mut child = cmd
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
                    let line = line.unwrap_or_default();
                    if line.to_lowercase().contains("warning") {
                        eprintln!("{}", style(line).yellow());
                    } else if line.to_lowercase().contains("error") {
                        eprintln!("{}", style(line).red().bold());
                    } else {
                        eprintln!("{}", style(line).white());
                    }
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
            let step_name = step
                .name
                .clone()
                .unwrap_or_else(|| "unnamed step".to_string());
            println!(
                "{} Step {} Completed",
                emoji_success,
                style(step_name).green()
            );
        }
    }
    println!(
        "\n{} {}",
        emoji_success,
        style("Workflow completed successfully").bold().green(),
    );
    Ok(())
}
