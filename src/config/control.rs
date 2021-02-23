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
        let additional = Control::format_additional_fields(user_defined_fields);

        let source = Control::format_source_contents(&config);
        let binary = Control::format_binary_contents(&config);

        let contents = format!(
            "
{source_data}

{binary_data}
{additional}
",
            source_data = source,
            binary_data = binary,
            additional = additional,
        );

        contents.trim().to_string()
    }

    fn format_str(key: &str, val: &str, acc: &mut String) {
        if val.is_empty() {
            return;
        }
        let f = format!("{k}: {v}\n", k = key, v = val);
        acc.push_str(&f);
    }

    fn format_maintainer(name: &str, email: &str, acc: &mut String) {
        let f = format!("Maintainer: {n} <{e}>\n", n = name, e = email);
        acc.push_str(&f);
    }

    fn format_custom_data<T: Display>(key: &str, val: &T, acc: &mut String) {
        let f = format!("{k}: {v}\n", k = key, v = val);
        acc.push_str(&f);
    }

    fn format_binary_contents(config: &Config) -> String {
        let mut binary_data = String::new();

        Control::format_str(
            "Package",
            &config.control.binary_control.package,
            &mut binary_data,
        );

        Control::format_str(
            "Section",
            &config.control.binary_control.section,
            &mut binary_data,
        );

        Control::format_custom_data(
            "Priority",
            &config.control.binary_control.priority,
            &mut binary_data,
        );

        Control::format_str(
            "Pre-Depends",
            &config.control.binary_control.pre_depends,
            &mut binary_data,
        );

        Control::format_custom_data(
            "Architecture",
            &config.control.binary_control.architecture,
            &mut binary_data,
        );

        Control::format_str(
            "Description",
            &config.control.binary_control.description,
            &mut binary_data,
        );

        binary_data.trim().to_string()
    }

    fn format_source_contents(config: &Config) -> String {
        let mut source_data = String::new();

        let source = &config.control.source_control.source;
        if !source.is_empty() {
            let f = format!("Source: {}\n", source);
            source_data.push_str(&f);
        }

        Control::format_str(
            "Section",
            &config.control.source_control.section,
            &mut source_data,
        );

        Control::format_custom_data(
            "Priority",
            &config.control.source_control.priority,
            &mut source_data,
        );

        let name = &config.control.source_control.maintainer.name;
        let email = &config.control.source_control.maintainer.email;
        Control::format_maintainer(name, email, &mut source_data);

        Control::format_str(
            "Build-Depends",
            &config.control.source_control.build_depends,
            &mut source_data,
        );

        Control::format_str(
            "Standards-Version",
            &config.control.source_control.standards_version,
            &mut source_data,
        );

        Control::format_str(
            "Homepage",
            &config.control.source_control.homepage,
            &mut source_data,
        );

        Control::format_str(
            "Vcs-Browser",
            &config.control.source_control.vcs_browser,
            &mut source_data,
        );

        source_data.trim().to_string()
    }

    fn format_additional_fields(user_defined_fields: Vec<&str>) -> String {
        let mut additional = String::new();
        for field in user_defined_fields {
            additional.push_str(&format!("{}\n", field));
        }

        additional.trim().to_string()
    }

    pub(crate) fn update<'a>(
        config: &Config,
        user_defined_fields: Vec<&str>,
    ) -> Result<&'a str, Box<dyn Error>> {
        if !config.control.update {
            return Ok("debian/control file not updated due to config file setting.");
        }

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
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

    #[test]
    fn test_format_str() {
        let fake_key = "fake key";
        let fake_value = "fake value";
        let mut acc = String::new();
        let expected = format!("{k}: {v}\n", k = fake_key, v = fake_value);

        Control::format_str(fake_key, fake_value, &mut acc);

        assert_eq!(acc, expected);
    }

    #[test]
    fn test_format_str_empty_string() {
        let fake_key = "fake key";
        let fake_value = "";
        let mut acc = String::new();
        let empty_str = String::new();

        Control::format_str(fake_key, fake_value, &mut acc);

        assert_eq!(acc, empty_str);
    }

    #[test]
    fn test_format_maintainer() {
        let fake_name = "fake key";
        let fake_email = "fake email";
        let mut acc = String::new();

        Control::format_maintainer(fake_name, fake_email, &mut acc);
        let expected = format!("Maintainer: {n} <{e}>\n", n = fake_name, e = fake_email);

        assert_eq!(acc, expected);
    }

    #[test]
    fn test_format_custom_data_priority() {
        let fake_key = "fake key";
        let fake_value = Priority::Optional;
        let mut acc = String::new();
        let expected = format!("{k}: {v}\n", k = fake_key, v = fake_value);

        Control::format_custom_data(fake_key, &fake_value, &mut acc);

        assert_eq!(acc, expected);
    }

    #[test]
    fn test_format_custom_data_arch() {
        let fake_key = "fake key";
        let fake_value = Architecture::All;
        let mut acc = String::new();
        let expected = format!("{k}: {v}\n", k = fake_key, v = fake_value);

        Control::format_custom_data(fake_key, &fake_value, &mut acc);

        assert_eq!(acc, expected);
    }

    #[test]
    fn test_format_additional_fields() {
        let fake_row_1 = "key1: value1";
        let fake_row_2 = "key2: value2";
        let fake_row_3 = "key3: value3";
        let fake_fields: Vec<&str> = vec![fake_row_1, fake_row_2, fake_row_3];
        let expected = format!(
            "
{row_1}
{row_2}
{row_3}
",
            row_1 = fake_row_1,
            row_2 = fake_row_2,
            row_3 = fake_row_3,
        )
        .trim()
        .to_string();

        let actual = Control::format_additional_fields(fake_fields);

        assert_eq!(actual, expected);
    }
}
