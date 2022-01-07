# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
 - New CLI app for quickly and easily generating a project skeleton, as well as packaging up a build (inspired by [WASM-4](https://wasm4.org/))
   
   Create a new project from a template:
   ```
   cavernos new project-name --template rust
   ```
   
   Package up the project into a web build:
   ```
   cavernos package
   ```
