# byterider

_Byte Rider, a shadowy flight into the dangerous world of a Rust crate that should not exist. Byte-al Knight, a memory-safe loner on a crusade to champion the cause of the innocent, the helpless, the powerless, in a world of binary formats who operate above the law._

This library provides byte and multi-byte-level access to memory without using any `unsafe` code. This crate is used by [Diosix](https://diosix.org) to access binary data in RAM. As such, it does not require the standard library. However, it will require a dynamic memory allocator to create binary structures.

If you wish to use this for your own project, let me know and I'll tidy up the documentation and API.

### Contact and code of conduct <a name="contact"></a>

Please [email](mailto:diosix@tuta.io) project lead Chris Williams if you have any questions or issues to raise, wish to get involved, have source to contribute, or have found a security flaw. You can, of course, submit pull requests or raise issues via GitHub, though please consider disclosing security-related matters privately. Please also observe the Diosix project's [code of conduct](https://diosix.org/docs/conduct.html) if you wish to participate.

### Copyright and license <a name="copyright"></a>

Copyright &copy; Chris Williams, 2020. See [LICENSE](LICENSE) for distribution and use of source code and binaries.