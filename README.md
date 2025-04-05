# Moxctl

A command-line utility for controlling the Mox desktop environment.

## Overview

`moxctl` automatically searches your system's `$PATH` for binaries that start with `mox` and end with `ctl`. It then adds these binaries as subcommands, making it easy to invoke them through a unified command. For example, a binary like `moxnotifyctl` will be accessible as a subcommand `mox notify`.

## Usage

### Basic Command Structure

Once installed, you can invoke `moxctl` with any of the detected subcommands by simply calling:

```
mox <subcommand> [options]
```

For instance, if the `moxnotifyctl` binary is detected in your `$PATH`, you can invoke it like this:

```
mox notify [options]
```

