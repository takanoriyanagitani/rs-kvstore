#!/bin/sh

protodir=proto
pydir=test.d

python3 \
	-m grpc_tools.protoc \
	-I "${protodir}" \
	--python_out="${pydir}" \
	--pyi_out="${pydir}" \
	--grpc_python_out="${pydir}" \
	proto/rkv/v1/kvstore.proto
