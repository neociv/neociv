# ![NEOCIV](https://raw.githubusercontent.com/neociv/neociv/master/logo.svg)

# neociv-civil

Lua based API for interacting with the state of a Neociv game.

## (CIV)(I)n(L)ua *or* (CIV)(I)n(L)isp

Supports both Lua and Lisp (via Fennel) syntax. Syntax is chosen by file extension (.lua or .fnl / .cvl) - fennel will be transpiled at runtime. Examples will be shown with both syntaxes.

## cvl

The top-level API exists entirely in the `cvl` global.

### Events

To listen to events the `on` function is used. Look to an event's documentation to see what data is provided. For the most part any data can be found by querying the state, any data provided to events is simply for convenience.

```lua
cvl.on("example", handler)
```

```fennel
(cvl.on :example handler)
```

| Event | Data | Description |
| ----- | ---- | ----------- |

## Third Party Libraries

The runtime API includes several compiled-in SDKs, some exposed via the `cvl` global and others available as importable modules for your own code. All of these modules are MIT licensed.

[fennel](https://fennel-lang.org)

[inspect.lua](https://github.com/kikito/inspect.lua)
