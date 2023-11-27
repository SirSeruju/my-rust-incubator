use crate::models::{Role, UserRole};
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use slug::slugify;

pub async fn create(
    conn: &mut AsyncPgConnection,
    name: String,
    permissions: String,
) -> Result<Role, diesel::result::Error> {
    let new_role = Role {
        slug: slugify(name.clone()),
        name,
        permissions,
    };
    diesel::insert_into(crate::schema::roles::table)
        .values(&new_role)
        .returning(Role::as_returning())
        .get_result(conn)
        .await
}

pub async fn read(
    conn: &mut AsyncPgConnection,
    role_slug: String,
) -> Result<Role, diesel::result::Error> {
    use crate::schema::roles::dsl::*;
    roles
        .filter(slug.eq(role_slug))
        .select(Role::as_select())
        .first(conn)
        .await
}

pub async fn reads(conn: &mut AsyncPgConnection) -> Result<Vec<Role>, diesel::result::Error> {
    use crate::schema::roles::dsl::*;
    roles.select(Role::as_select()).load(conn).await
}

pub async fn update(
    conn: &mut AsyncPgConnection,
    role_slug: String,
    new_name: Option<String>,
    new_permissions: Option<String>,
) -> Result<Role, diesel::result::Error> {
    use crate::schema::roles::dsl::*;
    match new_name {
        None => (),
        Some(n) => {
            diesel::update(roles.find(&role_slug))
                .set(name.eq(n))
                .execute(conn)
                .await?;
        }
    };
    match new_permissions {
        None => (),
        Some(p) => {
            diesel::update(roles.find(&role_slug))
                .set(permissions.eq(p))
                .execute(conn)
                .await?;
        }
    };
    roles
        .filter(slug.eq(&role_slug))
        .select(Role::as_select())
        .first(conn)
        .await
}

pub async fn delete(
    conn: &mut AsyncPgConnection,
    role_slug: String,
) -> Result<(), diesel::result::Error> {
    use crate::schema::roles::dsl::*;
    diesel::delete(roles.filter(slug.eq(role_slug)))
        .execute(conn)
        .await?;
    Ok(())
}

pub async fn assign(
    conn: &mut AsyncPgConnection,
    slug: String,
    id: i32,
) -> Result<(), diesel::result::Error> {
    diesel::insert_into(crate::schema::users_roles::table)
        .values(&UserRole {
            role_slug: slug,
            user_id: id,
        })
        .execute(conn)
        .await?;

    Ok(())
}

pub async fn unassign(
    conn: &mut AsyncPgConnection,
    slug: String,
    id: i32,
) -> Result<(), diesel::result::Error> {
    use crate::schema::users_roles::dsl::*;
    diesel::delete(users_roles.filter(user_id.eq(id).and(role_slug.eq(slug))))
        .execute(conn)
        .await?;
    Ok(())
}
