## Develop

### Planned Features

- [ ] All modifiers
- [ ] Possibility to use a different dossier configuration name
- [ ] PDF compile format
- [ ] Vintage style (typewriter)
- [ ] Dark style
- [ ] Table of Content
- [ ] Run code
- [ ] Video
- [ ] Bibliography
- [ ] Scientific style
- [ ] Linkify (convert URL-like strings in links)

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
- [ ] Multiple image in a single row
- [ ] Set image dimensions
- [ ] Image compression
- [x] Image in dossier: default path to `assets/images`
- [x] Focus block
- [x] Light base page style
- [ ] Paper format support (A4, A5, ...)
- [ ] Footer with page counter
- [x] Embedded style
- [ ] Embedded chapter style
- [x] Custom style files
- [ ] Tables
- [ ] Embedded Greek letters
- [x] Fix single list item
- [x] Todo modifier with only `todo` or `TODO`
- [ ] Todo modifier with text between `TODO:` and `:TODO`
- [ ] Relative header (e.g. `#+` to indicate precedent header level + 1, `#=` to indicate same header level of precedente header)
- [x] `nmd dossier add` auto-add `.nmd`
- [x] `nmd dossier add` accept more than one file

### Known issues

- [x] `nmd dossier add` transform relative paths to absolute paths
- [x] `nmd dossier add` save only 2 paths
