# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Todo before release
 - [ ] Add a C template
 - [ ] Docs (quick start and api reference)
 - [ ] Quality of life improvements to the C interface:
   - [ ] Function for clearing the screen `clear(char, bg, fg)`
   - [ ] Function for drawing text `text(x, y, text)`
   - [ ] Functions for char, bg_col, fg_col at xy coords

### Added
 - New CLI command for quickly and easily generating a project skeleton, as well as packaging up a build (inspired by [WASM-4](https://wasm4.org/))
   
   Create a new project from a template:
   ```
   cavernos new project-name --template rust
   ```
   
   Start a local server running a web build:
   ```
   cavernos run
   ```
   
   Package up the project into a distributable web build:
   ```
   cavernos package
   ```
   
- New AssemblyScript project template

### Changed
 - Replaced parcel bundler with `esbuild`, shaving 20kB off the bundle! The web runtime is now only 59kB (excluding assets).
 - Key inputs are no longer specified in manifest. There is instead a standard set of inputs mapped to 0-254 (ascii values where it makes sense)
