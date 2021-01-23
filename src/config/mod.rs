use serde::Deserialize;

use std::fmt::Display;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::{error::Error, fs};

mod changelog;

use changelog::Changelog;

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

const CONFIG_FILE: &str = ".debyrc";

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
        let config_data = match fs::read_to_string(CONFIG_FILE) {
            Ok(data) => data,
            Err(_) => {
                panic!(
                    "No config file provided. Create {} file in your project root.",
                    CONFIG_FILE
                )
            }
        };
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
