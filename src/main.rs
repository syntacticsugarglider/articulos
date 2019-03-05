#[macro_use]
extern crate lazy_static;

extern crate askama;
extern crate google_drive3;
extern crate hyper;
extern crate hyper_native_tls;
extern crate regex;
extern crate yup_oauth2;

use std::fs;
use std::io::Read;
use std::path::Path;

use google_drive3::Drive;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use yup_oauth2::{
    read_application_secret, Authenticator, DefaultAuthenticatorDelegate, DiskTokenStorage,
    FlowType,
};

use askama::Template;
use regex::Regex;

enum ImagePlacement {
    Default,
    FloatLeft(u8),
    FloatRight(u8),
}

struct ArticuloLlevador {
    hub: Drive<
        hyper::Client,
        Authenticator<DefaultAuthenticatorDelegate, DiskTokenStorage, hyper::Client>,
    >,
}

impl ArticuloLlevador {
    fn new(client_secret: &str, token_cache: &str) -> std::io::Result<ArticuloLlevador> {
        let secret = read_application_secret(Path::new(&client_secret.to_string()))?;
        let client =
            hyper::Client::with_connector(HttpsConnector::new(NativeTlsClient::new().unwrap()));
        let authenticator = Authenticator::new(
            &secret,
            DefaultAuthenticatorDelegate,
            client,
            DiskTokenStorage::new(&token_cache.to_string())?,
            Some(FlowType::InstalledInteractive),
        );
        let client =
            hyper::Client::with_connector(HttpsConnector::new(NativeTlsClient::new().unwrap()));
        Ok(ArticuloLlevador {
            hub: Drive::new(client, authenticator),
        })
    }

    fn file_id_from_url(google_url: &str) -> Option<&str> {
        lazy_static! {
            static ref ID_RE: Regex = Regex::new(r"[-\w]{25,}").unwrap();
        }

        ID_RE
            .captures(google_url)
            .map_or(None, |c| c.get(0))
            .map(|m| m.as_str())
    }

    fn get_article(self: &Self, file_id: &str) -> Result<Articulo, Box<std::error::Error>> {
        lazy_static! {
            static ref ARTICLE_RE: Regex = Regex::new(r#"<body(?P<html>(.*))</body>"#).unwrap();
        }

        let (_resp, metadata) = match self.hub.files().get(file_id).doit() {
            Ok(m) => m,
            Err(e) => return Err(Box::new(e)),
        };
        let mut file = match self.hub.files().export(file_id, "text/html").doit() {
            Ok(f) => f,
            Err(e) => return Err(Box::new(e)),
        };
        let mut article = String::new();
        file.read_to_string(&mut article)?;

        let caps = &ARTICLE_RE.captures(article.as_str()).expect("hello");
        let mut html = String::from(r#"<div id="article""#);
        html.push_str(
            caps.name("html")
                .expect("Google export returned bad html")
                .as_str(),
        );
        html.push_str("</div>");

        Ok(Articulo::new(html, metadata))
    }
}

#[derive(Template)]
#[template(path = "articulo.html")]
struct Articulo {
    html: String,
    metadata: google_drive3::File,
    url: String,
}

impl Articulo {
    fn new(html: String, metadata: google_drive3::File) -> Articulo {
        Articulo { html, metadata, url: String::from("index.html") }
    }

    fn set_image_styles(self: &mut Self, image_styles: Vec<ImagePlacement>) {
        lazy_static! {
            static ref IMG_STYLE_RE: Regex = Regex::new("display: inline-block;").unwrap(); // TODO: get real regex
        }
        for style in image_styles {
            self.html = self.html.replacen(
                &IMG_STYLE_RE.to_owned(),
                match style {
                    ImagePlacement::Default => continue,
                    ImagePlacement::FloatLeft(p) => {
                        format!("float: left; padding-right: {:?}px;", p)
                    }
                    ImagePlacement::FloatRight(p) => {
                        format!("float: right; padding-left: {:?}px;", p)
                    }
                }
                .as_str(),
                1,
            );
        }
    }
}

#[derive(Template)]
#[template(path = "index.html")]
struct Articulos {
    articles: Vec<Articulo>,
}

fn main() {
}
