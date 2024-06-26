# New MarkDown [BETA]

**New way to write in markdown**

[![License](https://img.shields.io/badge/license-GPL3-green.svg)](LICENSE)
[![Version](https://img.shields.io/badge/version-v0.29.0-blue.svg)](CHANGELOG.md)

NMD stands for **New MarkDown**, or for friends, *Nicola MarkDown* (if Stephen Bourne can name a shell, why can't I name mine NMD?)

- [New MarkDown \[BETA\]](#new-markdown-beta)
  - [Overview](#overview)
    - [Why NMD?](#why-nmd)
      - [Extended Syntax](#extended-syntax)
      - [Order with Integrated Dossiers](#order-with-integrated-dossiers)
      - [Predefined Styles and Customization](#predefined-styles-and-customization)
      - [Cool Factor](#cool-factor)
  - [Getting Started](#getting-started)
    - [TL;DR](#tldr)
    - [Installation](#installation)
    - [Generate a new dossier using compiler](#generate-a-new-dossier-using-compiler)
      - [Add a new document](#add-a-new-document)
      - [Compile dossier](#compile-dossier)
      - [HTML](#html)
  - [Develop](#develop)
  - [NMD Syntax](#nmd-syntax)
  - [Author](#author)
  - [Contributing](#contributing)
  - [License](#license)


## Overview

NMD is a custom Markdown dialect designed to enhance the classic Markdown syntax with additional features for styling and enriching text. With NMD, you can effortlessly create beautifully formatted text for your projects while enjoying some unique features tailored to modern needs.

NMD is full compatible with CommonMark standard.

### Why NMD?

#### Extended Syntax

NMD introduces new modifiers and components to elevate your document styling.

For example, you can't emphasize section "work in progress" in common mark, but in NMD you can!

You can use [TODO](NMD.md#todo) modifier to emphasize a missed section:

![TODO modifier](docs/assets/images/todo-modifier.png)

Alternatively, do you want emphasize a section where you wrote a tip or warning comment? You can use [Focus Block](NMD.md#focus-block)

![Focus Block modifier](docs/assets/images/focus-block-modifier.png)

#### Order with Integrated Dossiers

Keep your documents organized with integrated "dossiers" for more intuitive and structured management.

#### Predefined Styles and Customization

- **Built-in Styles**: Choose from three predefined styles like Light, Dark, and Vintage, for a personalized reading experience.
- **Styling in Syntax**: Apply style directly in the text using NMD syntax.
- **Additional Page Styling**: Further customize the look of your page with additional styles.

- **Easy Integration**: Compile NMD files into HTML effortlessly for seamless integration with your web projects.
- **Open Source**: This compiler is open source under the GPL-3.0 License, allowing you to modify and adapt it to suit your needs.

#### Cool Factor

Why stick to Markdown when you can be cool using **NMD**?

## Getting Started

### TL;DR

```shell
nmd generate dossier -p dossier/input/path -f -w

nmd dossier add -p dossier/input/path -d new-document.nmd

nmd compile dossier -f html -i dossier/input/path -o artifact/output/path
```

### Installation

To install NMD, follow these steps:

1. Download the last release based on your operating system
2. Extract files
3. Run `nmd` execution file

### Generate a new dossier using compiler

To **generate a new dossier** you can use the following command:

```shell
nmd generate dossier -p dossier/input/path
```

There are many *flags* that you can use in combination with `generate dossier`. For example, if you want *force* the generation you can use `-f`, or if you want a *welcome page* you can use `-w`.

```shell
nmd generate dossier -p dossier/input/path -f -w
```

The Git support is planned, but not implemented yet. You can only add `.gitkeep` files in assets directories using `-k`.

#### Add a new document

To **add a new document** you can use the following command:

```shell
nmd dossier add -p dossier/input/path -d new-document.nmd
```

If the document name doesn't have `nmd` extension, it will be added automatically.

You can add more than one document at the same time:

```shell
nmd dossier add -p dossier/input/path -d new-document-1.nmd -d new-document-2.nmd -d new-document-3.nmd
```

#### Compile dossier

#### HTML

Compile a dossier in `html`:

```shell
nmd compile dossier -f html -i dossier/input/path -o artifact/output/path
```

If you watch dossier files and compile them if anything changes, you should use watcher mode (`-w` option).

Watcher mode compile dossier if any change is captured. Changes are captured only if a minimum time is elapsed. To set minimum time use `-t` option

## Develop

Develop [check list](DEVELOP.md)

## NMD Syntax

[NMD Syntax](NMD.md)

> [!WARNING]
> NMD syntax is working in progress yet, you can contribute following [contribution guidelines](CONTRIBUTING.md)!

## Author

Nicola Ricciardi

## Contributing

If you would like to contribute to the development of the NMD compiler, please follow [contribution guidelines](CONTRIBUTING.md).

## License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.
