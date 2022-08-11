# Racetrack Plugin: Rust Job Type

This is a plugin for Racetrack which extends it with Rust Job Type.
It's a language wrapper converting your code written in Rust to a Fatman web service.

## Setup
1. Make sure that current version of language wrapper docker image
  (provided by plugin) is pushed to your Docker registry,
  which is used by your Racetrack instance. 
  Do it by pushing to public registry: `make push`  
  or if you want to use private registry, run `make env-template`,
  fill in `.env` file and run `make push-private-registry`

2. Activate the plugin in Racetrack, 
  add the following to your image-builder configuration (in kustomize ConfigMap):

```yaml
plugins:
- name: rust-job-type
  git_remote: https://github.com/TheRacetrack/plugin-rust-job-type
  git_ref: '1.0.0'
  git_directory: rust-job-type
```

## Usage
You can deploy sample Rust job by running:
```bash
racetrack deploy sample-rust-function <RACETRACK_URL>
```
