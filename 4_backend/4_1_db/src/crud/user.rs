use crate::models::{NewUser, Role, User, UserRole};
use diesel::prelude::*;
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};

pub async fn create(
    conn: &mut AsyncPgConnection,
    name: String,
    bio: String,
    role_slug: String,
) -> Result<User, diesel::result::Error> {
    conn.transaction(|conn| {
        Box::pin(async {
            let new_user = NewUser { name, bio };
            let user = diesel::insert_into(crate::schema::users::table)
                .values(&new_user)
                .returning(User::as_returning())
                .get_result(conn)
                .await?;
            diesel::insert_into(crate::schema::users_roles::table)
                .values(&UserRole {
                    role_slug,
                    user_id: user.id,
                })
                .execute(conn)
                .await?;
            Ok(user)
        })
    })
    .await
}

pub async fn reads(conn: &mut AsyncPgConnection) -> Result<Vec<User>, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    users.select(User::as_select()).load(conn).await
}

pub async fn read(
    conn: &mut AsyncPgConnection,
    user_id: i32,
) -> Result<(User, Vec<Role>), diesel::result::Error> {
    let user = {
        use crate::schema::users::dsl::*;
        users
            .filter(id.eq(user_id))
            .select(User::as_select())
            .first(conn)
            .await
    }?;
    let roles = {
        use crate::schema::roles::dsl::*;
        use crate::schema::users_roles::dsl::*;
        users_roles
            .filter(user_id.eq(user.id))
            .inner_join(roles)
            .select(Role::as_select())
            .load(conn)
            .await
    }?;
    Ok((user, roles))
}

pub async fn update(
    conn: &mut AsyncPgConnection,
    user_id: i32,
    new_name: Option<String>,
    new_bio: Option<String>,
) -> Result<User, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    match new_name {
        None => (),
        Some(n) => {
            diesel::update(users.find(user_id))
                .set(name.eq(n))
                .execute(conn)
                .await?;
        }
    };
    match new_bio {
        None => (),
        Some(b) => {
            diesel::update(users.find(user_id))
                .set(bio.eq(b))
                .execute(conn)
                .await?;
        }
    };
    users
        .filter(id.eq(user_id))
        .select(User::as_select())
        .first(conn)
        .await
}

pub async fn delete(
    conn: &mut AsyncPgConnection,
    user_id: i32,
) -> Result<(), diesel::result::Error> {
    use crate::schema::users::dsl::*;
    diesel::delete(users.filter(id.eq(user_id)))
        .execute(conn)
        .await?;
    Ok(())
}
