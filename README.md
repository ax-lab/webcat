# webcat

Lightning fast tool to help developers debug and test Web/HTTP requests.

## Project goals

The goal of this project is to create a lightning fast HTTP client to be used
as a debugging and testing tool by developers.

The main features that set this project apart from other tools are:

- Handle massive requests (e.g. a one-gigabyte JSON result):
  - Most tools just crash and burn spectacularly when requests go even into the
    megabytes range.
  - Falling back to tools such as `curl` is a viable option, but cumbersome.
- Allow querying the request data once it is loaded:
  - This is important for debugging, particularly massive requests.
  - The ability to save the data and inspect it later using different queries
    can be extremely powerful.

In addition to the above, the following features are also planned:

- Save and manage collections of requests as plain files:
  - Easy to share, so teams can build a set of common requests.
  - Can be easily copied, edited, shared, added to version control, etc.
  - Have ways to import request configurations for improved reuse.
- Set up environments that can be used to parameterize a request:
  - Allows reusing saved requests between different environments.
  - Environments themselves are plain files, like requests.
- Support local variables and secrets (e.g. logins and passwords):
  - Similar to environments but for local use, not sharing.
  - Secrets can be safely stored locally, instead of as plain-text.
  - Support running an arbitrary command to retrieve a secret (e.g. from
    a wallet application).
  - Teams can share requests but still have individualized logins and settings.
- Support automation through the command line and scripts:
  - For example, defining a collection as a test suite and run it with a
    single command.

The goal is to have all those features first by using plain text files and as
a command-line tool, later adding a GUI on top to provide things like improved
data visualization and ease of use.

## Development

This project uses submodules:

```sh
# execute these commands after cloning
git submodule init
git submodule update
```
