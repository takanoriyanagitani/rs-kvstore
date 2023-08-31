from google.protobuf import timestamp_pb2 as _timestamp_pb2
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class Uuid(_message.Message):
    __slots__ = ["hi", "lo"]
    HI_FIELD_NUMBER: _ClassVar[int]
    LO_FIELD_NUMBER: _ClassVar[int]
    hi: int
    lo: int
    def __init__(self, hi: _Optional[int] = ..., lo: _Optional[int] = ...) -> None: ...

class Bucket(_message.Message):
    __slots__ = ["b"]
    B_FIELD_NUMBER: _ClassVar[int]
    b: str
    def __init__(self, b: _Optional[str] = ...) -> None: ...

class Key(_message.Message):
    __slots__ = ["k"]
    K_FIELD_NUMBER: _ClassVar[int]
    k: bytes
    def __init__(self, k: _Optional[bytes] = ...) -> None: ...

class Val(_message.Message):
    __slots__ = ["v"]
    V_FIELD_NUMBER: _ClassVar[int]
    v: bytes
    def __init__(self, v: _Optional[bytes] = ...) -> None: ...

class GetRequest(_message.Message):
    __slots__ = ["request_id", "bucket", "key"]
    REQUEST_ID_FIELD_NUMBER: _ClassVar[int]
    BUCKET_FIELD_NUMBER: _ClassVar[int]
    KEY_FIELD_NUMBER: _ClassVar[int]
    request_id: Uuid
    bucket: Bucket
    key: Key
    def __init__(self, request_id: _Optional[_Union[Uuid, _Mapping]] = ..., bucket: _Optional[_Union[Bucket, _Mapping]] = ..., key: _Optional[_Union[Key, _Mapping]] = ...) -> None: ...

class GetResponse(_message.Message):
    __slots__ = ["val", "got"]
    VAL_FIELD_NUMBER: _ClassVar[int]
    GOT_FIELD_NUMBER: _ClassVar[int]
    val: Val
    got: _timestamp_pb2.Timestamp
    def __init__(self, val: _Optional[_Union[Val, _Mapping]] = ..., got: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ...) -> None: ...

class SetRequest(_message.Message):
    __slots__ = ["request_id", "bucket", "key", "val"]
    REQUEST_ID_FIELD_NUMBER: _ClassVar[int]
    BUCKET_FIELD_NUMBER: _ClassVar[int]
    KEY_FIELD_NUMBER: _ClassVar[int]
    VAL_FIELD_NUMBER: _ClassVar[int]
    request_id: Uuid
    bucket: Bucket
    key: Key
    val: Val
    def __init__(self, request_id: _Optional[_Union[Uuid, _Mapping]] = ..., bucket: _Optional[_Union[Bucket, _Mapping]] = ..., key: _Optional[_Union[Key, _Mapping]] = ..., val: _Optional[_Union[Val, _Mapping]] = ...) -> None: ...

class SetResponse(_message.Message):
    __slots__ = ["set"]
    SET_FIELD_NUMBER: _ClassVar[int]
    set: _timestamp_pb2.Timestamp
    def __init__(self, set: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ...) -> None: ...
