use crate::config::get_config;

pub fn process_get() -> color_eyre::eyre::Result<()> {
    let config = get_config()?;

    println!("{:#?}", config);

    Ok(())
}
