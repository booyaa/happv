//! A minimal AppVeyor API library in Rust.
//!
//! Warning: this is a partial implemented API of the AppVeyor REST [API][1]. I only needed the
//! following end points:
//!
//! - Get a list of projects - /api/projects
//! - Get the last build of a project - /api/projects/{accountName}/{projectSlug}
//! - Add a project - /api/projects
//! - Cancel a build - /api/builds/{accountName}/{projectSlug}/{buildVersion}
//!
//! [PRs][2] welcome if you want to implement other endpoints and/or the Build Worker API.

//! Documentation: https://www.appveyor.com/docs/api/
//!
//! https://gist.github.com/fredhsu/ae4088f905ac25bfcc04
//! - Get the last build of a project - /api/projects/{accountName}/{projectSlug}
//! curl --silent --header 'Authorization: Bearer '$APPVEYOR --request GET \
//!      https://ci.appveyor.com/api/projects/booyaa/hai
//!
//! [1]: https://www.appveyor.com/docs/api/
//! [2]: https://github.com/booyaa/happv/issues/new
#![cfg_attr(feature = "serde_macros", feature(plugin, custom_derive))]
#![cfg_attr(feature = "serde_macros", plugin(serde_macros))]

extern crate serde;
extern crate serde_json;
extern crate hyper;
#[macro_use]
extern crate log;

pub mod error;

use hyper::Client;
use hyper::header::{Authorization, ContentType, Bearer};
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use std::io::Read;
use error::Error;

#[cfg(feature = "serde_macros")]
include!("serde_types.in.rs");

#[cfg(feature = "serde_codegen")]
include!(concat!(env!("OUT_DIR"), "/serde_types.rs"));

const BASE_URL: &'static str = "https://ci.appveyor.com/api/";
const BASE_PROJECTS: &'static str = "projects";
#[allow(dead_code)]
const BASE_BUILDS: &'static str = "builds";


/// Main struct
#[allow(dead_code)]
#[derive(Debug,PartialEq)]
pub struct AppVeyor {
    token: String,
    test_mode: bool,
}

impl AppVeyor {
    /// Creates a new AppVeyor session using the API token provided
    pub fn new(token: &str) -> AppVeyor {
        AppVeyor {
            token: token.to_string(),
            test_mode: false,
        }
    }

    /// Toggles test mode, primarily used for writing tests
    pub fn enable_test_mode(&mut self) {
        self.test_mode = true;
    }

    /// Returns a list of Projects
    /// GET /api/projects
    pub fn get_projects(&self) -> Result<Vec<Project>, Error> {
        let mut buffer: String;

        if self.test_mode == true {
            buffer = load_file("get_projects.json");
        } else {
            let client = Client::new();
            let url = format!("{}{}", BASE_URL, BASE_PROJECTS);
            debug!("url: {}", url);
            let mut res = try!(client.get(&url)
                                     .header(Authorization({
                                         Bearer { token: self.token.to_owned() }
                                     }))
                                     .send());

            if res.status != hyper::status::StatusCode::Ok {
                return Err(Error::BadStatus(res.status));
            }

            buffer = String::new();
            try!(res.read_to_string(&mut buffer));
            debug!("buffer: {}", buffer);
        }

        let result = try!(serde_json::from_str(&buffer));
        Ok(result)
    }

    /// Adds a project to AppVeyor
    /// Warning: doesn't check if the repo's already been added
    ///
    /// POST /api/projects
    /// Error: 500! {"message":"Error reading repository details: HTTP GET to https://api.github.com/repos/booyaa/hello-homx returned 404: Not Found"}
    pub fn add_project(&self,
                       repository_provider: String,
                       repository_name: String)
                       -> Result<Project, Error> {
        let client = Client::new();
        let url = format!("{}{}", BASE_URL, BASE_PROJECTS);
        debug!("url: {}", url);

        let project = AddProject {
            repository_provider: repository_provider,
            repository_name: repository_name,
        };

        let body_json = try!(serde_json::to_string(&project));
        debug!("body_json: {:?}", body_json);
        let mut res = try!(client.post(&url)
                                 .header(Authorization({
                                     Bearer { token: self.token.to_owned() }
                                 }))
                                 .header(ContentType(Mime(TopLevel::Application,
                                                          SubLevel::Json,
                                                          vec![(Attr::Charset, Value::Utf8)])))
                                 .body(&body_json)
                                 .send());

        if res.status != hyper::status::StatusCode::Ok {
            return Err(Error::BadStatus(res.status)); // FIXME: will return early, so we lose the error message from AppVeyor
        }

        let mut buffer = String::new();
        try!(res.read_to_string(&mut buffer));
        debug!("buffer: {}", buffer);

        let result = try!(serde_json::from_str(&buffer));
        Ok(result)
    }

    /// Deletes a project
    /// DELETE /api/builds/{accountName}/{projectSlug}
    pub fn delete_project(&self, account_name: String, project_slug: String) -> Result<(), Error> {
        let client = Client::new();
        let url = format!("{}{}/{}/{}",
                          BASE_URL,
                          BASE_PROJECTS,
                          account_name,
                          project_slug);
        println!("url: {}", url);

        let res = try!(client.delete(&url)
                             .header(Authorization({
                                 Bearer { token: self.token.to_owned() }
                             }))
                             .send());

        println!("status: {:?}", res.status);
        if res.status != hyper::status::StatusCode::NoContent {
            return Err(Error::BadStatus(res.status));
        }

        Ok(())
    }

    /// DELETE /api/builds/{accountName}/{projectSlug}/{buildVersion}
    ///
    /// error handling
    /// - build has already finished - will give you HTTP 500 and {"message":"The build has already finished."}
    ///
    /// Incomplete
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
fn should_return_project_list_in_testmode() {
    let mut happv = AppVeyor::new("adsasd");
    happv.enable_test_mode();

    let result = happv.get_projects().unwrap();

    assert_eq!(3, result.len());
    println!("enumerate list:");
    for i in result.into_iter() {
        println!("\trepo: {}", i.slug);
    }
}

#[test]
#[ignore] // integration test
fn integration_should_return_project_list() {
    let happv = AppVeyor::new(env!("APPVEYOR"));

    let result = happv.get_projects().unwrap();

    // assert(0 < result.len());
    println!("enumerate list:");
    for i in result.into_iter() {
        println!("\tId:{} Slug:{}", i.project_id, i.slug);
    }
}

#[test]
#[ignore] // integration test
fn should_add_project() {
    let happv = AppVeyor::new(env!("APPVEYOR"));
    let result = happv.add_project("gitHub".to_string(), "booyaa/hello-homu".to_string());
    assert!(result.is_ok());
    println!("{:#?}", result.unwrap());
}

#[test]
#[ignore] // integration test
fn should_fail_to_add_project() {
    let happv = AppVeyor::new(env!("APPVEYOR"));
    let result = happv.add_project("gitHub".to_string(), "booyaa/i_dont_exist".to_string());
    assert!(result.is_err());
    println!("{:#?}", result.err()); // returns BadStatus::InternalServerError would prefer the message
}

#[test]
#[ignore] // integration test
fn should_delete_project() {
    let happv = AppVeyor::new(env!("APPVEYOR"));
    let result = happv.delete_project("booyaa".to_string(), "hello-homu".to_string());
    println!("{:#?}", result);
    assert!(result.is_ok());
}


// --- These tests are temporary for sketching out error handling
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
