# ![no_std] support

## Status

Accepted

## Context

In order to support portability and a wide variety of usecases code bases eliminate standard library usage via `#![no_std]`.

## Decision

In order to have our repository also support `#![no_std]` we must change our import structure. inside the lib.rs file will exist a lib module that stores every specific use expression the library uses in regards to an external crate. Then in every library file we use a `use crate::lib::*;` glob import to use those expressions.

## Consequences

What becomes easier or more difficult to do because of this change?

### Ease
- Defined requirements for library usage in different environment assumptions.
- Easy switching of implementations based on features.
- Unified place of authority of what the library uses.
### Pain
- Requires library developers to know what they need to import and add it to the library internal glob import.
- Glob import in library code, while not bad due to diligient use of specific use expressions in the lib mod have a potential to be a smell.
