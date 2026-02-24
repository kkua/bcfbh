use crate::booklet::BindingRule;
use crate::pdf_render::PdfDocumentHolder;
use oxidize_pdf::Color;
use oxidize_pdf::Document;
use oxidize_pdf::Font;
use oxidize_pdf::Page;
use oxidize_pdf::graphics::LineDashPattern;
use pdfium_render::prelude::PdfDocumentMetadataTagType;

/// 创建册子
///
/// # 参数
/// * `src_pdf` - 源PDF文档容器
/// * `binding_rule` - 装订规则
/// * `booklet_idx` - 册子索引
/// * `booklet_start_page` - 小册子开始页索引(包含)
/// * `booklet_end_page` - 小册子结束页索引(不包含)
pub fn create_booklet(
    src_pdf: &PdfDocumentHolder,
    binding_rule: &BindingRule,
    booklet_idx: u16,
    booklet_start_page: u16,
    booklet_end_page: u16,
) {
    let mut doc = Document::new();
    write_pdf_metadata(src_pdf, &mut doc);
    let file_name = binding_rule
        .input_path
        .file_prefix()
        .expect("没有文件名")
        .to_string_lossy();
    doc.set_title(format!("booklet #{}", booklet_idx));
    let mut page_idx = booklet_start_page;
    while page_idx < booklet_end_page {
        if let Some(page) = create_page(
            src_pdf,
            page_idx,
            booklet_start_page,
            booklet_end_page,
            booklet_idx,
            binding_rule,
        ) {
            doc.add_page(page);
        } else {
            break;
        }
        page_idx += 1;
    }

    doc.save(format!(
        "{}/{}_{:02}.pdf",
        binding_rule.output_dir.display(),
        file_name,
        booklet_idx
    ))
    .unwrap();

    println!(
        "完成第{}册，共{}页, 开始页: {}, 结束页: {}",
        booklet_idx,
        booklet_end_page - booklet_start_page,
        booklet_start_page,
        booklet_end_page
    );
}

/// 设置PDF文档的元数据
///
/// # 参数
/// * `src_pdf` - 源PDF文档容器
/// * `doc` - 目标PDF文档对象
fn write_pdf_metadata(src_pdf: &PdfDocumentHolder<'_>, doc: &mut Document) {
    let creator = format!(
        "{} v{} - {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_DESCRIPTION")
    );
    doc.set_creator(creator);
    // doc.set_producer(pkg_name);

    if let Some(author) = src_pdf.metadata().get(PdfDocumentMetadataTagType::Author) {
        let author_value = author.value();
        if !author_value.is_empty() {
            doc.set_author(author_value);
        }
    }

    if let Some(subject) = src_pdf.metadata().get(PdfDocumentMetadataTagType::Subject) {
        let subject_value = subject.value();
        if !subject_value.is_empty() {
            doc.set_subject(subject_value);
        }
    }

    if let Some(keywords) = src_pdf.metadata().get(PdfDocumentMetadataTagType::Keywords) {
        let keywords_value = keywords.value();
        if !keywords_value.is_empty() {
            doc.set_keywords(keywords_value);
        }
    }
}

fn create_page(
    src_pdf: &PdfDocumentHolder,
    page_idx: u16,
    group_start_idx: u16,
    group_end_idx: u16,
    booklet_num: u16,
    binding_rule: &BindingRule,
) -> Option<Page> {
    let page_low_idx = page_idx;
    let mut page_high_idx = group_end_idx - page_idx + group_start_idx - 1;
    let binding_at_middle = binding_rule.binding_at_middle;
    if page_low_idx >= page_high_idx {
        // 本册结束了
        return None;
    }
    if !binding_at_middle {
        page_high_idx = (group_end_idx - group_start_idx + 1) / 2 + page_idx;
    }

    let page_count = src_pdf.get_page_count();

    let is_sheet_back = page_idx % 2 != 0;
    let img_low = if page_low_idx >= page_count {
        // 空白的情况，没有低页
        None
    } else {
        // 获取低页的图像数据
        let reverse_image = is_sheet_back;
        let (page_low_width, page_low_height, page_low_rgba) =
            src_pdf.get_page_image(page_low_idx, reverse_image);
        Some(
            oxidize_pdf::Image::from_rgba_data(page_low_rgba, page_low_width, page_low_height)
                .unwrap(),
        )
    };
    println!("{}, {}", page_low_idx, page_high_idx);
    let img_high = if page_high_idx >= page_count {
        // 空白的情况，没有高页
        None
    } else {
        // 获取高页的图像数据
        let reverse_image = !(is_sheet_back ^ binding_at_middle);
        let (page_high_width, page_high_height, page_high_rgba) =
            src_pdf.get_page_image(page_high_idx, reverse_image);
        let img_high =
            oxidize_pdf::Image::from_rgba_data(page_high_rgba, page_high_width, page_high_height)
                .unwrap();
        Some(img_high)
    };
    let v_1mm_to_pt = 72.0 / 25.4;
    // 3mm
    let margin = 3.0 * v_1mm_to_pt;
    let mut new_page = Page::a4();
    let (w, h) = (new_page.width(), new_page.height());
    let half_h = h / 2.0;
    let half_w = w / 2.0;
    // 等比缩放
    // let margin_tb = margin * h / w / 2.0; ==> margin * h/2.0 / w;
    let margin_tb = margin * half_h / w;

    let (img_bottom, img_bottom_idx, img_top, img_top_idx) = if binding_rule.binding_at_middle {
        (img_low, page_low_idx, img_high, page_high_idx)
    } else {
        (img_high, page_high_idx, img_low, page_low_idx)
    };

    let v_12mm = 12.0 * v_1mm_to_pt;
    if let Some(img) = img_bottom {
        new_page.add_image(format!("{}", img_bottom_idx), img);
        new_page
            .draw_image(
                format!("{}", img_bottom_idx).as_str(),
                margin,
                margin_tb,
                w - v_12mm,
                half_h - margin_tb,
            )
            .unwrap();
    }
    if let Some(img) = img_top {
        new_page.add_image(format!("{}", img_top_idx), img);
        new_page
            .draw_image(
                format!("{}", img_top_idx).as_str(),
                margin,
                half_h + margin_tb,
                w - v_12mm,
                half_h - margin_tb,
            )
            .unwrap();
    }

    // 间隔1.2mm
    let dot_space = v_12mm;
    let padding = 6.0 * v_1mm_to_pt;
    let ((start_x, start_y), (to_x, to_y)) = if is_sheet_back {
        ((padding, half_h), (w, half_h))
    } else {
        ((w - padding, half_h), (0.0, half_h))
    };
    new_page
        .graphics()
        .set_stroke_color(Color::Gray(0.3))
        .move_to(start_x, start_y)
        .line_to(to_x, to_y)
        .set_line_dash_pattern(LineDashPattern::dotted(1.0, dot_space))
        .stroke();
    if !is_sheet_back {
        let _ = new_page
            .text()
            .set_font(Font::TimesRoman, 6.0)
            .at(half_w - 9.0 * v_1mm_to_pt, half_h)
            .write(format!("^- {} -^", booklet_num).as_str());
    }
    return Some(new_page);
}