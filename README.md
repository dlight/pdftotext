# pdftotext

This crate extracts [poppler](https://poppler.freedesktop.org/)'s `pdftotext
-layout` code into a library, linking dynamically to system's poppler.

It was tested with poppler 20.12.1. It calls popper's internal APIs so it
may break with future library versions.