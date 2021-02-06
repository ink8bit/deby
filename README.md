# deby

`deby` allows you to create debian changelog and control files via a config file.

## How to use

1. create a config file `.debyrc` in your project root
2. use `update` function to create (or update) debian files

```rust
// include deby crate
use deby;

// provide required arguments
let version = "1.0.0";
let changes = "some changes";
// if you want to provide additional fields - separate them with `;` or just put an empty string
let additional_fields = "Some-Field: A;Another-Field: B";

if let Err(e) = deby::update(version, changes, additional_fields) {
    panic!("{}", e);
}
```

## Configuration file

A configuration file `.debyrc` should be placed in the project root.

It should be a valid JSON file and contain the following fields:

```json
{
  "package": "package name",
  "maintainer": {
    "email": "user@example.com",
    "name": "username"
  },
  "changelog": {
    "update": true,
    "distribution": "unstable",
    "urgency": "low"
  },
  "control": {
    "update": true,
    "sourceControl": {
      "source": "source",
      "section": "section",
      "priority": "optional",
      "buildDepends": "depends",
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

If you don't want to create `control` or `changelog` file you have to use `update: false` in the corresponding sections of your `.debyrc`:

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
      "buildDepends": "depends",
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

## Official docs

You can read more information about all fields on official website:

- [Debian packages](https://www.debian.org/doc/debian-policy/index.html)
- [Changelog](https://www.debian.org/doc/debian-policy/ch-source.html#debian-changelog-debian-changelog)
- [Control](https://www.debian.org/doc/debian-policy/ch-controlfields.html#)
