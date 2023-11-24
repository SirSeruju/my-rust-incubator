mod cli;
mod settings;

use settings::Settings;

fn main() {
    let s = Settings::new().expect("failed to get settings with error:");
    println!("Your settings:\n{:?}", s);
}
