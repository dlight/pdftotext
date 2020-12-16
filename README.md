# pdftotext

This crate extracts [Poppler](https://poppler.freedesktop.org/)'s `pdftotext
-layout` code into a library, linking dynamically to system's Poppler.

The library was tested with Poppler 20.12.1. It calls popper's internal APIs so it
may break with future library versions. If this is a concern, build with
`static-poppler` enabled, which statically links vendored Poppler 20.12.1.
