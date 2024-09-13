.PHONY: all backend frontend

all: build-backend build-frontend serve

build-backend:
	@echo "Building Actix backend..."
	cd back-end && cargo build --release

build-frontend:
	@echo "Building Yew frontend..."
	cd front-end && trunk build --release

serve: check-css-file
	@echo "Serving Yew frontend..."
	cd front-end && trunk serve --open

check-css-file:
	@if [ ! -f ./front-end/styles/main.css ]; then \
		echo "Creating main.css..."; \
		mkdir -p ./front-end/styles; \
		touch ./front-end/styles/main.css; \
	fi
