# TLDR - Lambda
Tang in a Lambda with Database, written in Rust.

## Description
An inexpensive high-availability/fault-tolerant Tang server in AWS.

The TLDR Lambda is a fork and reimplementation of Martyn P's [Tangy](https://github.com/martynp/tangy), which is itself a reimplementation of Latchset's [Tang](https://github.com/latchset/tang) server.

TLDR focuses on creating a highly-available fault-tolerant service, where the original Tang and Tangy are standalone services. This approach runs counter to the [official guidance](https://github.com/latchset/tang/blob/master/doc/tang.8.adoc#high-availability) which recommends binding clients to multiple servers, however the increased uptime and resilience is a desirable side effect of pushing keys into an external backend. Clients may still bind to multiple instances for increased diversity.

The Tang protocol allows clients to store secrets which can only be recovered when they have access to the Tang server. For example, the Clevis tools allows the automated decryption of LUKS partitions when the encrypted device is connected to the local network that Tang is accessible on.

See the original Tang project for a complete description: https://github.com/latchset/tang

Fraser Tweedale's 2020 Linux Conference Australia talk on "Clevis and Tang: securing your secrets at rest" is a great resource:

[![Clevis and Tang: securing your secrets at rest](https://img.youtube.com/vi/Dk6ZuydQt9I/0.jpg)](https://www.youtube.com/watch?v=Dk6ZuydQt9I)


## Usage

(This section is forthcoming. Use `cargo-lambda` for local debugging and production deployment, or manually deploy the lambda.)

## To / From Tangd

(This section is forthcoming. It will entail interacting directly with the backend DynamoDB instance.)

## Credits

The original author of Tangy is Martyn P.

The original authors of [Tang](https://github.com/latchset/tang) are [Latchset](https://github.com/latchset/). Tang is based on the protocol described by Nathaniel McCallum and Robert Relyea (https://marc.info/?m=144173814525805).

## License

Apache-2.0 or MIT - you decide!

## How to Contribute

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed (Apache-2.0 and MIT), without any additional terms or conditions.
