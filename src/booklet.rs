use std::{fs::File, path::PathBuf};

use oxidize_pdf::{
    PdfDocument, PdfReader,
    operations::{self, SplitMode, SplitOptions},
};

use crate::pdf_edit;

pub struct BindingRule {
    /// 输入PDF文件路径
    pub input_path: PathBuf,
    /// 输出目录（默认源文件所在目录下的out文件夹）
    pub output_dir: PathBuf,
    /// 每个小册子的A4纸数量（默认10张，即40页）
    pub sheets_per_booklet: usize,
    // /// 是否在首页前添加空白页作为封面
    // pub add_blank_cover: bool,
    // /// 是否添加页码
    // pub add_page_numbers: bool,
    // /// 页码格式
    // pub page_number_format: PageNumberFormat,
    // /// 页码位置
    // pub page_number_position: PageNumberPosition,
}

struct BookletConfig {
    booklet_sheets: u32,
    // 加1张纸的册子数量
    add_sheet_booklet_count: u32,
}

/// 计算每册的纸张数量
fn calc_booklet_pages(page_count: u32, sheets_per_booklet: u32) -> BookletConfig {
    let last_add = page_count % 4;
    // 对齐到4的倍数
    let total = page_count + last_add;
    // 每册对应的页数
    let pages_per_booklet = sheets_per_booklet * 4;
    // 获取册数
    let mut booklet_count = total / pages_per_booklet;
    // 最后一册的页数
    let last_booklet_pages = total % pages_per_booklet;
    let mut booklet_sheets = sheets_per_booklet;
    // 重新分配每册页数
    if (last_booklet_pages / 4 <= booklet_count) {
        // 最后一册全部分给前几册，每册多分1张纸
        let res = BookletConfig {
            booklet_sheets,
            add_sheet_booklet_count: last_booklet_pages / 4,
        };
        booklet_sheets += 1;

        return res;
    } else if (last_booklet_pages * 2 < pages_per_booklet) {
        // 小册子均分一下
        booklet_count += 1;
        // booklet_sheets 一定会小于 paper_count_per_booklet
        booklet_sheets = total / booklet_count;
        // remain_booklet_sheets 一定会小于 booklet_sheets
        let remain_booklet_sheets = (total - booklet_sheets * 4 * booklet_count) / 4;
        let booklet_config = BookletConfig {
            booklet_sheets,
            add_sheet_booklet_count: remain_booklet_sheets,
        };

        booklet_sheets += 1;
        return booklet_config;
    } else {
        BookletConfig {
            booklet_sheets,
            add_sheet_booklet_count: 0,
        }
    }
    // return booklet_papers;
}

pub fn split_pdf(config: &BindingRule) {
    let reader = PdfReader::open(&config.input_path).unwrap();
    let pdf_doc = PdfDocument::new(reader);
    let page_count = pdf_doc.page_count().unwrap() as u32;
    let booklet_config = calc_booklet_pages(page_count, config.sheets_per_booklet as u32);
    let mut booklet_idx = 0u32;
    let mut page_idx = 0u32;

    let pages_per_booklet = booklet_config.booklet_sheets * 4;
    while page_idx < page_count {
        let booklet_start_page = page_idx;
        let mut booklet_end_page = booklet_start_page + pages_per_booklet;
        if (booklet_idx < booklet_config.add_sheet_booklet_count) {
            booklet_end_page += 4;
        }
        if booklet_end_page > page_count {
            booklet_end_page = page_count;
        }
        booklet_idx += 1;
        pdf_edit::retain_page_range(&pdf_doc, booklet_idx, booklet_start_page, booklet_end_page);
        page_idx = booklet_end_page;
    }

    // pdf_edit::split_pages(&pdf_doc, booklet_config.booklet_sheets * 4);
    // let booklet_config = calc_booklet_pages(config.input_path.metadata().unwrap().len() as u32, config.sheets_per_booklet);
}

// pub fn split_pdf(booklet_pages: u32, pdf_doc: &PdfDocument<File>) {
//     // let doc = PdfDocument::new(reader);
//     // let reader = PdfReader::open("path");;
//     let page_count = pdf_doc.page_count().unwrap() as u32;
//     let mut booklet_num = 0u32;
//     let mut page_num = 0u32;
//     let mut options = SplitOptions {
//             mode: SplitMode::ChunkSize(booklet_pages),
//             preserve_metadata: true,
//             optimize: false,
//             output_pattern: "booklet_{}.pdf".to_string(),
//             ..Default::default()
//         };
//         // options.output_pattern = format!("booklet_{}.pdf", options);
//         // options.mode = SplitMode::chunked(booklet_start, booklet_end);
//         dbg!(operations::split::split_pdf("input.pdf", options).unwrap());
//     while page_num < page_count {
//         let booklet_start = booklet_num * booklet_pages;
//         let mut booklet_end = booklet_start + booklet_pages;
//         if booklet_end > page_count {
//             booklet_end = page_count;
//         }
//         // let mut options = SplitOptions {
//         //     mode: SplitMode::ChunkSize(booklet_pages),
//         //     preserve_metadata: true,
//         //     optimize: false,
//         //     output_pattern: format!("booklet_"),
//         //     ..Default::default()
//         // };
//         // // options.output_pattern = format!("booklet_{}.pdf", options);
//         // // options.mode = SplitMode::chunked(booklet_start, booklet_end);
//         // dbg!(operations::split::split_pdf("input.pdf", options));

//         // booklet_num += 1;
//         // page_num += booklet_pages;
//     }
// }
