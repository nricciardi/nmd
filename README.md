# New MarkDown [BETA]

[![License](https://img.shields.io/badge/license-GPL3-green.svg)](LICENSE)
[![Version](https://img.shields.io/badge/version-v0.5.0-blue.svg)](CHANGELOG.md)


- [New MarkDown \[BETA\]](#new-markdown-beta)
  - [Overview](#overview)
    - [Features](#features)
    - [Structure](#structure)
      - [Dossier](#dossier)
  - [Modifier](#modifier)
    - [Heading (Title of a chapter)](#heading-title-of-a-chapter)
    - [Inline modifier](#inline-modifier)
      - [Escape \[NOT SUPPORTED YET\]](#escape-not-supported-yet)
      - [Metadata \[NOT SUPPORTED YET\]](#metadata-not-supported-yet)
      - [Reference \[NOT SUPPORTED YET\]](#reference-not-supported-yet)
      - [Bold](#bold)
      - [Italic](#italic)
      - [Strikethrough text](#strikethrough-text)
      - [Underlined text](#underlined-text)
        - [Colors and Highlighted Text \[NOT SUPPORTED YET\]](#colors-and-highlighted-text-not-supported-yet)
        - [Compatible highlight text \[NOT SUPPORTED YET\]](#compatible-highlight-text-not-supported-yet)
      - [Custom text style \[NOT SUPPORTED YET\]](#custom-text-style-not-supported-yet)
      - [Emoji \[NOT SUPPORTED YET\]](#emoji-not-supported-yet)
      - [Superscript \[NOT SUPPORTED YET\]](#superscript-not-supported-yet)
      - [Subscript \[NOT SUPPORTED YET\]](#subscript-not-supported-yet)
      - [Link \[NOT SUPPORTED YET\]](#link-not-supported-yet)
      - [Inline code](#inline-code)
      - [Inline comments \[TO BE DEFINE; NOT SUPPORTED YET\]](#inline-comments-to-be-define-not-supported-yet)
      - [Bookmark \[TO BE DEFINE; NOT SUPPORTED YEY\]](#bookmark-to-be-define-not-supported-yey)
        - [Todo](#todo)
    - [Paragraph modifier](#paragraph-modifier)
      - [Paragraph styles and metadata \[TO BE DEFINE; NOT SUPPORTED YET\]](#paragraph-styles-and-metadata-to-be-define-not-supported-yet)
      - [Image](#image)
      - [Line separator \[NOT SUPPORTED YET\]](#line-separator-not-supported-yet)
      - [List \[NOT SUPPORTED YET; WIP\]](#list-not-supported-yet-wip)
      - [Code block](#code-block)
      - [Multiline comments \[TO BE DEFINE; NOT SUPPORTED YET\]](#multiline-comments-to-be-define-not-supported-yet)
      - [Focus block \[TO BE DEFINE; NOT SUPPORTED YET\]](#focus-block-to-be-define-not-supported-yet)
      - [Math block (LaTeX)](#math-block-latex)
  - [Getting Started](#getting-started)
    - [Installation](#installation)
    - [How to use](#how-to-use)
      - [Create a new dossier](#create-a-new-dossier)
      - [Generate a new empty dossier using compiler](#generate-a-new-empty-dossier-using-compiler)
      - [Compile dossier](#compile-dossier)
      - [HTML](#html)
  - [Features](#features-1)
    - [Planned Features](#planned-features)
    - [Features in Progress](#features-in-progress)
  - [Author](#author)
  - [Contributing](#contributing)
  - [License](#license)


## Overview

**New MarkDown** NMD, a new way to write in markdown.

NMD is a custom Markdown dialect designed to enhance the classic Markdown syntax with additional features for styling and enriching text. With NMD, you can effortlessly create beautifully formatted text for your projects while enjoying some unique features tailored to modern needs.

NMD is full compatible with CommonMark standard.

### Features

- **Extended Syntax**: NMD introduces new modifiers and components to elevate your document styling.
- **Easy Integration**: Compile NMD files into HTML effortlessly for seamless integration with your web projects.
- **Open Source**: This compiler is open source under the GPL-3.0 License, allowing you to modify and adapt it to suit your needs.

### Structure

TODO

#### Dossier

TODO

## Modifier

### Heading (Title of a chapter)

Create headings using `#` (up to 6 levels). `#` must be separated from text using a blank space ` `.

```markdown
# Heading 1
## Heading 2
...
###### Heading 6
```

It's possible to use this alternative format:

```
#1 Heading 1
#2 Heading 2
...
#6 Heading 6
```

### Inline modifier

TODO

#### Escape [NOT SUPPORTED YET]

You can prevent text modification using **escape**, i.e. `\`:

```
\*
\_
...
```

#### Metadata [NOT SUPPORTED YET]

**Metadata** are a set of data which gives information about document, project and so on.

The syntax is:

```
%metadata%
```

#### Reference [NOT SUPPORTED YET]

**Reference** is a... reference! You can use a fictitious name as a classic variable in the programming languages.

References must be set in `nmd.json`.

The syntax is below.

```
&reference
```

#### Bold

```markdown
**Bold**

or

__Bold__
```

#### Italic

```markdown
_Italic_

or

*Italic*
```

#### Strikethrough text

```
~~Strikethrough text~~
```

#### Underlined text

```
++Underlined text++
```

##### Colors and Highlighted Text [NOT SUPPORTED YET]

Color can be written in hexadecimal if you use `#rrggbb` convention or you can use their names.

You can modify text color, text background and its font using this modifier:

```
[Custom colored text]{{textColor;backgroundColor;fontName}}
```

You can omit font and background color if you want only modify text color.

```
[Only text color]{{#rrggbb}}
```

You can insert only background color or only text font using this convention:

```
[Only background]{{;#rrggbb}}
[Only font]{{;;fontName}}
```

##### Compatible highlight text [NOT SUPPORTED YET]

You can use also `==Highlight text==`.

#### Custom text style [NOT SUPPORTED YET]

```
{Custom text style}(style)
```

There are some standard style such as the color names (to color text) and others.

#### Emoji [NOT SUPPORTED YET]

Two ways to add emoji:

- Copy and paste an emoji
- Using `:emojiCode:`, for example 🐫


#### Superscript [NOT SUPPORTED YET]

```
1^st^
```

> This modifier can be placed attached on other text.

#### Subscript [NOT SUPPORTED YET]

For example, if you want to write "water" in a more scientific way:

```
H~2~O
```

Pay attention, those are two single quote

> This modifier can be placed attached on other text.

#### Link [NOT SUPPORTED YET]

```markdown
[Link](http://a.com)
```

#### Inline code

```markdown
`inline code`
```

#### Inline comments [TO BE DEFINE; NOT SUPPORTED YET]

```
// this is a comment
```

#### Bookmark [TO BE DEFINE; NOT SUPPORTED YEY]

**Bookmarks** are label which can be inserted in text body to mark a paragraph or a piece of paragraph.

```
@[bookmark](description)
```

Description can be multi-lines or can be omitted:

```
@[bookmark]
```

##### Todo

Todo is a special tag to insert... TODOs

```
@[TODO]
```
















### Paragraph modifier

TODO

#### Paragraph styles and metadata [TO BE DEFINE; NOT SUPPORTED YET]

In NMD each paragraph can be decorated with a set of **paragraph decorators**, i.e. **metadata**, **in-line styles** and **style classes**. 

There is a set of standard and custom styles which each indicates a particular style. These are guide lines, each editor could implement a standard style in different ways.

Metadata are introduced using `@`:

```
@ + metadata tag + single space + metadata content
```

Supported metadata:

- `author`
- `content` description of paragraph content
- `createdAt`
- `updatedAt`

A special metadata is the **id** which can be written in two alternatives ways:

```
#the-id
@id the-id
```

> the identifiers should be all in lowercase and each word should be separated using `-`.

Style classes are introduced using `.`, e.g. `.styleClass1`.

In-line styles use CSS (or SCSS/SASS based on editor) key-value modifiers, they haven't a symbol.

To add decorators to a paragraph you must insert `{}` in the line below title, in parenthesis each type of decorator has a particular symbol which introduces it. You can use `;` to separate decorator in the same line or a `\n` to insert decorator in multiple lines.

There is an example below.

```
## Foo title
{
    #the-id
    @author you
    @author yourFriend
    .styleClass1
    background-color: red
}
```

You can add decorators also to a single word using this syntax:

```
This [word]{#the-word; color: red} is red.
```


#### Image

```markdown
![Image](http://url/a.png)
```

#### Line separator [NOT SUPPORTED YET]

To apply a line separator use --- or *** in a new blank line.

#### List [NOT SUPPORTED YET; WIP]

Different types of list are supported in NMD, below the list with modifier

- `-` first style bullet
- `*` second style bullet
- `-[] or -[ ] or - [] or - [ ]` todo bullet
- `->` arrow bullet
- `1. or 1) or a. or a) or I. or I)` ordered bullet (numerical, alphabetical, romans numbers)
- `&unicode;` UNICODE bullet

Using `tabs` or `   ` (3 spaces) you can create different list levels.

Style of first and second bullet types can be managed using the configuration file.

#### Code block

Code blocks use ``` as paragraph modifier.

It's possible to specify the language used in code block, as in CommonMark, writing language name after first three quotes.

NMD uses [PrimJS](https://prismjs.com/) to render code blocks. So, the supported languages (tag in parenthesis) are the same of that library:

- Python (python)
- Java (java)
- Javascript (javascript)
- PHP (php)
- HTML (html)
- CSS (css)
- Typescript (typescript)
- Kotlin (kotlin)
- ...

#### Multiline comments [TO BE DEFINE; NOT SUPPORTED YET]

```
/*
multi
line
comment
*/
```

#### Focus block [TO BE DEFINE; NOT SUPPORTED YET]

Focus blocks allow to insert text in particular paragraph in which the text is highlighted.

There are many types of focus block:

- **info**
- **warning**
- **danger**

The syntax is below.

```
::: warning
Watch out!!!
:::
```

#### Math block (LaTeX)

Math block is a particular paragraph used to print mathematical formulas and more.

The paragraph modifier for math block is double $, i.e. `$$` to open and close blocks.

NMD uses [Katex](https://katex.org/) to render math blocks.



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
- [x] Other sections in dossier configuration to manage all options
- [x] Local math (no CDN)
- [ ] Lists
- [ ] Quotation and "focus block"
- [ ] Base page style
- [ ] Paper format support (A4, A5, ...)
- [ ] Custom style
- [ ] Style modifier
- [ ] PDF compile format
- [ ] Tables
- [ ] Vintage style (typewriter)


## Author

Nicola Ricciardi

## Contributing

If you would like to contribute to the development of the NMD compiler, please follow our [contribution guidelines](CONTRIBUTING.md).

## License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.
