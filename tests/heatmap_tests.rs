mod common;

use common::sample_job;
use job_tracker::{ApplicationHeatmap, Status};

#[test]
fn heatmap_counts_applications_by_date() {
    let mut first = sample_job("Acme", "Backend", Status::Applied);
    first.applied_on = "2026-05-01".into();
    let mut second = sample_job("Globex", "Product", Status::Pending);
    second.applied_on = "2026-05-01".into();
    let mut third = sample_job("Initrode", "Design", Status::Rejected);
    third.applied_on = "2026-05-02".into();

    let heatmap = ApplicationHeatmap::from_jobs(&[first, second, third]);

    assert_eq!(heatmap.count_on("2026-05-01"), 2);
    assert_eq!(heatmap.count_on("2026-05-02"), 1);
    assert_eq!(heatmap.active_days(), 2);
    assert_eq!(heatmap.max_count(), 2);
}

#[test]
fn heatmap_handles_empty_data() {
    let heatmap = ApplicationHeatmap::from_jobs(&[]);
    assert_eq!(heatmap.active_days(), 0);
    assert_eq!(heatmap.max_count(), 0);
    assert_eq!(heatmap.cells().count(), 0);
}
