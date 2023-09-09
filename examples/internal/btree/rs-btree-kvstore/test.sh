#!/bin/sh

listen_port=9151
listen_addr=127.0.0.1:${listen_port}
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

bdrop(){
	echo '{}' |
	jq -c '{
		request_id: {
			hi: 20230908,
			lo: 85358,
		},
		bucket: { b: "earth" },
	}' |
	grpcurl \
		-plaintext \
		-d @ \
		-import-path "${protodir}" \
		-proto rkv/v1/kvstore.proto \
		"${listen_addr}" \
		rkv.v1.KeyValService/Drop
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
bget
bdrop
bget

grpc-health-probe -addr=:${listen_port}
