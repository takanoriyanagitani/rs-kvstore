import grpc

from rkv.v1 import kvstore_pb2_grpc
from rkv.v1.kvstore_pb2 import SetRequest, GetRequest, Uuid, Key, Val, Bucket

ADDR = "127.0.0.1:2951"

with grpc.insecure_channel(ADDR) as chan:
	stub = kvstore_pb2_grpc.KeyValServiceStub(chan)

	res = stub.Set(SetRequest(
		request_id = Uuid(hi=20230830, lo=94839),
		bucket     = Bucket(b="earth"),
		key        = Key(k=b"helo"),
		val        = Val(v=b"wrld"),
	))
	print(res)

	res = stub.Set(SetRequest(
		request_id = Uuid(hi=20230831, lo=85532),
		bucket     = Bucket(b="moon"),
		key        = Key(k=b"helo"),
		val        = Val(v=b"hell"),
	))
	print(res)

	res = stub.Get(GetRequest(
		request_id = Uuid(hi=20230830, lo=95020),
		bucket     = Bucket(b="earth"),
		key        = Key(k=b"helo"),
	))
	print(res)

	res = stub.Get(GetRequest(
		request_id = Uuid(hi=20230830, lo=95020),
		bucket     = Bucket(b="moon"),
		key        = Key(k=b"helo"),
	))
	print(res)

	pass
