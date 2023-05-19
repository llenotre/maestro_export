# export

The export tool allows to export the Maestro kernel into an executable or shared library.

Exporting the kernel as a shared library allows to compile kernel modules without including the kernel's symbols.



# Why is this required?

The kernel cannot be exported directly with cargo because the kernel uses metadata to identify each crate, and those metadata are different for executables and shared library.

This produces different symbol names for the two versions, which prevents correct loading of kernel modules at runtime.
