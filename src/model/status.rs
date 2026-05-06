use crate::JobTrackerError;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Status {
    Applied,
    Interviewing,
    Rejected,
    Pending,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StatusColours {
    pub background: &'static str,
    pub border: &'static str,
    pub text: &'static str,
}

impl Status {
    pub const fn all() -> [Status; 4] {
        [
            Status::Applied,
            Status::Interviewing,
            Status::Rejected,
            Status::Pending,
        ]
    }

    pub const fn as_db_value(self) -> &'static str {
        match self {
            Status::Applied => "applied",
            Status::Interviewing => "interviewing",
            Status::Rejected => "rejected",
            Status::Pending => "pending",
        }
    }

    pub fn from_db_value(value: &str) -> Result<Self, JobTrackerError> {
        match value {
            "applied" => Ok(Status::Applied),
            "interviewing" => Ok(Status::Interviewing),
            "rejected" => Ok(Status::Rejected),
            "pending" => Ok(Status::Pending),
            other => Err(JobTrackerError::InvalidStatus(other.to_string())),
        }
    }

    pub const fn colours(self) -> StatusColours {
        match self {
            Status::Applied => StatusColours {
                background: "#0f2f4a",
                border: "#38bdf8",
                text: "#bae6fd",
            },
            Status::Interviewing => StatusColours {
                background: "#123c2a",
                border: "#34d399",
                text: "#bbf7d0",
            },
            Status::Rejected => StatusColours {
                background: "#471f2a",
                border: "#fb7185",
                text: "#fecdd3",
            },
            Status::Pending => StatusColours {
                background: "#41320b",
                border: "#facc15",
                text: "#fef08a",
            },
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Status::Applied => "Applied",
            Status::Interviewing => "Interviewing",
            Status::Rejected => "Rejected",
            Status::Pending => "Pending",
        })
    }
}
