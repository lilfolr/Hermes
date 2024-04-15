.PHONY: all build docker docker_device

all: build docker


build: 
	cargo build --all

docker: docker_device

docker_device:
	docker build -f device.dockerfile . -t hermes_device:latest
