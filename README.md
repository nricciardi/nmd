# New Markdown (NMD)
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


### Paragraph modifiers

The _paragraph modifier_ modifies the style of the paragraph in which are applied.

> Each paragraph **must** be separated by a blank line. 

#### Heading

Create headings using '#' (up to 6 levels).

```markdown
# Heading 1
## Heading 2
...
###### Heading 6
```

#### Line separator

To apply a line separator use --- or *** in a new blank line.

#### List

Different types of list are supported in NMD, below the list with modifier

- `-` common list
- `*` second style list
- `-[] or -[ ]` todo list
- `1. or 1)` ordered list

Using `tabs` you can create different list levels.

#### Code block

Code blocks use ``` as paragraph modifier.

It's possible to specify the language used in code block, as in Commonmark, writing language name after first quotes.

```markdown
# code block
print("Write all documents in NMD!!")
```

The list with supported languages (tag in parentesis):
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