# deby

![Rust](https://github.com/ink8bit/deby/workflows/Rust/badge.svg)

`deby` allows you to create debian changelog and control files via a config file.

## How to use

1. create a config file [.debyrc](#configuration-file) in your project root
2. use `update` function to create (or update) debian files

Add `deby` crate to your dependencies in `Cargo.toml`:

The crate is only available via git repo for now. You can include `git` key with `rev`, `tag` or `branch`. Read more in [Cargo docs](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#specifying-dependencies-from-git-repositories).

```sh
[dependencies]
deby = { git = "https://github.com/ink8bit/deby", branch = "main" }
```

## Public API

### `update`

Update *debian control* and *changelog* files.

```rust
// provide required arguments
let version = "1.0.0";
let changes = "some changes";

// if you want to provide additional fields - separate them with `;` or just put an empty string
let user_defined_fields: Vec<&str> = vec!["Some-Field: A", "Another-Field: B"];

match deby::update(version, changes, user_defined_fields) {
    Ok(msg) => {
        println!("{}", msg.0);
        println!("{}", msg.1);
    }
    Err(e) => panic!("{}", e),
}
```

### `update_changelog_file`

Update only *debian changelog* file.

```rust
let version = "1.0.0";
let changes = "changes:\nline1\nline2\nline3";

match deby::update_changelog_file(version, changes) {
    Ok(msg) => println!("{}", msg),
    Err(e) => panic!("{}", e),
}
```

### `update_control_file`

Update only *debian control* file.

```rust
let user_defined_fields: Vec<&str> = vec!["Some-Field: A", "Another-Field: B"];

match deby::update_control_file(user_defined_fields) {
    Ok(msg) => println!("{}", msg),
    Err(e) => panic!("{}", e),
}
```

## Configuration file

A configuration file `.debyrc` should be placed in the project root.

It should be a valid *JSON* file and contain the following fields:

```json
{
  "changelog": {
    "update": true,
    "package": "changelog package name",
    "distribution": "unstable",
    "urgency": "low",
    "maintainer": {
      "name": "maintainer name",
      "email": "maintainer email"
    }
  },
  "control": {
    "update": true,
    "sourceControl": {
      "source": "source",
      "section": "section",
      "priority": "optional",
      "buildDepends": ["depends"],
      "standardsVersion": "1.2.3",
      "homepage": "url",
      "vcsBrowser": "url",
      "maintainer": {
        "name": "maintainer name",
        "email": "maintainer email"
      }
    },
    "binaryControl": {
      "package": "binary package name",
      "description": "description",
      "section": "section",
      "priority": "optional",
      "preDepends": "depends",
      "architecture": "all"
    }
  }
}
```

If you don't want to create `control` or `changelog` files you need to use `update: false` in the corresponding sections of your `.debyrc`:

```json
{
  "package": "package name",
  "maintainer": {
    "email": "user@example.com",
    "name": "username"
  },
  "changelog": {
    "update": false,
    "distribution": "unstable",
    "urgency": "low"
  },
  "control": {
    "update": false,
    "sourceControl": {
      "source": "source",
      "section": "section",
      "priority": "optional",
      "buildDepends": ["depends"],
      "standardsVersion": "1.2.3",
      "homepage": "url",
      "vcsBrowser": "url"
    },
    "binaryControl": {
      "description": "description",
      "section": "section",
      "priority": "optional",
      "preDepends": "depends",
      "architecture": "all"
    }
  }
}
```

You can omit `changelog` or `control` (or both) sections entirely:

```json
{
  "package": "package name",
  "maintainer": {
    "email": "user@example.com",
    "name": "username"
  }
}
```

You can omit certain fields, in this case default values will be used. For example:

```json
{
  "package": "package name",
  "maintainer": {
    "email": "user@example.com",
    "name": "username"
  },
  "changelog": {
    "update": true
  }
}
```

This config will only update **changelog** file, *distribution* will have `unstable` value and *urgency* will be `low`.

Read more about possible values for certain fields below.

### Distribution

**Distribution** field should be one of these [values](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-distribution):

*Default value:* `unstable`

- `unstable`
- `experimental`

### Urgency

*Default value:* `low`

**Urgency** field should be one of these [values](https://www.debian.org/doc/debian-policy/ch-controlfields.html#urgency):

- `low`
- `medium`
- `high`
- `emergency`
- `critical`

### Architecture

**Architecture** field should be one of these [values](https://www.debian.org/doc/debian-policy/ch-controlfields.html#s-f-architecture):

- `all`
- `any`

### Priority

**Priority** field should be one of these [values](https://www.debian.org/doc/debian-policy/ch-archive.html#s-priorities):

- `required`
- `important`
- `standard`
- `optional`
- `extra`

### Depends

#### `Build-Depends` field

You can completely omit this field if you don't need to use it.
You can use only one value in `.debyrc`:

```json
{
  "buildDepends": ["value 1"]
}
```

or multiple values:

```json
{
  "buildDepends": [
    "depends 1",
    "depends 2"
  ]
}
```

## Official docs

You can read more information about all fields on official website:

- [Debian packages](https://www.debian.org/doc/debian-policy/index.html)
- [Changelog](https://www.debian.org/doc/debian-policy/ch-source.html#debian-changelog-debian-changelog)
- [Control](https://www.debian.org/doc/debian-policy/ch-controlfields.html#)
