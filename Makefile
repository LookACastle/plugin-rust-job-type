TAG ?= 1.2.0

run:
	cd rust-job-type/rust_wrapper &&\
	JOB_NAME=rust-function JOB_VERSION=0.0.1 JOB_DEPLOYMENT_TIMESTAMP=0 cargo run

perform:
	curl -X POST \
		"http://localhost:7000/pub/job/rust-function/latest/api/v1/perform" \
		-H "Content-Type: application/json" \
		-d '{"numbers": [40, 2]}'

build:
	cd rust-job-type &&\
	DOCKER_BUILDKIT=1 docker build \
		-t ghcr.io/theracetrack/racetrack/job-base/rust:latest \
		-f base.Dockerfile .

bundle:
	cd rust-job-type &&\
	racetrack plugin bundle --plugin-version=${TAG} --out=..

deploy-sample:
	racetrack deploy sample-rust-function
