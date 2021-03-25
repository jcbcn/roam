# Bark

[![Rust](https://github.com/jcbcn/bark/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/jcbcn/bark/actions/workflows/rust.yml)

Bark is a privacy-first Zettelkasten-style note taking tool using markdown and your local file system 

## Ideas

- Daily notes
- Templates e.g Morning pages
- Tags
- Linked references
- Unlinked references

## Architecture

- bark-core
	- Disk I/O operations
	- Parsing CommonMark
	- Search
		- Indexer
			- Index links in memory against filename and pos <sup>phase 1</sup>
			- Index each occurence of a word and its filename and pos <sup>phase 2</sup>
		- Search
			- Search the index for an exact match
			- Linking
			- Graph
- bark-frontend
	- Text - glyph-brush
	- Window - winit
	- OpenGl - glutin
