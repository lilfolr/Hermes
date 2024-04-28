.PHONY: all build docker docker_device uninstall install reinstall

all: build docker

reinstall: uninstall install

install: docker_device
	kind load docker-image hermes_device
	cd device_operator/helm_chart/hermes && helm install hermes . --namespace=hermes --create-namespace --atomic

uninstall:
	cd device_operator/helm_chart/hermes && helm uninstall hermes --namespace=hermes || true

build: 
	cargo build --all

docker: docker_device

docker_device:
	docker build -f device.dockerfile . -t hermes_device:0.0.1
