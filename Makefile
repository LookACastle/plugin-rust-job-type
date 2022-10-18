TAG ?= 1.2.0

run:
	cd rust-job-type/rust_wrapper &&\
	FATMAN_NAME=rust-function FATMAN_VERSION=0.0.1 FATMAN_DEPLOYMENT_TIMESTAMP=0 cargo run

perform:
	curl -X POST \
		"http://localhost:7000/pub/fatman/rust-function/latest/api/v1/perform" \
		-H "Content-Type: application/json" \
		-d '{"numbers": [40, 2]}'

build:
	cd rust-job-type &&\
	DOCKER_BUILDKIT=1 docker build \
		-t ghcr.io/theracetrack/racetrack/fatman-base/rust:latest \
		-f base.Dockerfile .

bundle:
	cd rust-job-type &&\
	racetrack plugin bundle --plugin-version=${TAG} --out=..

deploy-sample:
	racetrack deploy sample-rust-function docker
