use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    Schema,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    response::{Html, IntoResponse},
    Extension,
};

pub async fn graphql_playground(path: &str) -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new(path)))
}

pub async fn graphiql_handler(path: &str) -> impl IntoResponse {
    Html(
        async_graphql::http::GraphiQLSource::build()
            .endpoint(path)
            .finish(),
    )
}

pub async fn graphql_handler_with_auth<Query, Mutation, Subscription, Token>(
    schema: Extension<Schema<Query, Mutation, Subscription>>,
    token: Extension<Token>,
    req: GraphQLRequest,
) -> GraphQLResponse
where
    Query: async_graphql::ObjectType + 'static,
    Mutation: async_graphql::ObjectType + 'static,
    Subscription: async_graphql::SubscriptionType + 'static,
    Token: Send + Sync + 'static,
{
    let mut req = req.into_inner();
    req = req.data(token.0);
    schema.execute(req).await.into()
}

pub async fn graphql_handler<Query, Mutation, Subscription>(
    schema: Extension<Schema<Query, Mutation, Subscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse
where
    Query: async_graphql::ObjectType + 'static,
    Mutation: async_graphql::ObjectType + 'static,
    Subscription: async_graphql::SubscriptionType + 'static,
{
    let req = req.into_inner();
    schema.execute(req).await.into()
}
