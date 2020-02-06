# rust-kafka

This is a barebones producer/consumer scenario w/ struct serialization/deserialization. Its purpose is to demonstrate how to publish messages (produce) and subscribe to a topic (consume) to fetch published messages.

## explanations

What each file does, and how to adapt it to other systems.

### `main.rs`

1. Runs a loop in its own thread that publishes a new Data object to kafka with random contents, once per second.
2. Runs a loop in the main thread to consume whatever's been published to kafka every 200ms.

### `produce.rs`

Produces (or publishes) some data to kafka. `produce` accepts a key and a value. Data must be serialized with `encode::encode`.

### `consume.rs`

Consumes (or subscribes to) data from kafka.

`consume` accepts one parameter (a Consumer object) and consumes the contents of kafka's queue ONCE. Note that we don't return the actual value(s) that we retrieved, only a pass/fail `Ok(()) or Err(...)`. We don't return the values because there may not be any, and trying to pass those values around could become very spaghetti.

> _`consume` should be our command dispatcher_.

In a real pub/sub system, consume would have to be modified to dispatch commands based on each value retrieved from the message queue (`m in ms.messages()`).

This code...

```
println!("consumed {}:\n{}", key, dat);
```

... would be replaced with some code that matches `key` and fires off the appropriate function with the value (`dat`).

`do_consume` creates a Consumer object and calls `consume()` in a loop. Synchronously polls kafka and handles errors from `consume`, sleeping 200ms between polls.

### `data.rs`

Definitions for data structures:

#### `Key`

An enum that acts as an identifier; a "message type." This would represent a type of operation, like "MintErc20."

#### `Data`

A simple struct that holds a couple different fields. Allows us to demonstrate what serialization/deserialization looks like. This represents the operation parameters/data (e.g. `MintParams`).

### `encode.rs`

Contains functions which are used for serializing (`encode`) and deserializing (`decode`) our data objects (both Key and Data and anything else that implements serde::{Deserialize, Serialize}). These functions must be used to pass data to `produce` and `consume`.

### `common.rs`

Just responsible for holding host info for connecting to kafka.

> Note: Change `IP` manually after running the dockerized kafka system. We can't use `localhost` here for some reason.
