# Handling groups of coordinates

## Status

Accepted

## Context

How to handle transformations on groups of coordinates in a single pass. Also contextually aware operations such as bi-linear interpolation scaling algorithms.

## Decision

Implementing a core API object for use as defined by HexShape in [shape.rs](../src/hex/shape.rs). This will facilitate most if not all inter system communications.

## Consequences

### Ease
- Decoupling storage and operations is easier since the storage has to output a shape and the algorithms expect a shape.

### Pain
- Forces a lot of responsibility into the shape struct that it may or may not even need to do.