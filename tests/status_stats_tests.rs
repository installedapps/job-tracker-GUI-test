mod common;

use common::sample_job;
use job_tracker::{ActivePanel, AnimationState, Stats, Status};

#[test]
fn statuses_are_available_in_the_expected_order() {
    assert_eq!(
        Status::all(),
        [
            Status::Applied,
            Status::Interviewing,
            Status::Rejected,
            Status::Pending,
        ]
    );
}

#[test]
fn status_round_trips_through_database_values() {
    for status in Status::all() {
        assert_eq!(Status::from_db_value(status.as_db_value()).unwrap(), status);
    }
    assert!(Status::from_db_value("unknown").is_err());
}

#[test]
fn every_status_has_a_unique_colour_assignment() {
    let colours = Status::all().map(|status| status.colours());
    for (index, colour) in colours.iter().enumerate() {
        assert!(colours.iter().skip(index + 1).all(|other| other != colour));
    }
}

#[test]
fn active_panel_can_switch_between_jobs_and_stats() {
    let mut panel = ActivePanel::Jobs;
    assert!(!panel.shows_stats());
    panel = panel.switch_to(ActivePanel::Stats);
    assert_eq!(panel, ActivePanel::Stats);
    assert!(panel.shows_stats());
    panel = panel.switch_to(ActivePanel::Jobs);
    assert_eq!(panel, ActivePanel::Jobs);
}

#[test]
fn animation_state_cycles_and_offsets_visible_bars() {
    let mut animation = AnimationState::default();
    assert_eq!(animation.bar_value(0.5), 0.5);
    animation.advance();
    assert!(animation.bar_value(0.5) > 0.5);
    for _ in 0..AnimationState::MAX_STEP {
        animation.advance();
    }
    assert_eq!(animation.step(), 0);
}

#[test]
fn stats_count_jobs_by_status_and_total() {
    let jobs = vec![
        sample_job("Aperture", "Designer", Status::Applied),
        sample_job("Black Mesa", "Engineer", Status::Interviewing),
        sample_job("Cyberdyne", "Researcher", Status::Rejected),
        sample_job("Initech", "Analyst", Status::Pending),
        sample_job("Initrode", "Manager", Status::Applied),
    ];
    let stats = Stats::from_jobs(&jobs);
    assert_eq!(stats.total, 5);
    assert_eq!(stats.count_for(Status::Applied), 2);
    assert_eq!(stats.count_for(Status::Interviewing), 1);
    assert_eq!(stats.count_for(Status::Rejected), 1);
    assert_eq!(stats.count_for(Status::Pending), 1);
}
