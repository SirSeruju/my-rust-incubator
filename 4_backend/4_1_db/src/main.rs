use diesel_async::AsyncConnection;
use diesel_async::AsyncPgConnection;
use tokio::runtime::Runtime;

use step_4_1::cli::{Cli, Command, RoleCommand, StructOpt, UserCommand};
use step_4_1::crud;

use serde::Serialize;
use serde_yaml;
use std::fmt;

fn print_result<O: Serialize, E: fmt::Display>(r: Result<O, E>) {
    match r {
        Ok(o) => println!("Result:\n{}", serde_yaml::to_string(&o).unwrap()),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let opt = Cli::from_args();

    let rt = Runtime::new().unwrap();

    rt.block_on(async {
        let conn = &mut AsyncPgConnection::establish(&opt.database_url)
            .await
            .expect("failed to connect to PostgreSQL");

        // TODO: do something with result displaying
        match opt.cmd {
            Command::User { cmd } => match cmd {
                UserCommand::List => print_result(crud::user::reads(conn).await),
                UserCommand::Create {
                    name,
                    bio,
                    role_slug,
                } => print_result(crud::user::create(conn, name, bio, role_slug).await),
                UserCommand::Delete { user_id } => {
                    print_result(crud::user::delete(conn, user_id).await)
                }
                UserCommand::Update { user_id, name, bio } => {
                    print_result(crud::user::update(conn, user_id, name, bio).await)
                }
                UserCommand::Get { user_id } => print_result(crud::user::read(conn, user_id).await),
            },
            Command::Role { cmd } => match cmd {
                RoleCommand::Create { name, permissions } => {
                    print_result(crud::role::create(conn, name, permissions).await)
                }
                RoleCommand::Delete { role_slug } => {
                    print_result(crud::role::delete(conn, role_slug).await)
                }
                RoleCommand::Update {
                    role_slug,
                    name,
                    permissions,
                } => print_result(crud::role::update(conn, role_slug, name, permissions).await),
                RoleCommand::Unassign { role_slug, user_id } => {
                    print_result(crud::role::unassign(conn, role_slug, user_id).await)
                }
                RoleCommand::Assign { role_slug, user_id } => {
                    print_result(crud::role::assign(conn, role_slug, user_id).await)
                }
                RoleCommand::List => print_result(crud::role::reads(conn).await),
                RoleCommand::Get { role_slug } => {
                    print_result(crud::role::read(conn, role_slug).await)
                }
            },
        }
    });
}
