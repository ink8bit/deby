use chrono::prelude::*;
use serde::Deserialize;

use std::error::Error;
use std::fmt::Display;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

use super::{Config, Maintainer};

#[derive(Deserialize, Debug)]
pub(crate) struct Changelog {
    update: bool,
    package: String,
    #[serde(default = "Changelog::default_distribution")]
    distribution: Distribution,
    #[serde(default = "Changelog::default_urgency")]
    urgency: Urgency,
    maintainer: Maintainer,
}

impl Changelog {
    /// Formats contents of _changelog_ file.
    /// Newer entries will go first
    ///
    /// # Arguments
    ///
    /// - `entry`- a single _changelog_ entry to be added to _changelog_ file
    /// - `current_file_contents` - previous entries of _changelog_ file
    fn format_contents(entry: &str, current_file_contents: &str) -> String {
        let contents = format!(
            "
{entry}

{current}
",
            entry = entry,
            current = current_file_contents
        );

        let mut s = contents.trim().to_string();
        s.push('\n');
        s
    }

    /// Formats a single changelog entry
    ///
    /// # Arguments
    ///
    /// - `config` - data from config file `.debyrc`
    /// - `version` - version string value to be included in _changelog_ entry
    /// - `changes` - changes string value to be included in _changelog_ entry
    fn format_changelog_entry(config: &Config, version: &str, changes: &str) -> String {
        let date = Changelog::format_date();

        let contents = format!(
            "
{package} ({version}) {distribution}; urgency={urgency}

  {changes}

 -- {name} <{email}>  {date}",
            package = config.changelog.package,
            email = config.changelog.maintainer.email,
            name = config.changelog.maintainer.name,
            distribution = config.changelog.distribution,
            urgency = config.changelog.urgency,
            date = date,
            version = version,
            changes = changes,
        );

        contents
    }

    /// Formats changes section
    ///
    /// # Arguments
    ///
    /// - `changes` - string value of changes
    fn format_changes(changes: &str) -> String {
        if changes.is_empty() {
            return "".to_string();
        }
        let mut formatted_changes = String::new();
        for line in changes.lines() {
            formatted_changes.push_str(&format!("  * {}\n", line));
        }

        formatted_changes.trim().to_string()
    }

    /// Formats current date value according to RFC 2822
    fn format_date() -> String {
        let dt = Local::now();
        dt.to_rfc2822()
    }

    /// Updates _changelog_ file and writes its contents to `debian/changelog` file
    ///
    /// # Arguments
    ///
    /// - `config` - data from config file `.debyrc`
    /// - `version` - version string to be included in _changelog_ file
    /// - `changes` - changes string value to be included in _changelog_ file
    pub(crate) fn update<'a>(
        config: &Config,
        version: &str,
        changes: &str,
    ) -> Result<&'a str, Box<dyn Error>> {
        if !config.changelog.update {
            return Ok("debian/changelog file not updated due to config file setting");
        }

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .open("debian/changelog")?;

        let current_file = fs::read_to_string("debian/changelog")?;

        let formatted_changes = Changelog::format_changes(changes);
        let changelog_entry =
            Changelog::format_changelog_entry(&config, &version, &formatted_changes);
        let contents = Changelog::format_contents(&changelog_entry, &current_file);

        file.write_all(contents.as_bytes())?;

        Ok("Successfully created a new entry in debian/changelog file")
    }

    pub(crate) fn default() -> Self {
        Self {
            update: false,
            package: "".to_string(),
            distribution: Distribution::Unstable,
            urgency: Urgency::Low,
            maintainer: Maintainer {
                name: "".to_string(),
                email: "".to_string(),
            },
        }
    }

    fn default_distribution() -> Distribution {
        Distribution::Unstable
    }

    fn default_urgency() -> Urgency {
        Urgency::Low
    }
}

#[derive(Deserialize, Debug, PartialEq)]
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

#[derive(Deserialize, Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let default = Changelog::default();
        let empty_str = String::new();

        assert_eq!(default.update, false);

        assert_eq!(default.package, empty_str);
        assert_eq!(default.distribution, Distribution::Unstable);
        assert_eq!(default.urgency, Urgency::Low);
        assert_eq!(default.maintainer.name, empty_str);
        assert_eq!(default.maintainer.email, empty_str);
    }

    #[test]
    fn test_format_contents() {
        let fake_entry = "entry";
        let fake_current_file = "current file contents";
        let actual = Changelog::format_contents(fake_entry, fake_current_file);

        let expected = format!(
            "{entry}

{current}
",
            entry = fake_entry,
            current = fake_current_file
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_format_contents_whitespace_new_line() {
        let fake_entry = "entry     \n\n";
        let fake_current_file = "current file contents";
        let actual = Changelog::format_contents(fake_entry, fake_current_file);

        let expected = format!(
            "{entry}

{current}
",
            entry = fake_entry,
            current = fake_current_file
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_format_changes() {
        let fake_changes = "change1\nchange2\nchange3\n";

        let actual = Changelog::format_changes(fake_changes);
        let expected = "  * change1
  * change2
  * change3
"
        .trim()
        .to_string();

        assert_eq!(actual, expected);
    }
}
