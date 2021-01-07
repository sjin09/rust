## Place modules in their own file

For all except very short modules (<100 lines) and tests, place the module foo in a separate file, as in:

```
pub mod foo;

// in foo.rs or foo/mod.rs
pub fn bar() { println!("..."); }
/* ... */
```

rather than declaring it inline:

```
pub mod foo {
    pub fn bar() { println!("...");}
}
```

### Use subdirectories for modules with children

For modules that themselves have submodules, place the module in a separate directory (e.g., bar/mod.rs for a module bar) rather than the same directory.

Note the structure of std::io. Many of the submodules lack children, like io::fs and io::stdio. On the other hand, io::net contains submodules, so it lives in a separate directory:

```
io/mod.rs
   io/extensions.rs
   io/fs.rs
   io/net/mod.rs
          io/net/addrinfo.rs
          io/net/ip.rs
          io/net/tcp.rs
          io/net/udp.rs
          io/net/unix.rs
   io/pipe.rs
   ...
```

