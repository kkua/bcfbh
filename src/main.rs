use std::path::PathBuf;

use oxidize_pdf::{
    PdfDocument, PdfReader,
    operations::{self, PageRange, SplitMode, SplitOptions},
};

use crate::booklet::BindingRule;

mod booklet;
mod pdf_creator;
mod pdf_edit;
mod pdf_render;

fn main() {

    let filename = "input.pdf";
    let input_path = PathBuf::from(filename);
    let binding_rule = booklet::BindingRule::new(&input_path);
    let src_pdf = pdf_render::PdfDocumentHolder::new(&pdfium, &input_path, None);
    dbg!(src_pdf.get_page_count());
    booklet::create_booklet(&src_pdf, &binding_rule);
    // pdf_creator::create_booklet(&src_doc);
}
