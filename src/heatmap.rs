use crate::Job;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicationHeatmap {
    counts: BTreeMap<String, usize>,
}

impl ApplicationHeatmap {
    pub fn from_jobs(jobs: &[Job]) -> Self {
        let mut counts = BTreeMap::new();
        for job in jobs {
            *counts.entry(job.applied_on.clone()).or_insert(0) += 1;
        }
        Self { counts }
    }

    pub fn count_on(&self, date: &str) -> usize {
        self.counts.get(date).copied().unwrap_or(0)
    }

    pub fn active_days(&self) -> usize {
        self.counts.len()
    }

    pub fn max_count(&self) -> usize {
        self.counts.values().copied().max().unwrap_or(0)
    }

    pub fn cells(&self) -> impl Iterator<Item = (&str, usize)> {
        self.counts
            .iter()
            .map(|(date, count)| (date.as_str(), *count))
    }
}
