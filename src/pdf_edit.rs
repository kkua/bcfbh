use std::fs::File;

use oxidize_pdf::{Document, DocumentMetadata, PdfDocument, page};

pub fn split_pages(pdf_doc: &PdfDocument<File>, booklet_pages: u32) {
    let pdf_metadata = pdf_doc.metadata().unwrap();
    let page_count = dbg!(pdf_doc.page_count().unwrap());
    let mut booklet_num = 0u32;
    let mut page_num = 0u32;
    // let booklet_idx = 0u32;
    // let mut page_idx = booklet_idx;
    while page_num < page_count {
        let booklet_start = booklet_num * booklet_pages;
        let mut booklet_end = booklet_start + booklet_pages;
        if booklet_end > page_count {
            booklet_end = page_count;
        }

        booklet_num += 1;
        retain_page_range(pdf_doc, booklet_num, booklet_start, booklet_end);
        dbg!(booklet_num);
        page_num = booklet_end;
    }
}

pub fn retain_page_range(
    pdf_doc: &PdfDocument<File>,
    booklet_num: u32,
    booklet_start_page: u32,
    booklet_end_page: u32,
) {
    let mut doc = Document::new();
    for page_idx in booklet_start_page..booklet_end_page {
        let parsed_page = pdf_doc.get_page(page_idx).unwrap();
        let clone_page = page::Page::from_parsed_with_content(&parsed_page, pdf_doc).unwrap();
        doc.add_page(clone_page);
    }
    // 编号固定2位数，不足补0
    doc.save(format!("booklet_{:02}.pdf", booklet_num)).unwrap();
}

fn set_pdf_metadata(doc: &mut Document, metadata: DocumentMetadata) {
    if let Some(title) = metadata.title {
        doc.set_title(&title);
    }
    if let Some(author) = metadata.author {
        doc.set_author(&author);
    }
    if let Some(subject) = metadata.subject {
        doc.set_subject(&subject);
    }
    if let Some(keywords) = metadata.keywords {
        doc.set_keywords(&keywords);
    }
}
