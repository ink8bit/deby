use serde::Deserialize;

use std::error::Error;
use std::fmt::Display;
use std::fs::OpenOptions;
use std::io::Write;

use super::Config;

#[derive(Deserialize, Debug)]
pub(crate) struct Control {
    update: bool,
    #[serde(rename(deserialize = "sourceControl"))]
    source_control: SourceControl,
    #[serde(rename(deserialize = "binaryControl"))]
    binary_control: BinaryControl,
}

impl Control {
    pub(crate) fn update<'a>(
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
            additional.push_str(&format!("{}\n", field));
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

    pub(crate) fn default() -> Self {
        Self {
            update: false,
            source_control: SourceControl {
                source: "no source value provided".to_string(),
                section: "no section value provided".to_string(),
                priority: Priority::Optional,
                build_depends: "no build depends value provided".to_string(),
                standards_version: "no version value provided".to_string(),
                homepage: "no homepage value provided".to_string(),
                vcs_browser: "no vcs browser value provided".to_string(),
            },
            binary_control: BinaryControl {
                description: "no description value provided".to_string(),
                section: "no section value provided".to_string(),
                priority: Priority::Optional,
                pre_depends: "no pre-depends value provided".to_string(),
                architecture: Architecture::Any,
            },
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
