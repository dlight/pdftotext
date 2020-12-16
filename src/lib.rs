#![allow(dead_code)]

use std::{
    ffi::CString,
    os::raw::{c_char, c_int, c_void},
};

use std::fmt;

#[repr(C)]
enum ResultCode {
    NoError = 0,
    InternalError = 1,
    CouldntReadPdf = 2,
    CouldntOutput = 3,
}

#[derive(Debug)]
pub enum Error {
    InternalError,
    CouldntReadPdf,
    CouldntOutput,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            Error::InternalError => {
                "Could not get text encoding, something about poppler is broken"
            }
            Error::CouldntReadPdf => "Could not read PDF file",
            Error::CouldntOutput => "Could not create text output",
        };
        write!(f, "{}", str)
    }
}

impl std::error::Error for Error {}

type NewPageFunc = extern "C" fn(stream: *mut c_void, page: c_int);

type TextOutputFunc = extern "C" fn(stream: *mut c_void, text: *const c_char, len: c_int);

extern "C" {
    fn pdftotext_print_with_layout(
        filename: *const c_char,
        stream: *mut c_void,
        newpage_f: NewPageFunc,
        output_f: TextOutputFunc,
    ) -> ResultCode;
}

extern "C" fn newpage_callback(stream: *mut c_void, page: c_int) {
    let vec = stream as *mut Vec<String>;
    let vec = unsafe { &mut *vec };

    let page = page as usize;

    assert!(page - 1 <= vec.len());

    if page > vec.len() {
        vec.push(String::new());
    }
}

extern "C" fn output_callback(stream: *mut c_void, text: *const c_char, len: c_int) {
    let vec = stream as *mut Vec<String>;
    let vec = unsafe { &mut *vec };

    let slice = unsafe { std::slice::from_raw_parts(text as *const u8, len as usize) };

    let str = std::str::from_utf8(slice).unwrap();

    let veclen = vec.len();

    vec[veclen - 1].push_str(str);
}

pub fn pdftotext_layout(filename: &str) -> Result<Vec<String>, Error> {
    let mut vec = vec![];

    let c_filename = CString::new(filename).unwrap();

    let _ = unsafe {
        pdftotext_print_with_layout(
            c_filename.as_ptr(),
            &mut vec as *mut Vec<String> as *mut c_void,
            newpage_callback,
            output_callback,
        )
    };

    Ok(vec)
}

#[test]
fn test() {
    let a = pdftotext_layout("test.pdf").unwrap();
    assert_eq!(a, vec!["                          Adobe Acrobat PDF Files\nAdobe® Portable Document Format (PDF) is a universal file format that preserves all\nof the fonts, formatting, colours and graphics of any source document, regardless of\nthe application and platform used to create it.\n\nAdobe PDF is an ideal format for electronic document distribution as it overcomes the\nproblems commonly encountered with electronic file sharing.\n\n•   Anyone, anywhere can open a PDF file. All you need is the free Adobe Acrobat\n    Reader. Recipients of other file formats sometimes can\'t open files because they\n    don\'t have the applications used to create the documents.\n\n•   PDF files always print correctly on any printing device.\n\n•   PDF files always display exactly as created, regardless of fonts, software, and\n    operating systems. Fonts, and graphics are not lost due to platform, software, and\n    version incompatibilities.\n\n•   The free Acrobat Reader is easy to download and can be freely distributed by\n    anyone.\n\n•   Compact PDF files are smaller than their source files and download a\n    page at a time for fast display on the Web.\n"])
}
