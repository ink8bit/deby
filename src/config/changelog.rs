use chrono::prelude::*;
use serde::Deserialize;

use std::fmt::Display;
use std::fs::OpenOptions;
use std::io::Write;
use std::{error::Error, fs};

use super::Config;

#[derive(Deserialize, Debug)]
pub(crate) struct Changelog {
    update: bool,
    distribution: Distribution,
    urgency: Urgency,
}

impl Changelog {
    pub(crate) fn update<'a>(
        config: &Config,
        version: &str,
        changes: &str,
    ) -> Result<&'a str, Box<dyn Error>> {
        if !config.changelog.update {
            return Ok("debian/changelog file not updated due to config file setting.");
        }

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .open("debian/changelog")?;
        let current = fs::read_to_string("debian/changelog")?;

        let dt = Utc::now().to_rfc2822();

        let mut changes_list = String::new();
        for line in changes.lines() {
            changes_list.push_str(&format!("* {}\n", line));
        }

        let contents = format!(
            "
{package} ({version}) {distribution}; urgency={urgency}

{changes}

-- {name} <{email}>  {date}

{current}",
            package = config.package,
            email = config.maintainer.email,
            name = config.maintainer.name,
            distribution = config.changelog.distribution,
            urgency = config.changelog.urgency,
            current = current,
            date = dt,
            version = version,
            changes = changes_list.trim(),
        );

        file.write_all(contents.trim().as_bytes())?;

        Ok("Successfully created a new entry in debian/changelog file.")
    }
}

#[derive(Deserialize, Debug)]
enum Urgency {
    #[serde(rename(deserialize = "low"))]
    Low,
    #[serde(rename(deserialize = "medium"))]
    Medium,
    #[serde(rename(deserialize = "high"))]
    High,
    #[serde(rename(deserialize = "emergency"))]
    Emergency,
    #[serde(rename(deserialize = "critical"))]
    Critical,
}

impl Display for Urgency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Urgency::Low => write!(f, "low"),
            Urgency::Medium => write!(f, "medium"),
            Urgency::High => write!(f, "high"),
            Urgency::Emergency => write!(f, "emergency"),
            Urgency::Critical => write!(f, "critical"),
        }
    }
}

#[derive(Deserialize, Debug)]
enum Distribution {
    #[serde(rename(deserialize = "unstable"))]
    Unstable,
    #[serde(rename(deserialize = "experimental"))]
    Experimental,
}

impl Display for Distribution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Distribution::Unstable => write!(f, "unstable"),
            Distribution::Experimental => write!(f, "experimental"),
        }
    }
}
