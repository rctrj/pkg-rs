#[macro_export]
macro_rules! graphql_handler {
    ($method_name: ident, $schema_name: ident) => {
        use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
        use axum::extract::Extension;

        pub(crate) async fn $method_name(
            schema: axum::extract::Extension<$schema_name>,
            req: GraphQLRequest,
        ) -> GraphQLResponse {
            let response = async move { schema.execute(req.into_inner()).await }.await;
            response.into()
        }
    };
}

#[macro_export]
macro_rules! graphiql_handler {
    ($path:expr) => {
        use axum::response::IntoResponse;

        pub(crate) async fn graphiql() -> impl IntoResponse {
            axum::response::Html(
                async_graphql::http::GraphiQLSource::build()
                    .endpoint($path)
                    .finish(),
            )
        }
    };
}
