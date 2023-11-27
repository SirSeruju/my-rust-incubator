use step_4_3::cli::{Cli, Command, RoleCommand, StructOpt, UserCommand};
use step_4_3::endpoints::role::{NewRole, UpdateRole};
use step_4_3::endpoints::user::{NewUser, UpdateUser};

#[allow(clippy::single_component_path_imports)]
use reqwest;

#[tokio::main]
async fn main() {
    let opt = Cli::from_args();
    let server_url = opt.server_url;

    let client = reqwest::Client::new();

    // TODO: do something with result displaying
    let response = match opt.cmd {
        Command::User { cmd } => match cmd {
            UserCommand::List => client.get(server_url + "/users"),
            UserCommand::Create {
                name,
                bio,
                role_slug,
            } => {
                let new_user = NewUser {
                    name,
                    bio,
                    role_slug,
                };
                client.post(server_url + "/users").json(&new_user)
            }
            UserCommand::Delete { user_id } => {
                client.delete(server_url + "/users/" + &user_id.to_string())
            }
            UserCommand::Update { user_id, name, bio } => {
                let user_updates = UpdateUser {
                    new_name: name,
                    new_bio: bio,
                };
                client
                    .put(server_url + "/users/" + &user_id.to_string())
                    .json(&user_updates)
            }
            UserCommand::Get { user_id } => {
                client.get(server_url + "/users/" + &user_id.to_string())
            }
        },
        Command::Role { cmd } => match cmd {
            RoleCommand::Create { name, permissions } => {
                let new_role = NewRole { name, permissions };
                client.post(server_url + "/roles").json(&new_role)
            }
            RoleCommand::Delete { role_slug } => client.delete(server_url + "/roles/" + &role_slug),

            RoleCommand::Update {
                role_slug,
                name,
                permissions,
            } => {
                let role_updates = UpdateRole {
                    new_name: name,
                    new_permissions: permissions,
                };
                client
                    .put(server_url + "/roles/" + &role_slug)
                    .json(&role_updates)
            }

            RoleCommand::Unassign { role_slug, user_id } => {
                client.delete(server_url + "/assign/" + &role_slug + "/" + &user_id.to_string())
            }
            RoleCommand::Assign { role_slug, user_id } => {
                client.post(server_url + "/assign/" + &role_slug + "/" + &user_id.to_string())
            }

            RoleCommand::List => client.get(server_url + "/roles"),
            RoleCommand::Get { role_slug } => client.get(server_url + "/roles/" + &role_slug),
        },
    }
    .send()
    .await
    .expect("failed to get server response");
    println!("Response code: {}", response.status());
    let body = response.text().await.expect("failed to get response body");
    let body_message = match serde_json::from_str::<serde_json::Value>(&body) {
        Ok(v) => serde_json::to_string_pretty(&v).unwrap(),
        Err(_) => body,
    };
    println!("Body:\n{}", body_message);
}
