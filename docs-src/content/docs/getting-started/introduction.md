+++
title = "Introduction"
description = "CavernOS is a fantasy terminal for building classic DOS-era classic ASCII games and demos"
date = 2022-01-15T18:00:00+00:00
updated = 2022-01-15T18:00:00+00:00
draft = false
weight = 10
sort_by = "weight"
template = "docs/page.html"

[extra]
#lead = 'CavernOS is a fantasy terminal for building classic DOS-era classic ASCII games and demos.'
toc = true
top = false
+++

## What is it?

CavernOS is a DOS-like [fantasy console](https://en.wikipedia.org/wiki/Fantasy_video_game_console) designed from the ground up with simplicity, ease of use and performance in mind for 
ASCII games, and it runs blazing fast in a browser. It's great for building nostalgic roguelikes, text adventures, or prototyping mechanics
for your next game jam.

It's not an operating system in the traditional sense, it's more like an emulator for a terminal that never existed. Your
code compiles to WebAssembly which then runs in a completely sandboxed environment with access to only a few simple system calls for drawing
characters on the screen, getting input and storing data so you can skip the boring bits and get right to building something
fun.

CavernOS completely abstracts away the complexity of font sprite rendering, parsing input consistently across devices, weird historical
terminal obscurities and removes the need to compile for different operating systems or CPU architectures.

## What is WebAssembly?

[WebAssembly](https://webassembly.org/) is a new type of machine code designed to run virtually anywhere (not just the web!).
It's now supported in all modern browsers, and allows near-native performance on different CPU architectures
and operating systems with a single binary format.

CavernOS makes use of WebAssembly as a compilation target. You compile your code to a WebAssembly module (instead of for example an x86
or ARM binary). That code then runs using the browser's WebAssembly runtime, while CavernOS provides your module with some functions for
rendering, input etc. Currently there is only a web based CavernOS host runtime, but this could in future be expanded to native OS apps,
smartphone apps, even terminal apps - and you wouldn't even need to recompile your code.

## Install the command line tool

The optional command line tool simplifies creating and packaging projects. [Installation →](@/docs/getting-started/installation.md)

## Quick Start

Create a new project and render something to the screen in 5 minutes. [Quick Start →](@/docs/getting-started/quick-start.md)

## Source Code

Source code is available at [GitHub →](https://github.com/jordwest/cavernos).
