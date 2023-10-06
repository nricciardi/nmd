# New Markdown (NMD) [BETA]
New Markdown, a new way to write in markdown.

With NMD, you can effortlessly create beautifully formatted text for your projects while enjoying some unique features tailored to modern needs.

NMD is full compatible with CommonMark standard.

## Work In Progress Addons

There are a set of addons which we would add to NMD and many others that we will forgot to add, so you can propose to add a new addon through a new issue.

The list of addons that we would support in future:

- [x] document metadata (author, date, number of pages and so on)
- [x] footnotes
- [ ] page header
- [ ] database diagram
- [ ] uml diagram
- [ ] custom simplifier LaTeX
- [ ] bibliographies
- [x] note of text, i.e. comments
- [ ] image description and reference
- [x] more than one comment and its author for each paragraph

## Configuration file

NMD projects should have `nmd configuration file`, i.e. `nmd.json`, which contains a set of properties to manage NMD projects:

- `collection`: to manage collection mode projects, explained [here](#collections)
- `style`: a key-value map to manage style of project
- `metadata`
- `files`

### Style in configuration

It is possible to associate a set of classes to each type of element using the syntax below:

```json
{
  "style": {
    "heading-1": ["h1", "red-text"],
    "bold": ["bold", "big-text"],
    "link": []
  }
}
```

The list of possible element is below.

- **heading-1, heading-2, ..., heading-6**: heading
- **bold**
- **italic**
- **strikethrough**
- **underlined**
- **link**
- **image**
- **line-break**
- **info-focus-block**
- **warning-focus-block**
- **danger-focus-block**

### Metadata in configuration

There are these metadata:

- **headnotes** (can be `null`): NMD full features string which will be inserted as headnotes 
- **footnotes** (can be `null`): NMD full features string which will be inserted as footnotes 
- **authors**: list of authors
- **references**: NMD full features map of strings which represents the value used to substitute references in NDM text body

An example below.

```json
{
  "metadata": {
    "headnotes": "...",
    "footnotes": "...",
    "authors": ["author1", "author2", "authorN"],
    "references": {
        "ref1": "val1"
     }
  }
}
```

### For file metadata and style

Metadata and style information with the same syntax can be inserted in `files` property. In particular, each key must match with a file name, so that file will override style and metadata.


## Project structure

NMD can be used to create single files or structured projects, called **collections**.

Each project has a directory whit this folders and files hierarchy: 

- **asset**
  - **images**: contains the images
  - **styles**: contains the styles
  - **documents**: contains other documents which are typically linked in NDM files

Each directory can contains other subdirectories.

### Single file mode

If you use single file mode, project has a canonical Markdown file where all content is in that.

This file can be called as you want with `nmd` extension.

### Collections

A structured projects contain multiples files, called **document**. In particular, each file contains an argument, i.e. the argument name is inferred by heading 1 of the document.

Documents can be managed using the `nmd.js` file. In particular, there is a set of attributes can be inserted in the configuration:

```json
{
  "collection": {
    "cover": "cover-file.nmd",
    "order": [
      "file1.nmd",
      "file2.nmd",
      "file3.md",
      "fileN.nmd"
    ]
  }
}
```

> Files can be NMD or MD files.

- **cover** (can be `null`) is the name of the collection cover 
- **order** is the ordered list of file names which form the collection



## Syntax overview

There are two types of modifiers to manipulate your NMD files. In particular, there are the _inline modifiers_ and _paragraph modifiers_.

You can read below the list of all inline and paragraph modifiers.

### Escape

You can prevent text modification using **escape**, i.e. `\`:

```
\*
\_
...
```

### Inline modifiers

#### Metadata

**Metadata** are a set of data which gives information about document, project and so on.

The syntax is:

```
%metadata%
```

There are these metadata:

- **pageN**: page number
- **date**: current date
- **datetime**: current datetime
- **authors**: authors (`nmd.json` corresponding metadata must be set)

####

**Reference** is a... reference! You can use a fictitious name as a classic variable in the programming languages.

References must be set in `nmd.json`.

The syntax is below.

```
&reference
```

If reference is not found, it will be shown as actual reference name.

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

##### Colors and Highlighted Text

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

##### Compatible highlight text

You can use also `==Highlight text==`.

#### Custom text style

```
{Custom text style}(style)
```

There are some standard style such as the color names (to color text) and others.

#### Emoji

Two ways to add emoji:

- Copy and paste an emoji
- Using `:emojiCode:`, for example 🐫


#### Superscript

```
1^st^
```

> This modifier can be placed attached on other text.

#### Subscript

For example, if you want to write "water" in a more scientific way:

```
H~2~O
```

Pay attention, those are two single quote

> This modifier can be placed attached on other text.

#### Link

```markdown
[Link](http://a.com)
```

#### Image

```markdown
![Image](http://url/a.png)
```

#### Inline code

```markdown
`inline code`
```

##### Inline math (LaTeX)

```markdown
$E=mc^2$
```

#### Inline comments

```
// this is a comment
```

#### Bookmark

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


### Paragraph modifiers

The _paragraph modifier_ modifies the style of the paragraph in which are applied.

We define **paragraph** the set composed by a title and text, i.e. a paragraph is between its title and the title of the following paragraph.

Paragraph text **must** be separated from its title using a blank line.

You can press two times `enter`, i.e. `\n\n`, to separate text of the same paragraph **and** different paragraph modifiers. Single `\n` is ignored (you can write the same line in more than one line). 


#### Paragraph styles and metadata

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

#### Heading (Title of a paragraph)

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

#### Line separator

To apply a line separator use --- or *** in a new blank line.

#### List

Different types of list are supported in NMD, below the list with modifier

- `-` common list
- `*` second style list
- `-[] or -[ ]` todo list
- `1. or 1) or a. or a) or I. or I)` ordered list (numerical, alphabetical, romans numbers)

Using `tabs` you can create different list levels.

#### Code block

Code blocks use ``` as paragraph modifier.

It's possible to specify the language used in code block, as in CommonMark, writing language name after first quotes.

```markdown
# code block
print("Write all documents in NMD!!")
```

The list with supported languages (tag in parenthesis):
- Python (python)
- Java (java)
- Javascript (javascript)
- PHP (php)
- HTML (html)
- CSS (css)
- Typescript (typescript)
- Kotlin (kotlin)
- ...

#### Multiline comments

```
/*
multi
line
comment
*/
```

#### Focus block

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

#### Special components

In addition to inline and paragraph modifiers, there are _special components_ which are a set of useful graphic components such as tables, diagram and others.

Each special component has a particular text construct, but is can be defined as stand-alone paragraph, so is possible to add metadata and styles using _paragraph metadata addon_.

##### Tables

Each table has an table head, body and footer (like HTML tables). A table can have only head or only footer, but it must always have body.

##### Table Head

The pattern for a cell table head is: 

```
|| + single space + cell head text + one or more spaces ||
```

###### Table Body

The pattern for a generic table record is: | + single space + cell text + one or more spaces |

```
| cell text | cell text | cell text |
```

##### Table Footer

> WIP

