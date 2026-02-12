use std::path::PathBuf;

mod booklet;
mod pdf_creator;
mod pdf_edit;
mod pdf_render;

fn main() {
    let pdfium = pdf_render::init_pdfium();
    let filename = "XTJGS.pdf";
    let input_path = PathBuf::from(filename);
    let binding_rule = booklet::BindingRule::new(&input_path);
    let binding_rule = booklet::BindingRule {
        binding_at_middle: false,
        sheets_per_booklet: 16,
        ..binding_rule
    };
    let src_pdf = pdf_render::PdfDocumentHolder::new(&pdfium, &input_path, None);
    dbg!(src_pdf.get_page_count());
    booklet::create_booklet(&src_pdf, &binding_rule);
}
