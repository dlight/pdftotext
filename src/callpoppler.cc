//========================================================================
//
// pdftotext.cc
//
// Copyright 1997-2003 Glyph & Cog, LLC
//
// Modified for Debian by Hamish Moffatt, 22 May 2002.
//
//========================================================================

//========================================================================
//
// Modified under the Poppler project - http://poppler.freedesktop.org
//
// All changes made under the Poppler project to this file are licensed
// under GPL version 2 or later
//
// Copyright (C) 2006 Dominic Lachowicz <cinamod@hotmail.com>
// Copyright (C) 2007-2008, 2010, 2011, 2017-2020 Albert Astals Cid <aacid@kde.org>
// Copyright (C) 2009 Jan Jockusch <jan@jockusch.de>
// Copyright (C) 2010, 2013 Hib Eris <hib@hiberis.nl>
// Copyright (C) 2010 Kenneth Berland <ken@hero.com>
// Copyright (C) 2011 Tom Gleason <tom@buildadam.com>
// Copyright (C) 2011 Steven Murdoch <Steven.Murdoch@cl.cam.ac.uk>
// Copyright (C) 2013 Yury G. Kudryashov <urkud.urkud@gmail.com>
// Copyright (C) 2013 Suzuki Toshiya <mpsuzuki@hiroshima-u.ac.jp>
// Copyright (C) 2015 Jeremy Echols <jechols@uoregon.edu>
// Copyright (C) 2017 Adrian Johnson <ajohnson@redneon.com>
// Copyright (C) 2018 Klarälvdalens Datakonsult AB, a KDAB Group company, <info@kdab.com>. Work sponsored by the LiMux project of the city of Munich
// Copyright (C) 2018 Adam Reichold <adam.reichold@t-online.de>
// Copyright (C) 2018 Sanchit Anand <sanxchit@gmail.com>
// Copyright (C) 2019 Dan Shea <dan.shea@logical-innovations.com>
// Copyright (C) 2019 Oliver Sander <oliver.sander@tu-dresden.de>
//
//========================================================================

//========================================================================
//
// Part of pdftotext Rust crate
//
// Copyright (C) 2020 Elias Gabriel Amaral da Silva <tolkiendili@gmail.com>
//
//========================================================================

#include <iostream>
#include <iomanip>

#include <poppler/PDFDoc.h>
#include <poppler/PDFDocFactory.h>
#include <poppler/goo/GooString.h>
#include <poppler/TextOutputDev.h>
#include <poppler/GlobalParams.h>

using namespace std;

enum ResultCode {
    NoError = 0,
    InternalError = 1,
    CouldntReadPdf = 2,
    CouldntOutput = 3,
};

// typedef void (*TextOutputFunc)(void *stream, const char *text, int len);

typedef void (*NewPageFunc)(void *stream, int page);

extern "C" ResultCode pdftotext_print_with_layout(char *filename, void * stream, NewPageFunc newpage_f, TextOutputFunc output_f) {
    globalParams = std::make_unique<GlobalParams>();

    if (!(globalParams->getTextEncoding())) {
        return InternalError;
    }

    GooString *inputPdf = new GooString(filename);

    PDFDoc *doc = PDFDocFactory().createPDFDoc(*inputPdf, nullptr, nullptr);

    if (!doc->isOk()) {
        return CouldntReadPdf;
    }

    TextOutputDev* textOut;

    int lastPage = doc->getNumPages();

    textOut->setTextEOL(eolUnix);

    for (int pageNum = 1; pageNum <= lastPage; pageNum++) {
        newpage_f(stream, pageNum);

        textOut = new TextOutputDev(nullptr, true, 0.0, false, false, false);

        if (!textOut->isOk()) {
            return CouldntOutput;
        }

        doc->displayPage(textOut, pageNum, 72.0, 72.0, 0, true, false, false);

        TextPage *page = textOut->takeText();
        page->dump(stream, output_f, true, eolUnix, false);

        delete textOut;
    }

    return NoError;
}