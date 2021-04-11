# Bark

[![Rust](https://github.com/jcbcn/bark/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/jcbcn/bark/actions/workflows/rust.yml)

Bark is a Zettelkasten note taking tool that uses markdown and your local filesystem. 

## Features

- Daily notes
- Templates
- Tags
- Linked references
- Unlinked references
- Extension engine

## Architecture

- bark-core
  - Disk I/O operations
  - Parsing CommonMark
  - Search
    - Indexer
      - Index links in memory against filename and pos <sup>p1</sup>
      - Index each occurence of a word and its filename and pos <sup>p2</sup>
  - Linking
  - Graph <sup>p2</sup>
- bark-frontend
  - Text - glyph-brush
  - Window - winit
  - OpenGl - glutin
