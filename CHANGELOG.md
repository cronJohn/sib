0.1.0
=====

### 🎉 Initial Release

First release of sib.

### 🚀 Features
- fzf-style navigation of notes in a directory
- User can enter search criteria to filter and rank notes
- Implements ranking system to help find notes faster using the search criteria. Ranks by:
    - Path of note slug
    - Markdown frontmatter metadata
        - Tags
        - key: value pairs
    - Usage stats (calculated when using the TUI)
        - How many times the note was opened
        - The last time the note was opened
- Displays which filters are being applied to search for notes (with icons)
- When a note is selected, hitting Enter opens it in your editor of choice. Closing the editor brings you back to sib

### 🎯 Roadmap
This initial release focuses on core functionality. The following areas are planned for future improvements:

- Liveview is actually useful and displays information related to the note (content, metadata, etc.)
- UI needs updating to better utilize space
- Add more icons to the filter panel for common write-up attributes
- Add keyboard shortcuts to more efficiently query (clear all input, delete word, etc.)
