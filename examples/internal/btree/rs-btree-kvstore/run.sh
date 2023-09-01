#!/bin/sh

listen_addr=127.0.0.1:9151

ENV_LISTEN="${listen_addr}" \
	./rs-btree-kvstore
