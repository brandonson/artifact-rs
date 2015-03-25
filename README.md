#Artifact Logging Library

[![Build Status](https://travis-ci.org/brandonson/artifact-rs.png?branch=master)](https://travis-ci.org/brandonson/artifact-rs)

Artifact (or artifact-rs) is a logging library for the [rust language](https://github.com/rust-lang/rust).
It aims to provide effective logging in debug builds, while allowing release builds to limit or disable
logging entirely through Cargo configuration.

##Basic Usage

Within your main function, create an ArtifactGlobalLib object.  Then, use the `artifact::logger::Logger`
functionality to create and utilize your loggers.

##Documentation

The documentation is somewhat limited, but rustdocs are uploaded on every Travis build.

[Link](http://brandonson.github.io/artifact-rs/artifact/index.html)

##Examples

The bin directory has a few examples.

##Testing

This library desperately needs some unit tests, as there are none at the moment.

##License

Artifact (which is composed of all code within this, the artifact-rs github repository) is released under
the terms of the MIT license.

_Artifact, Copyright (c) 2014 Brandon Sanderson_
