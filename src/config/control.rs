use serde::Deserialize;

use std::fmt::Display;
use std::fs::OpenOptions;
use std::io::Write;

use super::{Config, DebyError, Maintainer};

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
        user_defined_fields: Vec<&str>,
    ) -> Result<&'a str, DebyError> {
        if !config.control.update {
            return Ok("debian/control file not updated due to config file setting.");
        }

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .open("debian/control")
            .map_err(|_| DebyError::ControlOpen)?;

        let mut additional = String::new();
        for field in user_defined_fields {
            additional.push_str(&format!("{}\n", field));
        }

        let contents = format!(
            "
Source: {source}
Section: {source_section}
Priority: {source_priority}
Maintainer: {name} <{email}>
Build-Depends: {build_depends}
Standards-Version: {standards_version}
Homepage: {homepage}
Vcs-Browser: {vcs_browser}

Package: {package}
Section: {binary_section}
Priority: {binary_priority}
Pre-Depends: {pre_depends}
Architecture: {arch}
Description: {description}
{additional}
",
            source = config.control.source_control.source,
            source_section = config.control.source_control.section,
            source_priority = config.control.source_control.priority,
            name = config.control.source_control.maintainer.name,
            email = config.control.source_control.maintainer.email,
            build_depends = config.control.source_control.build_depends,
            standards_version = config.control.source_control.standards_version,
            homepage = config.control.source_control.homepage,
            vcs_browser = config.control.source_control.vcs_browser,
            package = config.control.binary_control.package,
            binary_section = config.control.binary_control.section,
            binary_priority = config.control.binary_control.priority,
            pre_depends = config.control.binary_control.pre_depends,
            arch = config.control.binary_control.architecture,
            description = config.control.binary_control.description,
            additional = additional,
        );

        file.write_all(contents.trim().as_bytes())
            .map_err(|_| DebyError::ControlWrite)?;

        Ok("Successfully created a new entry in debian/control file.")
    }

    pub(crate) fn default() -> Self {
        Self {
            update: false,
            source_control: SourceControl {
                source: "".to_string(),
                maintainer: Maintainer {
                    name: "".to_string(),
                    email: "".to_string(),
                },
                section: "".to_string(),
                priority: Priority::Optional,
                build_depends: "".to_string(),
                standards_version: "".to_string(),
                homepage: "".to_string(),
                vcs_browser: "".to_string(),
            },
            binary_control: BinaryControl {
                package: "".to_string(),
                description: "".to_string(),
                section: "".to_string(),
                priority: Priority::Optional,
                pre_depends: "".to_string(),
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
    package: String,
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
    maintainer: Maintainer,
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
