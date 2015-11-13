# StemShim

StemShim is a shim for compatibility with the StemJail's *kage* client.
It use the *LD_PRELOAD* feature to hook open-like functions of dynamically-linked ELF binaries.

Needs cargo >= v0.4.0 for dynamic library generation without hash suffix.
