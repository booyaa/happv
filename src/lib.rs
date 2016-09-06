// - Get the last build of a project - /api/projects/{accountName}/{projectSlug}
// curl --silent --header 'Authorization: Bearer '$APPVEYOR --request GET \
//      https://ci.appveyor.com/api/projects/booyaa/hai
//
//

extern crate hyper;
#[macro_use]
extern crate log;

use hyper::Client;

use hyper::header::ContentType;
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};

use std::io::Read;

// const LIB_VERSION: &'static str = env!("CARGO_PKG_VERSION");
// const LIB_NAME: &'static str = env!("CARGO_PKG_NAME");

const BASE_URL: &'static str = "http://localhost:32768/";
const BASE_PROJECTS: &'static str = "/headers";
const BASE_BUILDS: &'static str = "/post";
// const BASE_URL: &'static str = "https://ci.appveyor.com/api/";
// const BASE_PROJECTS: &'static str = "projects";
// const BASE_BUILDS: &'static str = "builds";

/// Main struct
pub struct AppVeyor {
    token: String,
}

impl AppVeyor {
    pub fn new(token: &str) -> AppVeyor {
        AppVeyor { token: token.to_string() }
    }


    /// Get a list of projects - /api/projects
    ///
    /// curl --silent --header 'Authorization: Bearer '$APPVEYOR --request GET \
    ///         https://ci.appveyor.com/api/projects
    ///
    /// GET /api/projects
    pub fn get_projects(&self) -> String {
        let client = Client::new();
        let url = format!("{}{}", BASE_URL, BASE_PROJECTS);
        debug!("url: {}", url);

        let mut res = client.get(&url).send().unwrap();

        if res.status != hyper::status::StatusCode::Ok {
            panic!(res.status.to_string());
        }

        let mut buffer = String::new();
        res.read_to_string(&mut buffer).expect("no body");
        debug!("buffer: {}", buffer);

        buffer
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
        // let url = format!("{}{}", BASE_URL, "/post");
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


#[test]
fn test_httpbin_get() {
    let result = AppVeyor::new("foo");
    let expected = "{\n  \"headers\": {\n    \"Host\": \"localhost:32768\"\n  }\n}\n";
    assert_eq!(expected, result.get_projects());
}

#[test]
fn test_httpbin_post() {
    let happv = AppVeyor::new("foo");
    let result = happv.add_project();
    let expected = "xxx";
    assert_eq!(expected, result);
}

#[test]
#[ignore]
fn test_httpbin_delete() {
    let happv = AppVeyor::new("foo");
    let result = happv.cancel_build();
    let expected = "xxx";
    println!("{}", result);
    assert_eq!(expected, result);

}
