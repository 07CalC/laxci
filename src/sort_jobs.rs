use crate::workflow::Job;
use anyhow::Result;
use std::collections::{HashMap, VecDeque};

pub fn sort_jobs(jobs: &HashMap<String, Job>) -> Result<Vec<String>> {
    let mut indegree: HashMap<String, usize> = HashMap::new();
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for (job_name, job) in jobs {
        indegree.entry(job_name.clone()).or_insert(0);

        for dep in job.needs.clone().unwrap_or_default() {
            graph
                .entry(dep.clone())
                .or_insert_with(Vec::new)
                .push(job_name.clone());
            *indegree.entry(job_name.clone()).or_insert(0) += 1;
        }
    }

    let mut queue: VecDeque<String> = indegree
        .iter()
        .filter_map(|(job, &deg)| if deg == 0 { Some(job.clone()) } else { None })
        .collect();

    let mut result = Vec::new();

    while let Some(job_name) = queue.pop_front() {
        result.push(job_name.clone());

        if let Some(neighbors) = graph.get(&job_name) {
            for next in neighbors {
                if let Some(count) = indegree.get_mut(next) {
                    *count -= 1;
                    if *count == 0 {
                        queue.push_back(next.clone()); // ✅ clone the String
                    }
                }
            }
        }
    }

    if result.len() != jobs.len() {
        anyhow::bail!("Cycle detected in job dependencies (needs)");
    }

    Ok(result)
}
