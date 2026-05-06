use directories::ProjectDirs;
use std::path::PathBuf;

fn data_file(name: &str) -> PathBuf {
    ProjectDirs::from("com", "local", "job-tracker")
        .map(|dirs| dirs.data_local_dir().join(name))
        .unwrap_or_else(|| PathBuf::from(name))
}

pub fn database_path() -> PathBuf {
    data_file("jobs.sqlite3")
}

pub fn sankey_export_path() -> PathBuf {
    data_file("job-pipeline-sankey.svg")
}
