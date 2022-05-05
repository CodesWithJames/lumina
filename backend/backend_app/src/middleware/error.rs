use tide::{Request, Next, Middleware, utils::async_trait};
use tracing::{span, Level, Instrument};

pub struct ErrorMiddleware;

#[async_trait]
impl Middleware<()> for ErrorMiddleware {
    async fn handle(&self, req: Request<()>, next: Next<'_, ()>) -> tide::Result {

        let res = next.run(req).await;
        println!("{:?}", res);
        Ok(res)
    }
}

// #[async_trait]
// impl Middleware<()> for ErrorMiddleware {
//     async fn handle(&self, ctx: Context<()>, next: Next<()>) -> Result<(), Error> {
//         match next.run(req).await {
//             Ok(()) => {}.
//             Err(e) => {
//                 ctx.res.status = match e.status_code() {
//                     Some(status) => status,
//                     None => 500,
//                 };

//                 ctx.res.body = e.to_string();
//             }
//         }

//         Ok(())
//     }
// }