# happv

A minimal AppVeyor API library in Rust.

Warning: this is a partial implemented API of the AppVeyor REST API. I only need the following endpoints:

- Get a list of projects - /api/projects
- Get the last build of a project - /api/projects/{accountName}/{projectSlug}
- Add a project - /api/projects
- Cancel a build - /api/builds/{accountName}/{projectSlug}/{buildVersion}

PRs welcome if you want to implement other endpoints and/or the Build Worker API.

# Usage

to be completed.

# Copyright

Copyright 2016 Mark Sta Ana.

Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
http://www.apache.org/licenses/LICENSE-2.0> at your option. This file may not
be copied, modified, or distributed except according to those terms.
