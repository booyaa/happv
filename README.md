# happv

| travis-ci | appveyor |
|-----------|----------|
| [![Build Status](https://travis-ci.org/booyaa/happv.svg?branch=master)](https://travis-ci.org/booyaa/happv) | [![Build Status](https://ci.appveyor.com/api/projects/status/github/booyaa/happv)] |

A minimal AppVeyor API library in Rust.

Warning: this is a partial implemented API of the AppVeyor REST [API][1]. I only need the following endpoints:

- Get a list of projects - GET /api/projects
- Add a project - POST /api/projects
- Delete a project - DELETE /api/projects/{accountName}/{projectSlug}

The following have not yet been implemented:

- Get the last build of a project - GET /api/projects/{accountName}/{projectSlug}
- Cancel a build - DELETE /api/builds/{accountName}/{projectSlug}/{buildVersion}

[PRs][2] welcome if you want to implement other endpoints and/or the Build Worker API.

Full documentation can be found [here][3].


# Usage

This crate is on [crates.io][4] and can be used by adding `happv` to the dependencies in your project's `Cargo.toml`.

```toml
[dependencies]
happv = "0.1.*"
```

and this to your crate root:

```rust
extern crate happv;
```

# Example

How to get a list of projects on AppVeyor

```rust
use happv::AppVeyor;

let happv = AppVeyor::new(env!("APPVEYOR"));
let result = happv.get_projects().unwrap();

assert!(0 < result.len());

println!("Get project list:");
for i in result.into_iter() {
    println!("\tId:{} Slug:{}", i.project_id, i.slug);
}
```
# Contributing

## Rules

- Always write a test for the new feature
- When implementing a new endpoint, always use a test fixture (see tips) and write the test using `enable_test_mode`.
- Until I can find a better way of separating integration tests, decorate those
tests with `[ignore]` so they won't trip up travis.

## Tips

You may find it easy to work off a cached copy of the JSON response. I've placed my original test fixtures in tests/fixtures.

Retrieving a collection or single item

```
curl --silent --header 'Authorization: Bearer '$APPVEYOR \
  --request GET https://ci.appveyor.com/api/projects
```

Cancelling

```
curl --silent --header 'Authorization: Bearer '$APPVEYOR \
  --request DELETE https://ci.appveyor.com/api/builds/booyaa/hai/1.0.11
```

Adding/Modifying entries

```
curl --silent --header 'Authorization: Bearer '$APPVEYOR \
  --header 'Content-Type: application/json' \
  --request POST https://ci.appveyor.com/api/projects \
  -d '{"repositoryProvider" : "gitHub", "repositoryName" : "booyaa/hello-homu"}'

```

# Copyright

Copyright 2016 Mark Sta Ana.

Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
http://www.apache.org/licenses/LICENSE-2.0> at your option. This file may not
be copied, modified, or distributed except according to those terms.

[1]: https://www.appveyor.com/docs/api/
[2]: https://github.com/booyaa/happv/issues/new
[3]: https://docs.rs/happv
[4]: https://crates.io/crates/happv
