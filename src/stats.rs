use crate::{Job, Status};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stats {
    pub total: usize,
    counts: [(Status, usize); 4],
}

impl Stats {
    pub fn from_jobs(jobs: &[Job]) -> Self {
        let mut stats = Self {
            total: jobs.len(),
            counts: Status::all().map(|status| (status, 0)),
        };

        for job in jobs {
            if let Some((_, count)) = stats
                .counts
                .iter_mut()
                .find(|(status, _)| *status == job.status)
            {
                *count += 1;
            }
        }

        stats
    }

    pub fn count_for(&self, status: Status) -> usize {
        self.counts
            .iter()
            .find(|(candidate, _)| *candidate == status)
            .map(|(_, count)| *count)
            .unwrap_or(0)
    }

    pub fn percentage_for(&self, status: Status) -> f32 {
        if self.total == 0 {
            0.0
        } else {
            self.count_for(status) as f32 / self.total as f32
        }
    }
}
