/// Implements graphql queries, mutations and context
mod context;
mod models;
mod mutation;
mod query;

pub use self::context::*;
pub use self::models::*;
pub use self::mutation::*;
pub use self::query::*;

use juniper::{EmptySubscription, RootNode};

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new())
}
