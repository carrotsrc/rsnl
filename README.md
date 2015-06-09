# RSNL

This is the beginning of a rust library for interacting with netlink via libnl and their protocol suite (huge respect for their library, it is really great).

The plan is to structure it out similar to the library (core, route, genl, nf), but appropiately abstracted in ways that make it geared more toward being a rust interface. The separation means, like libnl, you only link with what you use.

This library is an exercise in building a clean interface that still feels familiar. It is function orientated, like the C library, with minimal use of structs. The functions and structs are split into submodules, taking advantage of namespaces so `nlmsg_alloc()` becomes `rsnl::message::alloc()`. The convention being to make the namespace the relevent context so allocating a new socket becomes `rsnl::socket::alloc()`. With that said, The design is still in flux with experiments in different ideas.

One thing that is experimented with is avoid handling any voids (a-*void*, if you will), so hiding 'em away behind the API with casting performed by generics.

Suggestions, changes and improvements are always welcome!

#### Dependencies

* [libnl-3](http://www.infradead.org/~tgr/libnl/)

#### Suite bindings

* rsnl -- this
* [rsgnl](http://www.github.com/carrotsrc/rsgnl) -- the generic netlink library interface

#### License

**MIT**

(Originally it was LGPL out of respect to libnl but really it's just a small layer, so in retrospect LGPL seems a bit heavy weight.)
