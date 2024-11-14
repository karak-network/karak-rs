use color_eyre::owo_colors::OwoColorize;

use crate::config::models::{Curve, Profile};

use super::prompt;

pub async fn process_list(profile: Profile, curve: Option<Curve>) -> eyre::Result<()> {
    let curve = prompt::prompt_curve(curve);
    let keystores = profile.keystores;
    let curve_keystores = keystores.get(&curve);
    if curve_keystores.is_none() {
        return Err(eyre::eyre!("No keystores found for curve {}", curve));
    }

    println!("\nKeystores for curve {}:", curve.blue());
    for (name, keystore) in curve_keystores.unwrap() {
        println!("Name: {}", name.yellow());
        println!("{:#?}\n", keystore.blue());
    }

    Ok(())
}
