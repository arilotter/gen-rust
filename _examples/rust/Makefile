all:
	@echo "please read Makefile source or README to see available commands"

bootstrap:
	rm -rf server/node_modules
	cd server && pnpm i

generate:
	webrpc-gen -schema=service.ridl -target=../../ -client -out=./client/src/gen.rs
	webrpc-gen -schema=service.ridl -target=typescript -server -out=./server/server.gen.ts

run-client:
	cd client && cargo run

run-server:
	cd server && pnpm start
