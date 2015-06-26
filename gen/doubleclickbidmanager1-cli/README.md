<!---
DO NOT EDIT !
This file was generated automatically from 'src/mako/cli/README.md.mako'
DO NOT EDIT !
-->
The `doubleclickbidmanager1` command-line interface *(CLI)* allows to use most features of the *Google DoubleClick Bid Manager* service from the comfort of your terminal.

By default all output is printed to standard out, but flags can be set to direct it into a file independent of your shell's
capabilities. Errors will be printed to standard error, and cause the program's exit code to be non-zero.

If data-structures are requested, these will be returned as pretty-printed JSON, to be useful as input to other tools.

Everything else about the *DoubleClick Bid Manager* API can be found at the
[official documentation site](https://developers.google.com/bid-manager/).

# Downloads

You can download the pre-compiled 64bit binaries for the following platforms:

* ![icon](http://megaicons.net/static/img/icons_sizes/6/140/16/ubuntu-icon.png) [ubuntu](http://dl.byronimo.de/google.rs/cli/0.3.0/ubuntu/doubleclickbidmanager1.tar.gz)
* ![icon](http://hydra-media.cursecdn.com/wow.gamepedia.com/a/a2/Apple-icon-16x16.png?version=25ddd67ac3dd3b634478e3978b76cb74) [osx](http://dl.byronimo.de/google.rs/cli/0.3.0/osx/doubleclickbidmanager1.tar.gz)

Find the source code [on github](https://github.com/Byron/google-apis-rs/tree/master/gen/doubleclickbidmanager1-cli).

# Usage

This documentation was generated from the *DoubleClick Bid Manager* API at revision *20150326*. The CLI is at version *0.3.0*.

```bash
doubleclickbidmanager1 [options]
        lineitems
                downloadlineitems (-r <kv>)... [-p <v>]... [-o <out>]
                uploadlineitems (-r <kv>)... [-p <v>]... [-o <out>]
        queries
                createquery (-r <kv>)... [-p <v>]... [-o <out>]
                deletequery <query-id> [-p <v>]...
                getquery <query-id> [-p <v>]... [-o <out>]
                listqueries [-p <v>]... [-o <out>]
                runquery <query-id> (-r <kv>)... [-p <v>]...
        reports
                listreports <query-id> [-p <v>]... [-o <out>]
  doubleclickbidmanager1 --help

Configuration:
  --config-dir <folder>
            A directory into which we will store our persistent data. Defaults to 
            a user-writable directory that we will create during the first invocation.
            [default: ~/.google-service-cli]
  --debug
            Output all server communication to standard error. `tx` and `rx` are placed 
            into the same stream.
  --debug-auth
            Output all communication related to authentication to standard error. `tx` 
            and `rx` are placed into the same stream.

```

# Configuration

The program will store all persistent data in the `~/.google-service-cli` directory in *JSON* files prefixed with `doubleclickbidmanager1-`.  You can change the directory used to store configuration with the `--config-dir` flag on a per-invocation basis.

More information about the various kinds of persistent data are given in the following paragraphs.

# Authentication

Most APIs require a user to authenticate any request. If this is the case, the [scope][scopes] determines the 
set of permissions granted. The granularity of these is usually no more than *read-only* or *full-access*.

If not set, the system will automatically select the smallest feasible scope, e.g. when invoking a
method that is read-only, it will ask only for a read-only scope. 
You may use the `--scope` flag to specify a scope directly. 
All applicable scopes are documented in the respective method's CLI documentation.

The first time a scope is used, the user is asked for permission. Follow the instructions given 
by the CLI to grant permissions, or to decline.

If a scope was authenticated by the user, the respective information will be stored as *JSON* in the configuration
directory, e.g. `~/.google-service-cli/doubleclickbidmanager1-token-<scope-hash>.json`. No manual management of these tokens
is necessary.

To revoke granted authentication, please refer to the [official documentation][revoke-access].

# Application Secrets

In order to allow any application to use Google services, it will need to be registered using the 
[Google Developer Console][google-dev-console]. APIs the application may use are then enabled for it
one by one. Most APIs can be used for free and have a daily quota.

To allow more comfortable usage of the CLI without forcing anyone to register an own application, the CLI
comes with a default application secret that is configured accordingly. This also means that heavy usage
all around the world may deplete the daily quota.

You can workaround this limitation by putting your own secrets file at this location: 
`~/.google-service-cli/doubleclickbidmanager1-secret.json`, assuming that the required *doubleclickbidmanager* API 
was enabled for it. Such a secret file can be downloaded in the *Google Developer Console* at 
*APIs & auth -> Credentials -> Download JSON* and used as is.

Learn more about how to setup Google projects and enable APIs using the [official documentation][google-project-new].


# Debugging

Even though the CLI does its best to provide usable error messages, sometimes it might be desirable to know
what exactly led to a particular issue. This is done by allowing all client-server communication to be 
output to standard error *as-is*.

The `--debug` flag will print all client-server communication to standard error, whereas the `--debug-auth` flag
will cause all communication related to authentication to standard error.
If the `--debug` flag is set, error-results will be debug-printed, possibly yielding more information about the 
issue at hand.

You may consider redirecting standard error into a file for ease of use, e.g. `doubleclickbidmanager1 --debug <resource> <method> [options] 2>debug.txt`.


[scopes]: https://developers.google.com/+/api/oauth#scopes
[revoke-access]: http://webapps.stackexchange.com/a/30849
[google-dev-console]: https://console.developers.google.com/
[google-project-new]: https://developers.google.com/console/help/new/