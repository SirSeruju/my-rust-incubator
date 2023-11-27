pub use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "step_4_1", about = "Simple Cli allows CRUD data.")]
pub struct Cli {
    /// URL for postgres connection
    #[structopt(long, short, env = "DATABASE_URL")]
    pub database_url: String,
    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    User {
        #[structopt(subcommand)]
        cmd: UserCommand,
    },
    Role {
        #[structopt(subcommand)]
        cmd: RoleCommand,
    },
}

#[derive(Debug, StructOpt)]
pub enum UserCommand {
    Create {
        #[structopt(long, short)]
        name: String,
        #[structopt(long, short)]
        role_slug: String,
        #[structopt(long, short)]
        bio: String,
    },
    Delete {
        #[structopt(long, short)]
        user_id: i32,
    },
    Update {
        #[structopt(long, short)]
        user_id: i32,
        #[structopt(long, short)]
        name: Option<String>,
        #[structopt(long, short)]
        bio: Option<String>,
    },
    List,
    Get {
        #[structopt(long, short)]
        user_id: i32,
    },
}

#[derive(Debug, StructOpt)]
pub enum RoleCommand {
    Create {
        #[structopt(long, short)]
        name: String,
        #[structopt(long, short)]
        permissions: String,
    },
    Delete {
        #[structopt(long, short)]
        role_slug: String,
    },
    Update {
        #[structopt(long, short)]
        role_slug: String,
        #[structopt(long, short)]
        name: Option<String>,
        #[structopt(long, short)]
        permissions: Option<String>,
    },
    Unassign {
        #[structopt(long, short)]
        role_slug: String,
        #[structopt(long, short)]
        user_id: i32,
    },
    Assign {
        #[structopt(long, short)]
        role_slug: String,
        #[structopt(long, short)]
        user_id: i32,
    },
    List,
    Get {
        #[structopt(long, short)]
        role_slug: String,
    },
}
