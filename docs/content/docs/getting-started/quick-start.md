+++
title = "Quick Start"
description = "5 minute guide to setting up a project and rendering to the screen"
date = 2022-01-01T18:00:00+00:00
updated = 2022-01-01T18:00:00+00:00
draft = false
weight = 30
sort_by = "weight"
template = "docs/page.html"

[extra]
lead = "5 minute guide to setting up a project and rendering to the screen"
toc = true
top = false
+++

## Requirements

To complete the quick start guide, you first need to [install the command line tool](@/docs/getting-started/installation.md).

## Create a project

This command will create a new directory for `my-project` containing everything you need to build a hello world application.

Replace `rust` with a supported project template, currently `rust` or `assemblyscript`.

```bash
cavernos new my-project --template rust
```

## Build your application

This is dependent on the language template you chose. Follow the instructions in the `README.md` inside
your project folder to build the WebAssembly module.

For example, if you chose the `rust` template you would run

```bash
cargo build --release
```

## Run it

To start a local web server that hosts your project, navigate to your project directory and run:

```bash
cavernos run
```

Then open a browser to http://localhost:8088/ and you should see your project running.

## Publish it to the web

When you're ready to distribute your project on the web, run the `package` command in your project directory:

```bash
cavernos package
```

This will create a `dist` directory in your project folder, which contains everything you need to host
the app on the web.

Upload these files to any web server and it should just work.

A great place to freely publish games on the web is [itch.io](https://itch.io/). Just zip up the `dist` directory
and upload it straight to itch.io.

Note that for security reasons, browsers won't run the WebAssembly if you simply open `index.html` on your
local filesystem. You'll need to serve them using a local web server.