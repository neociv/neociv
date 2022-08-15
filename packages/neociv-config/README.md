# ![NEOCIV](https://raw.githubusercontent.com/neociv/neociv/master/logo.svg)

# neociv-config

High level configuration objects for the game that can be expanded to be wholly controlled by scripts.

## Priority

Neociv will always prioritise _static_ configuration files over dynamic ones as it allows the game to easily save / load settings from a friendly GUI interface. Deleting `config.json` isn't enough - you must also provide one of the other files.

In order of priority:

1. config.json
2. config.lua
3. config.fnl
4. config.cvl

## --config-file

You can manually specify a config file to load which is helpful for safely developing your own dynamic config script.

```bash
neociv --config-file my-super-unsafe-config.lua
```

## Saving

The game will only save the configuration file if a JSON file is used.