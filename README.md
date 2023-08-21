# lori

[![Code Size](https://img.shields.io/github/languages/code-size/meysam81/lori)](https://github.com/meysam81/lori)
[![Repo Size](https://img.shields.io/github/repo-size/meysam81/lori)](https://github.com/meysam81/lori)
[![Docs](https://img.shields.io/badge/docs-rs/lori?logo=docs.rs&label=docs.rs)](https://crates.io/crates/lori)
[![Language Count](https://img.shields.io/github/languages/count/meysam81/lori)](https://github.com/meysam81/lori)
[![Commit Intervals](https://img.shields.io/github/commit-activity/m/meysam81/lori)](https://github.com/meysam81/lori/commits)
[![Last Release](https://img.shields.io/github/release-date/meysam81/lori?label=last%20release)](https://github.com/meysam81/lori/releases)
[![GitHub Stars](https://img.shields.io/github/stars/meysam81/lori?label=GitHub%20stars)](https://github.com/meysam81/lori/stargazers)
[![GitHub Release Downloads](https://img.shields.io/github/downloads/meysam81/lori/total?label=GitHub%20Release%20Downloads)](https://github.com/meysam81/lori/releases)
[![Cargo Crate](https://img.shields.io/crates/v/lori)](https://crates.io/crates/lori)
[![Crate Download](https://img.shields.io/crates/d/lori?label=crate%20download)](https://crates.io/crates/lori)
[![Docker pulls](https://img.shields.io/docker/pulls/meysam81/lori?label=Docker%20pulls)](https://hub.docker.com/r/meysam81/lori)
[![Docker Image](https://img.shields.io/docker/image-size/meysam81/lori?label=Docker%20image)](https://hub.docker.com/r/meysam81/lori)
[![License](https://img.shields.io/github/license/meysam81/lori)](https://github.com/meysam81/lori)
[![Lines of Code](https://img.shields.io/tokei/lines/github/meysam81/lori?label=lines%20of%20code)](https://github.com/meysam81/lori)

<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->
**Table of Contents**  *generated with [DocToc](https://github.com/thlorenz/doctoc)*

- [lori](#lori)
  - [Intro](#intro)
  - [How to install it?](#how-to-install-it)
    - [Using cargo](#using-cargo)
    - [Download compiled binary](#download-compiled-binary)
    - [Docker](#docker)
  - [How to use it?](#how-to-use-it)
  - [Todo](#todo)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->

## Intro

This is a simple SMTP server that receives mails and send it to SendGrid.

An improved version might include different incoming protocols and different
outgoing integrations.

Note that SendGrid already supports receiving messages from SMTP, but I didn't
find it before writing this app.

The format for [sending an email through SMTP to SendGrid][sendgrid-smtp] is as below:

```plaintext
smtps://apikey:SENDGRID_API_KEY@smtp.sendgrid.net:465/
```

## How to install it?

### Using cargo

```bash
cargo install lori
```

### Download compiled binary

You can download the latest release from the [releases page][release] for your
platform.

### Docker

```bash
docker run -p 2525:2525 -e SENDGRID_API_KEY=your-api-key meysam81/lori
```

## How to use it?

```bash
export SENDGRID_API_KEY=your-api-key
lori  # listening on localhost:2525
```

[sendgrid-smtp]: https://docs.sendgrid.com/for-developers/sending-email/getting-started-smtp
[release]: https://github.com/meysam81/lori/releases/latest

## Todo

- [ ] The docker image is stopped after processing the first email!
