use lambda_http::{
    start,
    Handler,
    Request,
    Response,
    IntoResponse,
    Body
};
use lambda_runtime::{
    Context,
    error::HandlerError
};

struct Subscribe {}

impl Handler<Response<Body>> for Subscribe {
    fn run(&mut self, _: Request, _: Context) -> 
        Result<Response<Body>, HandlerError> {
        Ok("Subscribe".into_response())
    }
}

#[tokio::main]
async fn main() -> Result<(), HandlerError> {
    let handler = Subscribe {};
    start(handler, None);
    Ok(())
}