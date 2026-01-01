A rust wrapper for the Mustang CLI.

In the future I might use jni but currently I just call it in a subprocess.

### Notice

This project is in no way affiliated with the Mustang Project.

### Other notes

To reduce the dependency size, I've been experimenting with ProGuard and GraalVM, but so far haven't been able to get it to work.

# Building the JRE

`build.rs` will build the JRE in ENV `OUT_DIR` if the cargo feature `jlink` is enabled.

It will also check if the binary is the right architecture by running `file` on it,
but that only works for select architectures. To disable the check, enable the cargo feature `skip-bin-check`.
