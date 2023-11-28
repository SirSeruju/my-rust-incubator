mod context;
mod models;
mod mutation;
mod query;

pub use self::context::*;
pub use self::models::*;
pub use self::mutation::*;
pub use self::query::*;

use juniper::{EmptySubscription, RootNode};

/// GraphQL schema type
pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

/// Allows to create [`Schema`]
pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}
