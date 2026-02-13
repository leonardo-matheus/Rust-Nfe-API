//! Schema GraphQL

use async_graphql::{EmptySubscription, Schema};
use super::resolvers::{QueryRoot, MutationRoot};

/// Tipo do schema GraphQL
pub type NfeSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

/// Cria o schema GraphQL
pub fn create_schema() -> NfeSchema {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .enable_federation()
        .finish()
}

/// SDL do schema (para documentação)
pub fn get_sdl() -> String {
    create_schema().sdl()
}
