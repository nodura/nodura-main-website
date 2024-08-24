.PHONY: all backend frontend

all: build-backend build-frontend

build-backend:
	@echo "Building Actix backend..."
	cd back-end && cargo build --release

build-frontend:
	@echo "Building Yew frontend..."
	cd front-end && trunk build --release

