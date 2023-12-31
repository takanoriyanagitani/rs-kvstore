use tonic::{Request, Response, Status};

use crate::rpc::{
    DelRequest, DelResponse, DropRequest, DropResponse, ExistsRequest, ExistsResponse, GetRequest,
    GetResponse, InsertRequest, InsertResponse, SetRequest, SetResponse, TruncateRequest,
    TruncateResponse,
};

pub trait KeyValue {
    fn get(&self, req: Request<GetRequest>) -> Result<Response<GetResponse>, Status>;
    fn exists(&self, req: Request<ExistsRequest>) -> Result<Response<ExistsResponse>, Status>;

    fn set(&mut self, req: Request<SetRequest>) -> Result<Response<SetResponse>, Status>;
    fn del(&mut self, req: Request<DelRequest>) -> Result<Response<DelResponse>, Status>;
    fn truncate(
        &mut self,
        req: Request<TruncateRequest>,
    ) -> Result<Response<TruncateResponse>, Status>;
    fn drop(&mut self, req: Request<DropRequest>) -> Result<Response<DropResponse>, Status>;
    fn insert(&mut self, req: Request<InsertRequest>) -> Result<Response<InsertResponse>, Status>;
}
