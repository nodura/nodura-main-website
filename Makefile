.PHONY: all backend frontend

all: build-backend build-frontend serve

build-backend:
	@echo "Building Actix backend..."
	cd backend && cargo build --release

build-frontend:
	@echo "Building Yew frontend..."
	trunk build --release

serve: 
	@echo "Serving Yew frontend..."
	trunk serve --open

check-css-file:
	@if [ ! -f ./frontend/styles/main.css ]; then \
		echo "Creating main.css..."; \
		mkdir -p ./frontend/styles; \
		touch ./frontend/styles/main.css; \
	fi
