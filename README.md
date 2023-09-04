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

#### Code block

It's possible to specify the language used in code block, as in Commonmark, writing language name after the ```.

```markdown
```python
# code block
print("Write all documents in NMD!!")
```
```