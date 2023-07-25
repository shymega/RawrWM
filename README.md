<!--
SPDX-FileCopyrightText: 2023 The RawrWM Developers

SPDX-License-Identifier: Apache-2.0
-->

# RawrWM

[![standard-readme compliant](https://img.shields.io/badge/standard--readme-OK-green.svg?style=flat-square)](https://github.com/RichardLitt/standard-readme)

<!-- TODO: Put more badges here. -->

An adaptable, fast and flexible window manager for various platforms - and
configured with GNU Guile!

# Description

This is an experiment in window managers, where the same language - GNU Guile! - is used to configure it - with a standardised gRPC interface for control.

It [will] comes with an out of the box configuration, and users are encouraged to tinker to their heart's desire! Control is done via the `rawrctl` executable, which [will] run on Windows, Linux, \*BSD, and macOS.

RawrWM is based around 'backends'. The idea is to have a unifying API exposed by Guile, and to be able to use the same configuration acrsos platforms. This includes Microsoft Windows 10/11 (previous versions will *not* be supported), and recent-ish macOS/Linux/\*BSD (TBC).

It's heavily a work in progress, but integration is expected with [automon][automon], another project of mine, and [guile-rs][guile-rs].

Ideas and notes can be seen [here](/docs/NOTES.md)

## Table of Contents

- [Background](#background)
- [Install](#install)
- [Usage](#usage)
- [Maintainers](#maintainers)
- [Contributing](#contributing)
- [License](#license)

## Background

## Install

Not ready for installation yet - keep tuned!

## Usage

Not ready for usage yet - keep tuned!

## Maintainers

[@shymega](https://github.com/shymega)

## Contributing

PRs accepted. Make sure they pass the 'pre-commit' hook checks.

Small note: If editing the README, please conform to the [standard-readme](https://github.com/RichardLitt/standard-readme) specification.

## License

Apache-2.0 © 2023 The RawrWM Developers

[automon]: https://github.com/shymega/automon
[guile-rs]: https://github.com/guile-rs/guile-rs
