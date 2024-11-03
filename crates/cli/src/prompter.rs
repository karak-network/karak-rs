use std::str::FromStr;

use color_eyre::owo_colors::OwoColorize;
use dialoguer::{theme::ColorfulTheme, Input, MultiSelect, Password, Select};
use strum::VariantNames;

pub fn select<T: VariantNames + ToString>(prompt: &str, default: Option<T>) -> (usize, bool) {
    let theme = ColorfulTheme::default();
    let options = T::VARIANTS;
    let default_index = find_default_index(default.as_ref(), options);

    let selection_index = Select::with_theme(&theme)
        .with_prompt(prompt)
        .default(default_index)
        .items(options)
        .interact()
        .unwrap();

    (selection_index, selection_index == default_index)
}

#[derive(Clone)]
pub struct InputOptions {
    pub allow_empty: bool,
    pub initial_text: String,
}

pub fn input<T>(prompt: &str, default: Option<T>, opts: Option<InputOptions>) -> T
where
    T: Clone + ToString + FromStr,
    <T as FromStr>::Err: ToString,
{
    let theme = ColorfulTheme::default();
    loop {
        let mut input = Input::with_theme(&theme).with_prompt(prompt);
        if let Some(opts) = opts.clone() {
            input = input
                .with_initial_text(&opts.initial_text)
                .allow_empty(opts.allow_empty);
        }

        if let Some(default) = default.clone() {
            input = input.default(default.to_string());
        }

        let response = input.interact_text().unwrap();
        match response.parse::<T>() {
            Ok(value) => return value,
            Err(e) => println!("Invalid input - {:?}", e.to_string().red()),
        }
    }
}

pub fn password(prompt: &str) -> String {
    Password::new().with_prompt(prompt).interact().unwrap()
}

pub fn multi_select<T: ToString>(prompt: &str, items: &[T]) -> Vec<usize> {
    MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .items(items)
        .interact()
        .unwrap()
}

fn find_default_index<T: ToString>(default: Option<&T>, variants: &[&str]) -> usize {
    default
        .and_then(|d| {
            variants
                .iter()
                .position(|&variant| variant == d.to_string())
        })
        .unwrap_or(0)
}
