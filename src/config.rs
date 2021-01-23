use chrono::prelude::*;
use serde::Deserialize;

use std::fmt::Display;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::{error::Error, fs};

#[derive(Deserialize, Debug)]
struct Changelog {
    update: bool,
    distribution: Distribution,
    urgency: Urgency,
}

impl Changelog {
    fn update<'a>(
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

#[derive(Deserialize, Debug)]
enum Architecture {
    #[serde(rename(deserialize = "all"))]
    All,
    #[serde(rename(deserialize = "any"))]
    Any,
}

impl Display for Architecture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Architecture::All => write!(f, "all"),
            Architecture::Any => write!(f, "any"),
        }
    }
}

#[derive(Deserialize, Debug)]
enum Priority {
    #[serde(rename(deserialize = "required"))]
    Required,
    #[serde(rename(deserialize = "important"))]
    Important,
    #[serde(rename(deserialize = "standard"))]
    Standard,
    #[serde(rename(deserialize = "optional"))]
    Optional,
    #[serde(rename(deserialize = "extra"))]
    Extra,
}

impl Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Priority::Required => write!(f, "required"),
            Priority::Important => write!(f, "important"),
            Priority::Standard => write!(f, "standard"),
            Priority::Optional => write!(f, "optional"),
            Priority::Extra => write!(f, "extra"),
        }
    }
}

#[derive(Deserialize, Debug)]
struct BinaryControl {
    description: String,
    section: String,
    priority: Priority,
    #[serde(rename(deserialize = "preDepends"))]
    pre_depends: String,
    architecture: Architecture,
}

#[derive(Deserialize, Debug)]
struct SourceControl {
    source: String,
    section: String,
    priority: Priority,
    #[serde(rename(deserialize = "buildDepends"))]
    build_depends: String,
    #[serde(rename(deserialize = "standardsVersion"))]
    standards_version: String,
    homepage: String,
    #[serde(rename(deserialize = "vcsBrowser"))]
    vcs_browser: String,
}

#[derive(Deserialize, Debug)]
struct Control {
    update: bool,
    #[serde(rename(deserialize = "sourceControl"))]
    source_control: SourceControl,
    #[serde(rename(deserialize = "binaryControl"))]
    binary_control: BinaryControl,
}

impl Control {
    fn update<'a>(
        config: &Config,
        version: &str,
        user_defined_fields: &str,
    ) -> Result<&'a str, Box<dyn Error>> {
        if !config.control.update {
            return Ok("debian/control file not updated due to config file setting.");
        }

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .open("debian/control")?;

        let mut additional = String::new();
        for field in user_defined_fields.split(';') {
            additional.push_str(&format!("X-{}\n", field));
        }

        let contents = format!(
            "
Source: {source}
Priority: {source_priority}
Maintainer: {name} <{email}>
Build-Depends: {build_depends}
Standards-Version: {standards_version}
Homepage: {homepage}
Vcs-Browser: {vcs_browser}

Package: {package}
Section: {section}
Priority: {binary_priority}
Pre-Depends: {pre_depends}
Architecture: {arch}
Description: {description}
{additional}
",
            source = config.control.source_control.source,
            source_priority = config.control.source_control.priority,
            name = config.maintainer.name,
            email = config.maintainer.email,
            build_depends = config.control.source_control.build_depends,
            standards_version = config.control.source_control.standards_version,
            homepage = config.control.source_control.homepage,
            vcs_browser = config.control.source_control.vcs_browser,
            package = config.package,
            section = config.control.binary_control.section,
            binary_priority = config.control.binary_control.priority,
            pre_depends = config.control.binary_control.pre_depends,
            arch = config.control.binary_control.architecture,
            description = config.control.binary_control.description,
            additional = additional,
        );

        file.write_all(contents.trim().as_bytes())?;

        Ok("Successfully created a new entry in debian/control file.")
    }
}

#[derive(Deserialize, Debug)]
struct Maintainer {
    name: String,
    email: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    maintainer: Maintainer,
    package: String,
    changelog: Changelog,
    control: Control,
}

impl Config {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let config = Self::parse()?;
        Ok(Self {
            package: config.package,
            maintainer: config.maintainer,
            changelog: config.changelog,
            control: config.control,
        })
    }

    fn parse() -> Result<Config, Box<dyn Error>> {
        // No config file provided case
        let config_data = fs::read_to_string(".debyrc")?;
        let config: Config = serde_json::from_str(&config_data)?;
        Ok(config)
    }

    pub fn update(
        &self,
        version: &str,
        changes: &str,
        user_defined_fields: &str,
    ) -> Result<(), Box<dyn Error>> {
        if !Path::new("debian").exists() {
            fs::create_dir("debian")?;
        }

        match Changelog::update(&self, &version, &changes) {
            Ok(msg) => println!("{}", msg),
            Err(e) => panic!("{}", e),
        };

        match Control::update(&self, &version, &user_defined_fields) {
            Ok(msg) => println!("{}", msg),
            Err(e) => panic!("{}", e),
        };

        Ok(())
    }
}
