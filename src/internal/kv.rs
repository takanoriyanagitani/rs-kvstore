use tonic::{Request, Response, Status};

use crate::rpc::{ExistsRequest, ExistsResponse, GetRequest, GetResponse, SetRequest, SetResponse};

pub trait KeyValue {
    fn get(&self, req: Request<GetRequest>) -> Result<Response<GetResponse>, Status>;
    fn exists(&self, req: Request<ExistsRequest>) -> Result<Response<ExistsResponse>, Status>;

    fn set(&mut self, req: Request<SetRequest>) -> Result<Response<SetResponse>, Status>;
}
