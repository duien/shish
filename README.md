# Shish

There's lots of tools that let you switch between shell prompts. But how many tools help you design your own?

Shish is a toolkit for improving your fish shell. Assemble your own kebab of powerful prompt components, hooked together into just the prompt you want. Each component has a standard config file (or piped in config) and a powerful API to give you exactly the output you want, colorized nicely.

## The juicy bits

So, what tools can you use to customize your prompt? There's built-in components for:

- Ruby version (extra info if you use rbenv)
- Git status
    - Current branch (optional: highlight specific branches)
    - Summarized status: symbols for untracked files, modified files, deleted files, unpushed commits, stashes, commits ahead/behind
- Title component (easily use any components in your terminal's tab or window title)
- Running program
- Return status
- Current directory (with a few display options)
- Current user (ssh, normal, root)

## Whaaaaat?

Portable prompt! There's a little bit of overhead, so you probably don't want to do it unless this is something you've already been wishing for, but if you want, you can compile your prompt pieces into a single binary that you can easily move between systems.

**Note:** This part is actually a complete fantasy.

# Details

- **wrapper:** this will probably work like a format string. Segments(config includes wrapper, empty-text) or plain text bits.
    - There's also the option of "joiners" with can be optionally powerline-style 
- **git-status:** current branch and symbols for lots of different things

