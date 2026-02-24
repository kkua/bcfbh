use native_dialog::DialogBuilder;

mod booklet;
mod pdf_creator;
mod pdf_render;

fn main() {
    let path = DialogBuilder::file()
        // .set_location("~/Desktop")
        .add_filter("PDF", ["pdf"])
        .set_title("选择源文件")
        .open_single_file()
        .show()
        .unwrap()
        .expect("必须选择一个文件");
    println!("{}", path.to_string_lossy());

    let out_path = DialogBuilder::file()
        .set_title("选择输出目标文件夹")
        .open_single_dir()
        .show()
        .unwrap();
    // println!("{}", out_path.to_string_lossy());
    let pdfium = pdf_render::init_pdfium();
    let input_path = path;
    let binding_rule = booklet::BindingRule::new(&input_path);
    let binding_rule = booklet::BindingRule {
        binding_at_middle: true,
        sheets_per_booklet: 10,
        ..binding_rule
    }
    .set_output_path(&out_path);
    let src_pdf = pdf_render::PdfDocumentHolder::new(&pdfium, &input_path, None);
    dbg!(src_pdf.get_page_count());
    booklet::create_booklet(&src_pdf, &binding_rule);
}
