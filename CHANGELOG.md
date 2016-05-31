# Changelog

Possible log types:

- `[added]` for new features.
- `[changed]` for changes in existing functionality.
- `[deprecated]` for once-stable features removed in upcoming releases.
- `[removed]` for deprecated features removed in this release.
- `[fixed]` for any bug fixes.
- `[security]` to invite users to upgrade in case of vulnerabilities.

### Latest

- [fixed] HTTP respose status code. Codes other than 5XX can return JSON content with error messages.
- [fixed] `Items::stat` tests the result error for `NotFound` to return `None` before returning `Err`.
- [add] Added the convenient method to take the absolute path from `Path::Absolute`.

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
