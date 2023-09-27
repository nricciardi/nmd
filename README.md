# New Markdown (NMD) [BETA]
New Markdown, a new way to write in markdown.

With NMD, you can effortlessly create beautifully formatted text for your projects while enjoying some unique features tailored to modern needs.

NMD is full compatible with CommonMark standard.

## Syntax overview

There are two types of modifiers to manipulate your NMD files. In particular, there are the _inline modifiers_ and _paragraph modifiers_.

You can read below the list of all inline and paragraph modifiers. 

### Inline modifiers

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

={ciao}(red)

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
{Custom colored text}(textColor;backgroundColor;fontName)
```

You can omit font and background color if you want only modify text color.

```
{Only text color}(#rrggbb)
```

You can insert only background color or only text font using this convention:

```
{Only background}(;#rrggbb)
{Only font}(;;fontName)
```

#### Custom text style

```
{Custom text style}(style)
```

There are some standard style such as the color names (to color text) and others.


#### Superscript

```
1^^st^^
```

> This modifier can be placed attached on other text.

#### Subscript

For example, if you want to write "water" in a more scientific way:

```
H''2''O
```

Pay attention, those are two single quote

> This modifier can be placed attached on other text.

#### Link

```markdown
[Link](http://a.com)

or

[Link][1]
⋮
[1]: http://b.org
```

#### Image

```markdown
![Image](http://url/a.png)

or

![Image][1]
⋮
[1]: http://url/b.jpg
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


### Paragraph modifiers

The _paragraph modifier_ modifies the style of the paragraph in which are applied.

We define **paragraph** the set composed by a title and text, i.e. a paragraph is between its title and the title of the following paragraph.

Paragraph text **must** be separated from its title using a blank line.

You can press two times `enter`, i.e. `\n\n`, to separate text of the same paragraph **and** different paragraph modifiers. Single `\n` is ignored (you can write the same line in more than one line). 

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

#### Math block (LaTeX)

Math block is a particular paragraph used to print mathematical formulas and more.

The paragraph modifier for math block is double $, i.e. `$$` to open and close blocks.

#### Multiline comments

```
/*
multi
line
comment
*/
```

#### Paragraph styler

In NMD is possible to indicate a paragraph style. There is a set of standard styles which each indicates a particular style that should be implemented from the NDM editors. These are guide lines, each editor could implement different styles.

In addiction, there is the possibility to add a custom style reference.

Styles could be implemented using any languages, commonly it is used CSS (or SCSS/SASS).

To indicate the style of the paragraph you must use `.` modifier. If it isn't presente, editors use default paragraph style.

A *style* is a set of style rules to modified a paragraph.

Styles are introduced using `.`, e.g. `.styleName`, below the title of paragraph **without** blank lines.

Each class can be written in different lines:

```css
.styleName1
.styleName2
...
.styleNameN
```

##### Standard styles

- `.default` default style (it can be omitted)
- `.todo` paragraph which must be written in future
- `.note` paragraph which contains a note
- `.warning` paragraph which contains a warning
