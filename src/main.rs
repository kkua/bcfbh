use std::path::PathBuf;

use oxidize_pdf::{
    PdfDocument, PdfReader,
    operations::{self, PageRange, SplitMode, SplitOptions},
};

mod booklet;
mod pdf_edit;

fn main() {
    let reader = PdfReader::open("2.pdf").unwrap();
    // // let booklet_pages = booklet::calc_booklet_pages(pdf_doc.page_count().unwrap(), 4);
    let pdf_doc = PdfDocument::new(reader);
    // booklet::split_pdf(40, &pdf_doc);

    pdf_edit::split_pages(&pdf_doc, 40);
    let booklet_config = booklet::BindingRule {
        input_path: PathBuf::from("XTJGS.pdf"),
        output_dir: PathBuf::from("out"),
        sheets_per_booklet: 10,
    };
    booklet::split_pdf(&booklet_config);
}
