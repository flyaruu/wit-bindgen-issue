use bindings::api::{exports::wasi::http::incoming_handler::Guest, wasi::http::types::{IncomingRequest, ResponseOutparam}};

struct MyComponent {}


impl Guest for MyComponent {
fn handle(request:IncomingRequest,response_out:ResponseOutparam,) -> () {
        todo!()
    }
}

bindings::api::export!(MyComponent);
