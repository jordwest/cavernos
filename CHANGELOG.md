# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Todo before release
 - [ ] Add a C template
 - [ ] Add an AssemblyScript template
 - [ ] Add a `serve` and `--watch` command that serves wasm with correct MIME type, and hot reloads assets
 - [ ] Quality of life improvements to the C interface:
   - [ ] Function for clearing the screen `clear(char, bg, fg)`
   - [ ] Function for drawing text `text(x, y, text)`
   - [ ] Functions for char, bg_col, fg_col at xy coords
   - [ ] Remove input map from manifest, have a standard set of inputs mapped to 0-254 (ascii values where it makes sense)

### Added
 - New CLI command for quickly and easily generating a project skeleton, as well as packaging up a build (inspired by [WASM-4](https://wasm4.org/))
   
   Create a new project from a template:
   ```
   cavernos new project-name --template rust
   ```
   
   Package up the project into a web build:
   ```
   cavernos package
   ```
