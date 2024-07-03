## Develop

### Features in Progress for current version

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
- [ ] Cover page
- [ ] Bibliography

### Known issues

- [x] `nmd dossier add` transform relative paths to absolute paths
- [x] `nmd dossier add` save only 2 paths
- [x] Chapter body are not parsed on Windows
- [ ] `* words *` works
- [ ] Table footer doesn't use `tfoot` in HTML format (caused by `build_html` crate)

### Planned Features

- [ ] All modifiers
- [ ] Possibility to use a different dossier configuration name
- [ ] PDF compile format
- [ ] Vintage style (typewriter)
- [ ] Dark style
- [ ] Run code
- [ ] Video
- [ ] Scientific style
- [ ] Linkify (convert URL-like strings in links)
- [x] Fast draft (prevent to parse time consuming parts)
- [ ] Dynamics addons (e.g. katex iff math is used)
- [x] Watcher mode
- [ ] Split CLI lib from compiler
- [ ] Compile only modified chapters in watcher mode
- [x] Compile only a subset of documents
- [ ] Paper format support (A3, A5, ...)
- [x] MD to NMD converter
- [x] Include all .nmd file in running directory as default option in dossier configuration
- [ ] Compile single files
- [ ] Table of contents with page numbers

