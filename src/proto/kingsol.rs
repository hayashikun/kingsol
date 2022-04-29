#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Link {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub link: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRequest {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetResponse {
    #[prost(message, optional, tag = "1")]
    pub link: ::core::option::Option<Link>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRequest {}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListResponse {
    #[prost(message, repeated, tag = "1")]
    pub links: ::prost::alloc::vec::Vec<Link>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRequest {
    #[prost(message, optional, tag = "1")]
    pub link: ::core::option::Option<Link>,
    #[prost(bool, tag = "2")]
    pub r#override: bool,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateResponse {}

/// Generated server implementations.
pub mod kingsol_api_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]

    use tonic::codegen::*;

    ///Generated trait containing gRPC methods that should be implemented for use with KingsolApiServer.
    #[async_trait]
    pub trait KingsolApi: Send + Sync + 'static {
        async fn get(
            &self,
            request: tonic::Request<super::GetRequest>,
        ) -> Result<tonic::Response<super::GetResponse>, tonic::Status>;
        async fn list(
            &self,
            request: tonic::Request<super::ListRequest>,
        ) -> Result<tonic::Response<super::ListResponse>, tonic::Status>;
        async fn create(
            &self,
            request: tonic::Request<super::CreateRequest>,
        ) -> Result<tonic::Response<super::CreateResponse>, tonic::Status>;
    }

    #[derive(Debug)]
    pub struct KingsolApiServer<T: KingsolApi> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }

    struct _Inner<T>(Arc<T>);

    impl<T: KingsolApi> KingsolApiServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
            where
                F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }

    impl<T, B> tonic::codegen::Service<http::Request<B>> for KingsolApiServer<T>
        where
            T: KingsolApi,
            B: Body + Send + 'static,
            B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/kingsol.KingsolAPI/Get" => {
                    #[allow(non_camel_case_types)]
                    struct GetSvc<T: KingsolApi>(pub Arc<T>);
                    impl<T: KingsolApi> tonic::server::UnaryService<super::GetRequest>
                    for GetSvc<T> {
                        type Response = super::GetResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/kingsol.KingsolAPI/List" => {
                    #[allow(non_camel_case_types)]
                    struct ListSvc<T: KingsolApi>(pub Arc<T>);
                    impl<T: KingsolApi> tonic::server::UnaryService<super::ListRequest>
                    for ListSvc<T> {
                        type Response = super::ListResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/kingsol.KingsolAPI/Create" => {
                    #[allow(non_camel_case_types)]
                    struct CreateSvc<T: KingsolApi>(pub Arc<T>);
                    impl<T: KingsolApi> tonic::server::UnaryService<super::CreateRequest>
                    for CreateSvc<T> {
                        type Response = super::CreateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }

    impl<T: KingsolApi> Clone for KingsolApiServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }

    impl<T: KingsolApi> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }

    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }

    impl<T: KingsolApi> tonic::transport::NamedService for KingsolApiServer<T> {
        const NAME: &'static str = "kingsol.KingsolAPI";
    }
}
