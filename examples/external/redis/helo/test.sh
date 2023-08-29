#!/bin/sh

svc=127.0.0.1:2951
protodir=proto

rget(){
	echo '{}' |
	jq -c '{
		request_id: {
			hi: 20230829,
			lo: 110138,
		},
		key: {
			k: "aGVs",
		},
	}' |
	grpcurl \
		-plaintext \
		-d @ \
		-import-path "${protodir}" \
		-proto rkv/v1/kvstore.proto \
		"${svc}" \
		rkv.v1.KeyValService/Get
}

rset(){
	echo '{}' |
	jq -c '{
		request_id: {
			hi: 20230829,
			lo: 110138,
		},
		key: {
			k: "aGVs",
		},
		val: {
			v: "d3Js",
		},
	}' |
	grpcurl \
		-plaintext \
		-d @ \
		-import-path "${protodir}" \
		-proto rkv/v1/kvstore.proto \
		"${svc}" \
		rkv.v1.KeyValService/Set
}

rset
rget
