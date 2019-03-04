#[macro_use]
extern crate lazy_static;

extern crate google_drive3;
extern crate hyper;
extern crate hyper_native_tls;
extern crate regex;
extern crate yup_oauth2;

use std::io::Read;
use std::path::Path;

use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;

use google_drive3::Drive;
use regex::Regex;
use yup_oauth2::{
    read_application_secret, Authenticator, DefaultAuthenticatorDelegate, DiskTokenStorage,
    FlowType,
};

const FINAL_HTML_STYLE: &'static str = "<style>body{padding:0;margin:0;}#article{margin:auto;}#navbar{width:100%;z-index:900;position:fixed;background-color:white;border-bottom:solid 2px grey;padding:5px;height:33px;text-align:center;font-family:Geneva,Tahoma,Verdana,sans-serif;}#navbar img:hover{cursor:pointer;border-radius:15px;background-color:#eeeeee;}#navbar #titulo{user-select:none;font-size:25px;}#navbar #hogar{float:left;margin-left:10px;}#navbar #engranaje{float:right;margin-right:10px;}</style>";
const FINAL_HTML_NAVBAR: &'static str = r#"<div id="navbar"><a href="index.html"><img id="hogar" src="hogar.svg" width="30px" height="30px"></a><img id="engranaje" src="engranaje.svg" width="30px" height="30px"><span id="titulo">Los Millenials Son Malos</span></div>"#;

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
            static ref ID_RE: Regex = Regex::new("[-\\w]{25,}").unwrap();
        }

        ID_RE
            .captures(google_url)
            .map_or(None, |c| c.get(0))
            .map(|m| m.as_str())
    }

    fn get_article(self: &Self, file_id: &str) -> Result<Articulo, Box<std::error::Error>> {


        let (_resp, metadata) = match self.hub.files().get(file_id).doit() {
            Ok(m) => m,
            Err(e) => return Err(Box::new(e)),
        };
        let mut file = match self.hub.files().export(file_id, "text/html").doit() {
            Ok(f) => f,
            Err(e) => return Err(Box::new(e)),
        };
        let mut article_html = String::new();
        file.read_to_string(&mut article_html)?;

        Ok(Articulo::new(article_html
            .replace("<body", "<body><div id=\"article\"")
            .replace("</body>", "</div></body>"), metadata))
    }
}

struct Articulo {
    html: String,
    metadata: google_drive3::File,
}

impl Articulo {
    fn new(html: String, metadata: google_drive3::File) -> Articulo {
        Articulo { html, metadata }
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

    fn generate_full_html(self: &Self) -> String {
        let mut add_navbar = "<body>".to_string();
        add_navbar.push_str(&FINAL_HTML_NAVBAR);
        let mut full_html = self.html
            .replace("<body>", add_navbar.as_str());
        full_html.push_str(&FINAL_HTML_STYLE);
        return full_html;
    }
}

fn main() {
}
