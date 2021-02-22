use serde::Deserialize;

use std::error::Error;
use std::fmt::Display;
use std::fs::OpenOptions;
use std::io::Write;

use super::{Config, Maintainer};

#[derive(Deserialize, Debug)]
pub(crate) struct Control {
    update: bool,
    #[serde(rename(deserialize = "sourceControl"))]
    source_control: SourceControl,
    #[serde(rename(deserialize = "binaryControl"))]
    binary_control: BinaryControl,
}

impl Control {
    fn create_contents(config: &Config, user_defined_fields: Vec<&str>) -> String {
        let mut additional = String::new();
        for field in user_defined_fields {
            additional.push_str(&format!("{}\n", field));
        }

        let mut source_data = String::new();
        let mut binary_data = String::new();

        let source = &config.control.source_control.source;
        if !source.is_empty() {
            let f = format!("Source: {}\n", source);
            source_data.push_str(&f);
        }

        let section = &config.control.source_control.section;
        if !section.is_empty() {
            let f = format!("Section: {}\n", section);
            source_data.push_str(&f);
        }

        let priority = &config.control.source_control.priority;
        source_data.push_str(&format!("Priority: {}\n", priority));

        let name = &config.control.source_control.maintainer.name;
        let email = &config.control.source_control.maintainer.email;
        source_data.push_str(&format!("Maintainer: {n} <{e}>\n", n = name, e = email));

        let build_depends = &config.control.source_control.build_depends;
        if !build_depends.is_empty() {
            let f = format!("Build-Depends: {}\n", build_depends);
            source_data.push_str(&f);
        }

        let standards_version = &config.control.source_control.standards_version;
        if !standards_version.is_empty() {
            let f = format!("Standards-Version: {}\n", standards_version);
            source_data.push_str(&f);
        }

        let homepage = &config.control.source_control.homepage;
        if !homepage.is_empty() {
            let f = format!("Homepage: {}\n", homepage);
            source_data.push_str(&f);
        }

        let vcs_browser = &config.control.source_control.vcs_browser;
        if !vcs_browser.is_empty() {
            let f = format!("Vcs-Browser: {}\n", vcs_browser);
            source_data.push_str(&f);
        }

        let binary_package = &config.control.binary_control.package;
        if !binary_package.is_empty() {
            let f = format!("Package: {}\n", binary_package);
            binary_data.push_str(&f);
        }

        let binary_section = &config.control.binary_control.section;
        if !binary_section.is_empty() {
            let f = format!("Section: {}\n", binary_section);
            binary_data.push_str(&f);
        }

        let binary_priority = &config.control.binary_control.priority;
        binary_data.push_str(&format!("Priority: {}\n", binary_priority));

        let pre_depends = &config.control.binary_control.pre_depends;
        binary_data.push_str(&format!("Pre-Depends: {}\n", pre_depends));

        let arch = &config.control.binary_control.architecture;
        binary_data.push_str(&format!("Architecture: {}\n", arch));

        let description = &config.control.binary_control.description;
        binary_data.push_str(&format!("Description: {}\n", description));

        let contents = format!(
            "
{source_data}

{binary_data}
{additional}
",
            source_data = source_data.trim(),
            binary_data = binary_data.trim(),
            additional = additional.trim(),
        );

        contents.trim().to_string()
    }

    pub(crate) fn update<'a>(
        config: &Config,
        user_defined_fields: Vec<&str>,
    ) -> Result<&'a str, Box<dyn Error>> {
        if !config.control.update {
            return Ok("debian/control file not updated due to config file setting.");
        }

        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .open("debian/control")?;

        let contents = Control::create_contents(config, user_defined_fields);

        file.write_all(contents.as_bytes())?;

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

    fn default_string_value() -> String {
        "".to_string()
    }
}

#[derive(Deserialize, Debug, PartialEq)]
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

#[derive(Deserialize, Debug, PartialEq)]
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
    #[serde(default = "Control::default_string_value")]
    package: String,
    #[serde(default = "Control::default_string_value")]
    description: String,
    #[serde(default = "Control::default_string_value")]
    section: String,
    priority: Priority,
    #[serde(
        rename(deserialize = "preDepends"),
        default = "Control::default_string_value"
    )]
    pre_depends: String,
    architecture: Architecture,
}

#[derive(Deserialize, Debug)]
struct SourceControl {
    #[serde(default = "Control::default_string_value")]
    source: String,
    maintainer: Maintainer,
    #[serde(default = "Control::default_string_value")]
    section: String,
    priority: Priority,
    #[serde(
        rename(deserialize = "buildDepends"),
        default = "Control::default_string_value"
    )]
    build_depends: String,
    #[serde(
        rename(deserialize = "standardsVersion"),
        default = "Control::default_string_value"
    )]
    standards_version: String,
    #[serde(default = "Control::default_string_value")]
    homepage: String,
    #[serde(
        rename(deserialize = "vcsBrowser"),
        default = "Control::default_string_value"
    )]
    vcs_browser: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let default = Control::default();
        let empty_str = String::new();

        assert_eq!(default.update, false);

        assert_eq!(default.source_control.source, empty_str);
        assert_eq!(default.source_control.maintainer.name, empty_str);
        assert_eq!(default.source_control.maintainer.email, empty_str);
        assert_eq!(default.source_control.section, empty_str);
        assert_eq!(default.source_control.priority, Priority::Optional);
        assert_eq!(default.source_control.build_depends, empty_str);
        assert_eq!(default.source_control.standards_version, empty_str);
        assert_eq!(default.source_control.homepage, empty_str);
        assert_eq!(default.source_control.vcs_browser, empty_str);

        assert_eq!(default.binary_control.package, empty_str);
        assert_eq!(default.binary_control.description, empty_str);
        assert_eq!(default.binary_control.section, empty_str);
        assert_eq!(default.binary_control.priority, Priority::Optional);
        assert_eq!(default.binary_control.pre_depends, empty_str);
        assert_eq!(default.binary_control.architecture, Architecture::Any);
    }
}
