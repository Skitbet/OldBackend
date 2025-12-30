use aws_sdk_s3::config::endpoint::{Endpoint, EndpointFuture, ResolveEndpoint};

#[derive(Debug)]
pub struct CustomEndpointResolver {
    pub endpoint: String,
}

impl ResolveEndpoint for CustomEndpointResolver {
    fn resolve_endpoint<'a>(
        &'a self,
        _params: &'a aws_sdk_s3::config::endpoint::Params,
    ) -> EndpointFuture<'a> {
        let endpoint = Endpoint::builder()
            .url(self.endpoint.clone())
            .build();
        EndpointFuture::ready(Ok(endpoint))
    }
}
