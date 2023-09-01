use tonic::{Request, Response, Status};

use crate::rpc::{GetRequest, GetResponse, SetRequest, SetResponse};

pub trait KeyValue {
    fn get(&self, req: Request<GetRequest>) -> Result<Response<GetResponse>, Status>;
    fn set(&mut self, req: Request<SetRequest>) -> Result<Response<SetResponse>, Status>;
}
