# Racetrack Plugin: Rust Job Type

This is a plugin for [Racetrack](https://github.com/TheRacetrack/racetrack)
which extends it with Rust Job Type.
It's a language wrapper converting your code written in Rust to a Fatman web service.

## Setup
1. Make sure that current version of language wrapper docker image
  (provided by plugin) is pushed to your Docker registry,
  which is used by your Racetrack instance. 
  - Do it by pushing to public registry: `make push`  
  - or if you want to use private registry, run `make env-template`,
  fill in `.env` file and run `make push-private-registry`.
  - If you wish to work on that locally, also run `make push-local`.

2. [Install racetrack-plugin-bundler](https://github.com/TheRacetrack/racetrack/blob/master/utils/plugin_bundler/README.md)
  and generate ZIP plugin by running `make bundle`.

3. Activate the plugin in Racetrack Dashboard Admin page
  by uploading the zipped plugin file.

## Usage
You can deploy sample Rust job by running:
```bash
racetrack deploy sample-rust-function <RACETRACK_URL>
```
