# Tilby

Tilby is a personal information manager.

## Frictionless

Inspired by [Notational Velocity](http://notational.net/), Tilby   expects to be out of the way.

For example:

	$ tilby four score

Will search and autocomplete your documents, or create a new one with the the title as a filename.

Documents are stored on your computer as plain text.

## Social

Documents are synced to [Tilby](https://www.tilby.io). 

## Private

Documents will use your public key for encryption and decryption. If you care to share a private document, another user’s document would need to be signed.

## Design

Rust was chosen so installation would be friction-free. No extraneous virtual machine is needed. It Just 

Every document has a permanent, immutable name, a [UUID](https://en.wikipedia.org/wiki/Universally_unique_identifier), influenced by Joe Armstrong’s [The Web of Names](https://joearms.github.io/2015/03/12/The_web_of_names.html).

	tilby://de305d54-75b4-431b-adb2-eb6b9e546013

This document is a JSON file. which 



