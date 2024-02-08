# NMD (New MarkDown) Compiler

[![License](https://img.shields.io/badge/license-GPL3-green.svg)](LICENSE)
[![Version](https://img.shields.io/badge/version-v0.3.1-blue.svg)](CHANGELOG.md)

## Overview

NMD is a custom Markdown dialect designed to enhance the classic Markdown syntax with additional features for styling and enriching text. This compiler translates NMD documents into HTML, making it easy to integrate this advanced markup language into your projects.

## Features

- **Extended Syntax**: NMD introduces new modifiers and components to elevate your document styling.
- **Easy Integration**: Compile NMD files into HTML effortlessly for seamless integration with your web projects.
- **Open Source**: This compiler is open source under the GPL-3.0 License, allowing you to modify and adapt it to suit your needs.

## Getting Started

### Installation

To install the NMD compiler, follow these steps:

TODO

### How to use

#### Create a new dossier

Each dossier must have a *dossier configuration file*. It can be named `nmd.yml` or `nmd.json`.

An example of `nmd.yml` file to create a dossier with 3 documents:

```yaml
name: "new dossier"
documents:
  - "./document1.nmd"
  - "./document2.nmd"
  - "./document3.nmd"
```

Each document path can me absolute or relative (from `nmd.yml`).

#### Generate a new empty dossier using compiler

To generate a new NMD dossier:

```shell
nmd-compiler generate dossier -p new/dossier/path
```

You can add a `welcome.nmd` page using `-w`, add `.gitkeep` using `-k` and force directory creation using `-f`.

#### Compile dossier

#### HTML

Build a dossier in `html`:

```shell
nmd-compiler compile dossier -f html -i dossier/input/path -o artifact/output/path
```

> In this moment, to render *math block* and *inline math* an Internet connection is needed. This requirement will be removed in future version.



## Features

### Planned Features

- [ ] All modifiers
- [ ] Possibility to use a different dossier configuration name

### Features in Progress

- [x] Use file name instead of absolute path in dossier configuration
- [x] Local math (no CDN)
- [ ] Lists
- [ ] Other sections in dossier configuration to manage all options
- [ ] Base page style
- [ ] Paper format support (A4, A5, ...)
- [ ] Custom style
- [ ] PDF compile format
- [ ] Tables


## Author

Nicola Ricciardi

## Contributing

If you would like to contribute to the development of the NMD compiler, please follow our [contribution guidelines](CONTRIBUTING.md).

## License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.
