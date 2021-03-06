use serde::de::{Deserializer, Error, Unexpected};
use serde::Deserialize;
use yansi::Color;

#[derive(Clone, Debug, Deserialize)]
#[serde(default)]
pub struct Config {
    #[serde(deserialize_with = "deserialize_color")]
    pub path_color: Color,
    pub git_branch_icon: String,
    #[serde(deserialize_with = "deserialize_color")]
    pub git_branch_color: Color,
    #[serde(deserialize_with = "deserialize_bool")]
    pub git_branch_disable: bool,
    pub git_status_staged_icon: String,
    pub git_status_unstaged_icon: String,
    pub git_status_untracked_icon: String,
    pub git_status_ahead_icon: String,
    pub git_status_behind_icon: String,
    #[serde(deserialize_with = "deserialize_color")]
    pub git_status_color: Color,
    #[serde(deserialize_with = "deserialize_bool")]
    pub git_status_disable: bool,
    pub char_user_icon: String,
    #[serde(deserialize_with = "deserialize_color")]
    pub char_user_color: Color,
    pub char_user_failed_icon: String,
    #[serde(deserialize_with = "deserialize_color")]
    pub char_user_failed_color: Color,
    pub char_root_icon: String,
    #[serde(deserialize_with = "deserialize_color")]
    pub char_root_color: Color,
    pub char_root_failed_icon: String,
    #[serde(deserialize_with = "deserialize_color")]
    pub char_root_failed_color: Color,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            path_color: Color::Blue,
            git_branch_icon: "".to_owned(),
            git_branch_color: Color::Cyan,
            git_branch_disable: false,
            git_status_staged_icon: "+".to_owned(),
            git_status_unstaged_icon: "!".to_owned(),
            git_status_untracked_icon: "?".to_owned(),
            git_status_ahead_icon: "↥".to_owned(),
            git_status_behind_icon: "↧".to_owned(),
            git_status_color: Color::Cyan,
            git_status_disable: false,
            char_user_icon: "$".to_owned(),
            char_user_color: Color::Green,
            char_user_failed_icon: "$".to_owned(),
            char_user_failed_color: Color::Red,
            char_root_icon: "#".to_owned(),
            char_root_color: Color::Green,
            char_root_failed_icon: "#".to_owned(),
            char_root_failed_color: Color::Red,
        }
    }
}

fn deserialize_bool<'de, D: Deserializer<'de>>(deserializer: D) -> Result<bool, D::Error> {
    let string = String::deserialize(deserializer)?;
    match string.as_ref() {
        "0" | "false" => Ok(false),
        "1" | "true" => Ok(true),
        other => Err(D::Error::invalid_value(
            Unexpected::Str(other),
            &"either `0`, `false`, `1`, or `true`",
        )),
    }
}

fn deserialize_color<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Color, D::Error> {
    let string = String::deserialize(deserializer)?;
    match string.to_lowercase().as_ref() {
        "default" => Ok(Color::Default),
        "black" => Ok(Color::Black),
        "red" => Ok(Color::Red),
        "green" => Ok(Color::Green),
        "yellow" => Ok(Color::Yellow),
        "blue" => Ok(Color::Blue),
        "magenta" => Ok(Color::Magenta),
        "cyan" => Ok(Color::Cyan),
        "white" => Ok(Color::White),
        _ => {
            if string.starts_with('#') {
                match string.len() {
                    4 => {
                        let r = u8::from_str_radix(&string[1..2], 16).map_err(D::Error::custom)?;
                        let g = u8::from_str_radix(&string[2..3], 16).map_err(D::Error::custom)?;
                        let b = u8::from_str_radix(&string[3..4], 16).map_err(D::Error::custom)?;
                        Ok(Color::RGB(r * 16 + r, g * 16 + g, b * 16 + b))
                    }
                    7 => {
                        let r = u8::from_str_radix(&string[1..3], 16).map_err(D::Error::custom)?;
                        let g = u8::from_str_radix(&string[3..5], 16).map_err(D::Error::custom)?;
                        let b = u8::from_str_radix(&string[5..7], 16).map_err(D::Error::custom)?;
                        Ok(Color::RGB(r, g, b))
                    }
                    _ => Err(D::Error::invalid_value(
                        Unexpected::Str(&string),
                        &"a string with 4 or 7 characters including the `#`",
                    )),
                }
            } else if string.chars().all(|c| c.is_numeric()) {
                Ok(Color::Fixed(string.parse().map_err(D::Error::custom)?))
            } else {
                let expected = format!(
                    "a hex color beginning with `#`, a ANSII number, an empty string, or one of {}",
                    "`default`, black`, `red`, `green`, `yellow`, `blue`, `magenta`, `cyan`, `white`"
                );
                Err(D::Error::invalid_value(
                    Unexpected::Str(&string),
                    &expected.as_ref(),
                ))
            }
        }
    }
}
