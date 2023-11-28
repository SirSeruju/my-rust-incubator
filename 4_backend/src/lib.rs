#![warn(missing_docs)]
//! Simple [`GraphQL`] API server implements:
//! - Ability to register users.
//! - Ability to authenticate users.
//! - Ability to retrieve a single user and all its friends (with their friends) (should require authorization).
//! - Ability to add some user to friends list and remove from there (should require authorization).
//!
//! uses [`juniper`] for [`GraphQL schema`] and [`diesel`] for [`orm`], [`migrations`] and [`DB schema`]
//!
//! [`juniper`]: https://docs.rs/juniper
//! [`diesel`]: https://docs.rs/diesel
//! [`GraphQL`]: https://graphql.org/
//! [`GraphQL schema`]: https://graphql.org/learn/schema/
//! [`orm`]: https://wikipedia.org/wiki/ORM
//! [`DB schema`]: https://wikipedia.org/wiki/Database_schema
//! [`migrations`]: https://wikipedia.org/wiki/Schema_migration

/// Implements graphql queries, mutations and context
pub mod graphql;
/// Database schema
pub mod schema;
/// Token storage, allows to generate token and maps it to users
pub mod token_storage;
