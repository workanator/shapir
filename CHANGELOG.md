# Changelog

Possible log types:

- `[added]` for new features.
- `[changed]` for changes in existing functionality.
- `[deprecated]` for once-stable features removed in upcoming releases.
- `[removed]` for deprecated features removed in this release.
- `[fixed]` for any bug fixes.
- `[security]` to invite users to upgrade in case of vulnerabilities.

### v0.2.1
- [changed] Changed contained value type of `UserId::Email` from `String` to `email::Mailbox`.
- [changed] Changed interface of creating `UserId` instance.

### v0.2.0
- [added] Added `api::shares::{Kind, AccessRight, Share, ShareConfig}`.
- [added] Added minimalistics Share Entity API implementation including share creation only at the moment.

### v0.1.3

- [added] Added user identifier `api::users::UserId`.

### v0.1.2

- [added] Move uploading chunk size from constant to managed option. See `Items::upload_chunk_size`.
- [added] New static function for initialization `Items::configured`.
- [added] Ability to create configured `Items` instance with `Connection::items_configured`.

### v0.1.1

- [changed] Changed the target URL of the crate documentation to [DOCS.RS](https://docs.rs/shapir).

### v0.1.0

- [changed] Refactor of `Error` type and error handling. The fixes could break compilation
            of the existing working code.

### v0.0.8

- [fixed] HTTP respose status code. Codes other than 5XX can return JSON content with error messages.
- [fixed] `Items::stat` tests the result error for `NotFound` to return `None` before returning `Err`.
- [added] Added the convenient method to take the absolute path from `Path::Absolute`.

### v0.0.7

- [fixed] Fixed `url` version to the same as in `hyper` to prevent build fails

### v0.0.6

- [added] Implemented `Items::upload`.

### v0.0.5

- [added] Implemented `Items::remove` and `Items::remove_bulk`.

### v0.0.4

- [added] Implemented `Items::mkdir`.

### v0.0.3

- The first version with usefull functionality.
