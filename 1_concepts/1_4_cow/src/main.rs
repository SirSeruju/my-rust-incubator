use std::env;

use std::borrow::Cow;

fn default_path() -> Cow<'static, str> {
    "/etc/app/app.conf".into()
}

fn replace_with_env_path(path: &mut Cow<'_, str>) {
    if let Ok(s) = env::var("APP_CONF") {
        if !s.is_empty() {
            *path.to_mut() = s
        }
    }
}

enum ArgsPathError {
    InvalidArgument(String),
    EmptyString,
}

fn replace_with_args_path(path: &mut Cow<'_, str>) -> Result<(), ArgsPathError> {
    let mut args = env::args();
    // executed
    args.next();

    match args.next() {
        Some(s) => {
            if s != "--conf" {
                return Err(ArgsPathError::InvalidArgument(s));
            }
        }
        None => return Ok(()),
    };
    match args.next() {
        Some(s) => {
            if s.is_empty() {
                Err(ArgsPathError::EmptyString)
            } else {
                *path.to_mut() = s;
                Ok(())
            }
        }
        None => Err(ArgsPathError::EmptyString),
    }
}

fn main() {
    let mut path = default_path();
    replace_with_env_path(&mut path);
    match replace_with_args_path(&mut path) {
        Ok(_) => println!("Your path: \"{}\"", path),
        Err(ArgsPathError::InvalidArgument(s)) => println!("Invalid argument {:?}, pass --conf", s),
        Err(ArgsPathError::EmptyString) => println!("--conf path must specified"),
    };
}
