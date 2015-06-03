# RSNL

This is the beginning of a library for netlink and libnl-3.

The plan is to structure it out similar to libnl (core, route, genl, nf), but appropiately abstracted in ways that make it geared more toward being a rust interface.

Recently cleaning the API -- dropped struct impl in favour of functions, where the context of the function is the module namespace. This feels a lot cleaner.

#### License

Out of respect to Libnl, the library is licensed under LGPL
