mod common;

use common::sample_job;
use job_tracker::{SankeyGraph, Stats, Status};

#[test]
fn sankey_export_contains_status_counts_and_svg_markup() {
    let jobs = vec![
        sample_job("Aperture", "Designer", Status::Applied),
        sample_job("Black Mesa", "Engineer", Status::Interviewing),
        sample_job("Cyberdyne", "Researcher", Status::Rejected),
        sample_job("Initech", "Analyst", Status::Pending),
        sample_job("Initrode", "Manager", Status::Applied),
    ];
    let svg = SankeyGraph::from_stats(&Stats::from_jobs(&jobs)).to_svg();
    assert!(svg.starts_with("<svg"));
    assert!(svg.contains("Applications"));
    assert!(svg.contains("Applied: 2"));
    assert!(svg.contains("Interviewing: 1"));
    assert!(svg.contains("</svg>"));
}

#[test]
fn sankey_export_handles_empty_stats() {
    let stats = Stats::from_jobs(&[]);
    let svg = SankeyGraph::from_stats(&stats).to_svg();
    assert!(svg.contains("Applications: 0"));
    assert!(svg.contains("No applications yet"));
}
