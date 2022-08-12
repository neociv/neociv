# ![NEOCIV](https://raw.githubusercontent.com/neociv/neociv/master/logo.svg)

# neociv-state

The "state" of any Neociv game is a struct that is managed by a high-level engine.

There are several goals of `NeocivState`:

- Serialisable
- Immutable
- Thread-safety
- Testing Testing Testing

## Serialisable

At all times the state can be written out to a JSON structure and read from the same structure. While the runtime state container is composed of high level structs there are no implementations beyond `Default` - the engine is responsible for high-level transformations in order to keep the translation from / to JSON as simple as possible.

## Immutable

The state is designed such that any change made to it generates an entire new structure from the point of view of the ECS in Bevy. The engine will transform and return new copies of the engine as needed.

## Thread-safety
