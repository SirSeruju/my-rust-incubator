use diesel_async::AsyncConnection;
use diesel_async::AsyncPgConnection;
use tokio::runtime::Runtime;

use step_4_1::cli::{Cli, Command, RoleCommand, StructOpt, UserCommand};
use step_4_1::crud;

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
                UserCommand::List => println!("{:?}", crud::user::reads(conn).await),
                UserCommand::Create {
                    name,
                    bio,
                    role_slug,
                } => {
                    println!("{:?}", crud::user::create(conn, name, bio, role_slug).await)
                }
                UserCommand::Delete { user_id } => {
                    println!("{:?}", crud::user::delete(conn, user_id).await)
                }
                UserCommand::Update { user_id, name, bio } => {
                    println!("{:?}", crud::user::update(conn, user_id, name, bio).await)
                }
                UserCommand::Get { user_id } => {
                    println!("{:?}", crud::user::read(conn, user_id).await)
                }
            },
            Command::Role { cmd } => match cmd {
                RoleCommand::Create { name, permissions } => {
                    println!("{:?}", crud::role::create(conn, name, permissions).await)
                }
                RoleCommand::Delete { role_slug } => {
                    println!("{:?}", crud::role::delete(conn, role_slug).await)
                }
                RoleCommand::Update {
                    role_slug,
                    name,
                    permissions,
                } => println!(
                    "{:?}",
                    crud::role::update(conn, role_slug, name, permissions).await
                ),
                RoleCommand::Unassign { role_slug, user_id } => {
                    println!("{:?}", crud::role::unassign(conn, role_slug, user_id).await)
                }
                RoleCommand::Assign { role_slug, user_id } => {
                    println!("{:?}", crud::role::assign(conn, role_slug, user_id).await)
                }
                RoleCommand::List => println!("{:?}", crud::role::reads(conn).await),
                RoleCommand::Get { role_slug } => {
                    println!("{:?}", crud::role::read(conn, role_slug).await)
                }
            },
        }
    });
}
