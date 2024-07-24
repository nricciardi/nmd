## Develop

### Features in Progress for current version

nothing

### Features done

- [x] Use file name instead of absolute path in dossier configuration
- [x] Other sections in dossier configuration to manage all options
- [x] Local math (no CDN)
- [x] List
- [x] List creation check
- [x] Link with identifier
- [x] Link to chapters
- [x] Quote
- [x] Image caption
- [x] Image URL fix meta-characters
- [x] Multiple image in a single row
- [x] Set image dimensions
- [x] Image compression
- [x] Image in dossier: default path to `assets/images`
- [x] Parse image caption 
- [x] Focus block
- [x] Light base page style
- [x] Embedded style
- [x] Embedded chapter style
- [x] Custom style files
- [x] Tables
- [x] Embedded Greek letters
- [x] Fix single list item
- [x] Todo modifier with only `todo` or `TODO`
- [x] Todo modifier with text between `TODO:` and `:TODO`
- [x] Relative header (e.g. `#+` to indicate precedent header level + 1, `#=` to indicate same header level of precedente header)
- [x] Short image modifier (without alt)
- [x] `nmd dossier add` auto-add `.nmd`
- [x] `nmd dossier add` accept more than one file
- [x] Escape
- [x] Metadata
- [x] Reference
- [x] Table of contents without page numbers
- [x] Bibliography
- [x] Compile only modified chapters in watcher mode
- [x] Web server to refresh compiled output

### Planned Features

- [ ] All modifiers
- [ ] `* words *`
- [x] Use `getset` crate
- [ ] embed_remote_image
- [ ] Possibility to use a different dossier configuration name
- [ ] PDF compile format
- [x] Vintage style (typewriter)
- [x] Dark style
- [ ] Run code
- [ ] Video
- [x] Scientific style
- [ ] Linkify (convert URL-like strings in links)
- [x] Fast draft (prevent to parse time consuming parts)
- [ ] Dynamics addons (e.g. katex iff math is used)
- [ ] Watcher mode for single file compilation
- [ ] Split CLI lib from compiler
- [x] Compile only a subset of documents
- [ ] Paper format support (A3, A5, ...)
- [x] MD to NMD converter
- [x] Include all .nmd file in running directory as default option in dossier configuration
- [x] Compile single files
- [ ] Table of contents with page numbers
- [ ] Select position of ToC and Bibliography
- [ ] Cover page
- [ ] VS Code extension (https://dev.to/salesforceeng/how-to-build-a-vs-code-extension-for-markdown-preview-using-remark-processor-1169)
