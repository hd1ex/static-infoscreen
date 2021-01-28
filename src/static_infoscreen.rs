use lazy_static::lazy_static;
use mime_guess::{self, mime};
use regex::Regex;
use std::fs::{self, File};
use std::{io::Write, path::Path};
use tera::{Context, Tera};
use String;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/**/*.html") {
            Ok(t) => t,
            Err(e) => {
                eprintln!("Template parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![]);
        tera
    };
}

#[derive(Debug)]
pub struct Content {
    display_time: f64,
    link: String,
}

#[derive(Debug)]
pub struct Config {
    content: Vec<Content>,
}

impl Content {
    const WEBSITE_TEMPLATE: &'static str = "website.html";
    const VIDEO_TEMPLATE: &'static str = "video.html";
    const IMAGE_TEMPLATE: &'static str = "image.html";
    const DEFAULT_TEMPLATE: &'static str = Content::WEBSITE_TEMPLATE;

    fn get_template(&self) -> &'static str {
        let guess = mime_guess::from_path(&self.link).first();

        if guess == None {
            return Content::DEFAULT_TEMPLATE;
        }

        let guess = guess.unwrap();

        if guess == mime::TEXT_HTML {
            return Content::WEBSITE_TEMPLATE;
        }

        if guess.type_() == "video" {
            return Content::VIDEO_TEMPLATE;
        }

        match guess.type_().as_str() {
            "video" => Content::VIDEO_TEMPLATE,
            "image" => Content::IMAGE_TEMPLATE,
            _ => Content::DEFAULT_TEMPLATE,
        }
    }

    pub fn create_html(&self, folder: &Path, num: usize, next_content: &Content, next_num: usize) {
        if !self.link.contains("://") && !Path::new(&self.link).exists() {
            eprintln!(
                "Warning: File '{}.html' probably links local file which does not exist: '{}'",
                num, self.link
            );
        }

        let template_name = self.get_template();
        let mut context = Context::new();

        context.insert("current_content", &self.link);
        context.insert("display_time", &(self.display_time * 1000.0));
        context.insert("next_url", &format!("{}.html", next_num));
        context.insert("next_file", &next_content.link);

        let html = match TEMPLATES.render(template_name, &context) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("{}", e);
                return;
            }
        };

        let mut file = match File::create(folder.join(format!("{}.html", num))) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("Can not create file.\n{}", e);
                return;
            }
        };
        match file.write_all(html.as_bytes()) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Can not write file.\n{}", e);
                return;
            }
        };
    }
}

impl Config {
    pub fn parse_from(config_filename: &str) -> Config {
        let mut content = Vec::new();

        let contents = fs::read_to_string(config_filename)
            .expect(&format!("Can not read config file: {}", config_filename));

        lazy_static! {
            static ref RE: Regex = Regex::new(r"^((?:[0-9]*[.])?[0-9]+) (.+)$").unwrap();
        };

        let mut line_num = 0;
        for line in contents.lines() {
            line_num += 1;

            let line = line.trim();

            if line.starts_with("#") || line.is_empty() {
                continue; // Skip comments and empty lines
            }

            let caps = RE.captures(line).unwrap();

            let display_time = match caps.get(1) {
                Some(m) => m,
                None => {
                    eprintln!(
                        "Error in config file {}:{}, can not parse display time. Skipping entry...",
                        config_filename, line_num
                    );
                    continue;
                }
            }
            .as_str();

            let display_time = match display_time.parse() {
                Ok(d) => d,
                Err(_) => {
                    eprintln!(
                        "Error in config file {}:{}, display time is not a number. Skipping entry...",
                        config_filename, line_num
                    );
                    continue;
                }
            };

            let link = match caps.get(2) {
                Some(m) => m,
                None => {
                    eprintln!(
                        "Error in config file {}:{}, no resource link specified. Skipping entry...",
                        config_filename, line_num
                    );
                    continue;
                }
            }
            .as_str();

            content.push(Content {
                display_time,
                link: String::from(link),
            });
        }

        Config { content }
    }

    pub fn create_html(&self, folder: &str) {
        let folder = Path::new(folder);

        for (index, content) in self.content.iter().enumerate() {
            let next_index = if index + 1 < self.content.len() {
                index + 1
            } else {
                0
            };
            content.create_html(
                folder,
                index,
                self.content.get(next_index).unwrap(),
                next_index,
            );
        }
    }
}
