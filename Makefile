all: run

run: npmbuild mod generate swaggerfast
	showme watch	

generate: stringer updated
	go generate

stringer:
	go get golang.org/x/tools/cmd/stringer

npmbuild: clean
	cd frontend && npm install && npm run build:prod && mv dist ../asset/dashboard
	cd d2admin && npm install && npm run build && mv dist ../asset/d2admin

log:
	fhst tool log

migrate:
	go run main.go migrate

mod:
	go mod tidy

gowatch:
	go get github.com/silenceper/gowatch

swagger:
	go get -u github.com/swaggo/swag/cmd/swag
	swag init
	go mod tidy

swaggerfast:
	swag init
	go mod tidy

vendor:
	go mod vendor

clean:
	rm -rf asset/dashboard
	rm -rf asset/d2admin

tauridev:
	cd frontend && cargo tauri dev

tauribuild:
	cd frontend && cargo tauri build

updated:
	go get -u github.com/go-eden/routine