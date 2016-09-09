//! Documentation: https://www.appveyor.com/docs/api/
//!

// - Get the last build of a project - /api/projects/{accountName}/{projectSlug}
// curl --silent --header 'Authorization: Bearer '$APPVEYOR --request GET \
//      https://ci.appveyor.com/api/projects/booyaa/hai
//
//
#![cfg_attr(feature = "serde_macros", feature(plugin, custom_derive))]
#![cfg_attr(feature = "serde_macros", plugin(serde_macros))]

extern crate serde;
extern crate serde_json;
extern crate hyper;
#[macro_use]
extern crate log;

use hyper::Client;
use hyper::header::{Authorization, ContentType, Bearer};
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use std::io::Read;

#[cfg(feature = "serde_macros")]
include!("serde_types.in.rs");

#[cfg(feature = "serde_codegen")]
include!(concat!(env!("OUT_DIR"), "/serde_types.rs"));

// const LIB_VERSION: &'static str = env!("CARGO_PKG_VERSION");
// const LIB_NAME: &'static str = env!("CARGO_PKG_NAME");


const BASE_URL: &'static str = "https://ci.appveyor.com/api/";
const BASE_PROJECTS: &'static str = "projects";
// const BASE_BUILDS: &'static str = "builds";

// errors
// use std::{self, fmt};

#[derive(Debug)]
pub enum Error {
    ///The response from Twitter gave a response code that indicated an error. The enclosed value
    ///was the response code.
    BadStatus(hyper::status::StatusCode),
    ///The web request experienced an error. The enclosed value was returned from hyper.
    NetError(hyper::error::Error),
    ///An error was experienced while processing the response stream. The enclosed value was
    ///returned from libstd.
    IOError(std::io::Error),
    // Failed to parse data as JSON
    ParseError(serde_json::error::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::BadStatus(ref val) => write!(f, "Error status received: {}", val),
            Error::NetError(ref err) => write!(f, "Network error: {}", err),
            Error::IOError(ref err) => write!(f, "IO error: {}", err),
            Error::ParseError(ref err) => write!(f, "Parsing error: {}", err),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::BadStatus(_) => "Response included error code",
            Error::NetError(ref err) => err.description(),
            Error::IOError(ref err) => err.description(),
            Error::ParseError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        match *self {
            Error::NetError(ref err) => Some(err),
            Error::IOError(ref err) => Some(err),
            Error::ParseError(ref err) => Some(err),
            _ => None,
        }
    }
}

impl From<hyper::error::Error> for Error {
    fn from(err: hyper::error::Error) -> Error {
        Error::NetError(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IOError(err)
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(err: serde_json::error::Error) -> Error {
        Error::ParseError(err)
    }
}

/// Main struct
#[allow(dead_code)]
#[derive(Debug,PartialEq)]
pub struct AppVeyor {
    token: String,
    test_mode: bool,
}

impl AppVeyor {
    pub fn new(token: &str) -> AppVeyor {
        AppVeyor {
            token: token.to_string(),
            test_mode: false,
        }
    }

    pub fn enable_test_mode(&mut self) {
        self.test_mode = true;
    }

    /// Get a list of projects - /api/projects
    ///
    /// curl --silent --header 'Authorization: Bearer '$APPVEYOR --request GET \
    ///         https://ci.appveyor.com/api/projects
    ///
    /// GET /api/projects
    pub fn get_projects(&self) -> Vec<Project> {
        // are we dev mode
        // yes - read local file
        // no - http
        // parse
        // return Vec<Project


        println!("self: {:#?}", &self);
        if self.test_mode == true {
            let result = load_file("get_projects.json");
            serde_json::from_str(&result).unwrap()
        } else {
            let client = Client::new();
            let url = format!("{}{}", BASE_URL, BASE_PROJECTS);
            debug!("url: {}", url);
            let mut res = client.get(&url)
                                .header(Authorization({
                                    Bearer { token: self.token.to_owned() }
                                }))
                                .send()
                                .unwrap();

            if res.status != hyper::status::StatusCode::Ok {
                panic!(res.status.to_string());
            }

            let mut buffer = String::new();
            res.read_to_string(&mut buffer).expect("no body");
            debug!("buffer: {}", buffer);
            serde_json::from_str(&buffer).unwrap()
        }
    }


    /// Add a project - /api/projects
    ///
    /// curl --silent --header 'Authorization: Bearer '$APPVEYOR \
    ///      --header 'Content-Type: application/json' \
    ///      --request POST https://ci.appveyor.com/api/projects \
    ///      -d '{"repositoryProvider" : "gitHub", "repositoryName" : "booyaa/hello-homu"}'
    ///
    /// POST /api/projects
    pub fn add_project(&self) -> String {
        let client = Client::new();
        let url = format!("{}{}", BASE_URL, BASE_PROJECTS);
        debug!("url: {}", url);

        let mut res = client.post(&url)
                            .header(ContentType(Mime(TopLevel::Application,
                                                     SubLevel::Json,
                                                     vec![(Attr::Charset, Value::Utf8)])))
                            .body(r#"{"foo": "bar", "fizz": 123}"#)
                            .send()
                            .unwrap();

        if res.status != hyper::status::StatusCode::Ok {
            panic!(res.status.to_string());
        }

        let mut buffer = String::new();
        res.read_to_string(&mut buffer).expect("no body");
        debug!("buffer: {}", buffer);

        buffer
    }


    /// Cancel a build - /api/builds/{accountName}/{projectSlug}/{buildVersion}
    ///
    /// curl --silent --header 'Authorization: Bearer '$APPVEYOR --request DELETE \
    ///      https://ci.appveyor.com/api/builds/booyaa/hai/1.0.11
    ///
    /// error handling
    /// - build has already finished - will give you HTTP 500 and {"message":"The build has already finished."}
    ///
    /// DELETE /api/builds/{accountName}/{projectSlug}/{buildVersion}
    pub fn cancel_build(&self) -> String {
        let client = Client::new();
        // let url = format!("{}{}", BASE_URL, "/delete");
        let url = format!("{}{}", BASE_URL, "/post");
        debug!("url: {}", url);

        let mut res = client.delete(&url).send().unwrap();

        if res.status != hyper::status::StatusCode::Ok {
            panic!(res.status.to_string());
        }

        println!("cancel_build response status code: {}", res.status);

        let mut buffer = String::new();
        res.read_to_string(&mut buffer).expect("no body");
        println!("buffer: {}", buffer);

        buffer
    }
}

fn load_file(file: &str) -> String {
    println!("load_file: {}", &file);

    use std::path::PathBuf;
    let mut path = PathBuf::new();
    path.push("tests");
    path.push("fixtures");
    path.push(&file);

    let file_path = path.as_os_str();

    use std::fs::File;
    use std::io::Read;

    let mut file = File::open(&file_path).unwrap();

    let mut file_string = String::new();
    file.read_to_string(&mut file_string).unwrap();

    file_string
}

// maybe this? fn call_appveyor(url, parameters : Some<params>) -> String
// TODO: fn call_appveyor() -> String {
//     let mut res = client.get(&url).send().unwrap();
//
//     if res.status != hyper::status::StatusCode::Ok {
//         panic!(res.status.to_string());
//     }
//
//     let mut buffer = String::new();
//     res.read_to_string(&mut buffer).expect("no body");
//     debug!("buffer: {}", buffer);
//
//     buffer
// }


#[test]
// #[ignore]
fn should_return_project_list() {
    let mut happv = AppVeyor::new("adsasd");
    happv.enable_test_mode();

    let result = happv.get_projects();

    assert_eq!(3, result.len());
    for i in result.into_iter() {
        println!("{}", i.slug);
    }
}

#[test]
// #[ignore]
fn integration_should_return_project_list() {
    let happv = AppVeyor::new(env!("APPVEYOR"));

    let result = happv.get_projects();

    assert!(0 < result.len());
    for i in result.into_iter() {
        println!("{}", i.slug);
    }
}

#[allow(dead_code)]
fn connection_refused() -> Result<String, Error> {
    let client = Client::new();
    let url = format!("{}{}", "http://localhost:32769", "/headers");
    debug!("url: {}", url);
    let res = try!(client.get(&url)
                         .header(Authorization({
                             Bearer { token: "XXX".to_owned() }
                         }))
                         .send());

    if res.status != hyper::status::StatusCode::Ok {
        return Err(Error::BadStatus(res.status));
        // panic!(res.status.to_string());
    }
    Ok("foo".to_string())
}

#[test]
#[ignore]
fn should_be_connection_refused() {
    let result = connection_refused();

    println!("Error: {:#?}", result);
    assert!(result.is_err());

}

#[allow(dead_code)]
fn bad_status() -> Result<String, Error> {
    let client = Client::new();
    let url = format!("{}{}", "http://localhost:32768", "/status/404");
    debug!("url: {}", url);
    let res = try!(client.get(&url)
                         .header(Authorization({
                             Bearer { token: "XXX".to_owned() }
                         }))
                         .send());

    if res.status != hyper::status::StatusCode::Ok {
        return Err(Error::BadStatus(res.status));
    }
    // let mut buffer = String::new();
    // res.read_to_string(&mut buffer).expect("no body");
    // debug!("buffer: {}", buffer);
    // Ok(serde_json::from_str(&buffer).unwrap())
    Ok("foo".to_string())
}

#[test]
#[ignore]
fn should_be_bad_status() {
    let result = bad_status();

    println!("Error: {:#?}", result);
    assert!(result.is_err());

}

#[allow(dead_code)]
fn failed_to_decode() -> Result<Project, Error> {
    let client = Client::new();
    let url = format!("{}{}", "http://localhost:32768", "/headers");
    debug!("url: {}", url);
    let mut res = try!(client.get(&url)
                             .header(Authorization({
                                 Bearer { token: "XXX".to_owned() }
                             }))
                             .send());

    if res.status != hyper::status::StatusCode::Ok {
        return Err(Error::BadStatus(res.status));
    }

    let mut buffer = String::new();
    try!(res.read_to_string(&mut buffer));
    println!("buffer: {}", buffer);

    let result = try!(serde_json::from_str(&buffer));
    Ok(result)
}

#[test]
#[ignore]
fn should_be_failed_to_decode() {
    let result = failed_to_decode();

    println!("Error: {:#?}", result);
    assert!(result.is_err());

}
