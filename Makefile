TAG ?= 1.0.0

-include .env

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

push: build
	docker tag ghcr.io/theracetrack/racetrack/fatman-base/rust:latest ghcr.io/theracetrack/racetrack/fatman-base/rust:$(TAG)
	docker push ghcr.io/theracetrack/racetrack/fatman-base/rust:$(TAG)

push-local: build
	docker tag ghcr.io/theracetrack/racetrack/fatman-base/rust:latest localhost:5000/theracetrack/racetrack/fatman-base/rust:$(TAG)
	docker push localhost:5000/theracetrack/racetrack/fatman-base/rust:$(TAG)

push-private-registry: build
	docker tag ghcr.io/theracetrack/racetrack/fatman-base/rust:latest ${REGISTRY}/fatman-base/rust:$(TAG)
	docker push ${REGISTRY}/fatman-base/rust:$(TAG)

push-all: push push-local push-private-registry

env-template:
	cp -n .env.dist .env
	@echo "Now fill in the .env file with your settings"
