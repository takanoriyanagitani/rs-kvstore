#!/bin/sh

listen_addr=127.0.0.1:9151
protodir=proto

bset(){
	echo '{}' |
	jq -c '{
		request_id: {
			hi: 20230901,
			lo: 90834,
		},
		bucket: { b: "earth" },
		key: { k: "helo" },
		val: { v: "wrld" },
	}' |
	grpcurl \
		-plaintext \
		-d @ \
		-import-path "${protodir}" \
		-proto rkv/v1/kvstore.proto \
		"${listen_addr}" \
		rkv.v1.KeyValService/Set
}

bget(){
	echo '{}' |
	jq -c '{
		request_id: {
			hi: 20230901,
			lo: 90834,
		},
		bucket: { b: "earth" },
		key: { k: "helo" },
	}' |
	grpcurl \
		-plaintext \
		-d @ \
		-import-path "${protodir}" \
		-proto rkv/v1/kvstore.proto \
		"${listen_addr}" \
		rkv.v1.KeyValService/Get
}

bexists(){
	echo '{}' |
	jq -c '{
		request_id: {
			hi: 20230901,
			lo: 90834,
		},
		bucket: { b: "earth" },
		key: { k: "helo" },
	}' |
	grpcurl \
		-plaintext \
		-d @ \
		-import-path "${protodir}" \
		-proto rkv/v1/kvstore.proto \
		"${listen_addr}" \
		rkv.v1.KeyValService/Exists
}

btruncate(){
	echo '{}' |
	jq -c '{
		request_id: {
			hi: 20230901,
			lo: 90834,
		},
		bucket: { b: "earth" },
	}' |
	grpcurl \
		-plaintext \
		-d @ \
		-import-path "${protodir}" \
		-proto rkv/v1/kvstore.proto \
		"${listen_addr}" \
		rkv.v1.KeyValService/Truncate
}

binsert(){
	echo '{}' |
	jq -c '{
		request_id: {
			hi: 20230903,
			lo: 61715,
		},
		bucket: { b: "earth" },
		key: { k: "helo" },
		val: { v: "wrld" },
	}' |
	grpcurl \
		-plaintext \
		-d @ \
		-import-path "${protodir}" \
		-proto rkv/v1/kvstore.proto \
		"${listen_addr}" \
		rkv.v1.KeyValService/Insert
}

bset
bexists
bget
btruncate
binsert
