pub mod rpc {
    tonic::include_proto!("rkv.v1");
}

pub mod uuid;

pub mod bucket;

pub mod cmd;

pub mod external;
