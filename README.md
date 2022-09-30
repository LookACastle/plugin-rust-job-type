# Racetrack Plugin: Rust Job Type

This is a plugin for [Racetrack](https://github.com/TheRacetrack/racetrack)
which extends it with Rust Job Type.
It's a language wrapper converting your code written in Rust to a Fatman web service.

## Setup
1. [Install racetrack-plugin-bundler](https://github.com/TheRacetrack/racetrack/blob/master/utils/plugin_bundler/README.md)
  and generate ZIP plugin by running `make bundle`.

2. Activate the plugin in Racetrack Dashboard Admin page
  by uploading the zipped plugin file.

## Usage
You can deploy sample Rust job by running:
```bash
racetrack deploy sample-rust-function <RACETRACK_URL>
```
