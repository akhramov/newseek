use std::str;
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{ Write, Read, BufReader };

use std::iter::FromIterator;

use dotenv::dotenv;

use handlebars::{
    to_json, Context, Handlebars, Helper, JsonRender, Output, RenderContext, RenderError,
};

use serde_json::value::{
    Map,
    Value
};

use repository::storage::*;
use errors::*;


use domain::{
    Settings,
};

use regex::Regex;
use base64;

macro_rules! picture_format_error {
    () => {
        ErrorKind::BadClientData("picture".to_string(),
                                 "is in invalid format".to_string())
    }
}

const ALLOWED_TYPES: [&'static str; 4] = ["jpeg", "jpe", "jpg", "png"];

pub fn update_settings(db: &Store, settings: &Settings) -> Result<Settings> {
    update_museek_settings(settings)?;
    update_config(settings)?;
    Ok(db.update_settings(settings)?)
}

fn update_config(settings: &Settings) -> Result<()> {
    dotenv().ok();

    let data_folder_path = env::var("DATA_FOLDER")
        .expect("DATA_FOLDER must be set");

    let template_path = Path::new(&data_folder_path)
        .join("config.template.xml");

    let config_path = Path::new(&data_folder_path)
        .join("config.xml");

    let mut handlebars = Handlebars::new();

    handlebars.register_template_file("config", template_path)
        .context(ErrorKind::ConfigTemplate)?;

    let mut file = File::create(config_path)
        .context(ErrorKind::FsError)?;

    let content = handlebars.render_to_write("config", settings, file)
        .context(ErrorKind::ConfigTemplate)?;

    Ok(())
}

fn update_museek_settings(settings: &Settings) -> Result<&Settings> {
    let re = Regex::new(r"data:image/(?P<type>\w+);base64,(?P<base64>.*)").unwrap();

    let picture = settings.picture.clone();

    let string = match str::from_utf8(picture.as_slice()) {
        Ok(string) => string,
        Err(_) => ""
    };

    if let Some(caps) = re.captures(string) {
        if ALLOWED_TYPES.into_iter().any(|&x| x == &caps["type"]) {
            persist_picture(&caps["base64"])?;
        } else {
            return Err(picture_format_error!().into());
        }
    }

    Ok(settings)
}

fn persist_picture(picture: &str) -> Result<()> {
    dotenv().ok();

    let data_folder_path = env::var("DATA_FOLDER")
        .expect("DATA_FOLDER must be set");

    let path = Path::new(&data_folder_path).join("config.image");

    let mut file = File::create(path).context(ErrorKind::FsError)?;

    let err = picture_format_error!();

    let mut content = base64::decode(picture).context(err)?;

    file.write_all(&content).context(ErrorKind::FsError)?;

    Ok(())
}
