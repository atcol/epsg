DATA_ARCHIVE  := EPSG-v9_8_13-v10model-PostgreSQL.zip
DIR_BUILD     := target
DIR_DATA			:= EPSG-v9_8_13-v10model-PostgreSQL
DIR_RELEASE   := ${DIR_BUILD}/release
BIN_NAME      := "!set_this!"
.DEFAULT_GOAL := all

.PHONY: all
all : build test

.PHONY: init
init :
	rustup toolchain install nightly
	rustup override set nightly
	cargo install cargo-tarpaulin
	cargo install cargo-audit

.PHONY : data
data ${DIR_DATA}/1.sql ${DIR_DATA}/2.sql ${DIR_DATA}/3.sql :
	mv ${DIR_DATA}/PostgreSQL_Table_Script.sql ${DIR_DATA}/1.sql
	mv ${DIR_DATA}/PostgreSQL_Data_Script.sql ${DIR_DATA}/2.sql
	mv ${DIR_DATA}/PostgreSQL_FKey_Script.sql ${DIR_DATA}/3.sql

.PHONY : docker
docker : ${DATA_ARCHIVE}
	docker-compose up

.PHONY : docker-exec
docker-exec :
	docker exec -it $(docker ps |grep postgres | awk '{print $1}') bash

.PHONY: build
build ${DIR_RELEASE}/${BIN_NAME} :
	cargo build --release

.PHONY: test
test :
	cargo test --verbose && \
	  cargo rustdoc

.PHONY: watch
watch :
	cargo-watch -x "test && cargo rustdoc"
