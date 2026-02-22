use crate::booklet::BindingRule;
use crate::booklet::BookletConfig;
use crate::pdf_render::PdfDocumentHolder;
use oxidize_pdf::Color;
use oxidize_pdf::Document;
use oxidize_pdf::Font;
use oxidize_pdf::Page;
use oxidize_pdf::graphics::LineDashPattern;
use pdfium_render::prelude::PdfDocumentMetadataTagType;

/// 创建小册子页面
///
/// # 参数
/// * `pdf_doc` - PDF文档持有者，用于获取页面图像
pub fn test_create_booklet(src_pdf: &PdfDocumentHolder) {
    let mut doc = Document::new();
    // doc.set_title("My PDF");
    // doc.set_producer("BCFBH");
    doc.set_creator("BCFBH");
    // let img = oxidize_pdf::Image::from_jpeg_file("r-0.jpg").unwrap();
    let idx = 0;
    let (width, height, page_rgba) = src_pdf.get_page_image(idx, false);

    let img = oxidize_pdf::Image::from_rgba_data(page_rgba, width, height).unwrap();
    // 3mm
    let margin = 3.0 * 72.0 / 25.4;
    // Create a page
    let mut page1 = Page::a4();
    let (w, h) = dbg!((page1.width(), page1.height()));
    let margin_tb = margin * h / w / 2.0;
    page1.add_image(format!("{}", idx), img);
    page1
        .draw_image(
            format!("{}", idx).as_str(),
            margin,
            h / 2.0 - margin_tb,
            w - margin * 4.0,
            h / 2.0 - margin_tb,
        )
        .unwrap();
    page1
        .graphics()
        .set_stroke_color(Color::Gray(0.3))
        .move_to(0.0, h / 2.0)
        .line_to(w, h / 2.0)
        .set_line_dash_pattern(LineDashPattern::dotted(1.0, margin * 10.0))
        .stroke();

    doc.add_page(page1);
    doc.save("0211.pdf").unwrap();
}

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
    page_start_idx: u16,
    page_end_idx: u16,
    booklet_num: u16,
    binding_rule: &BindingRule,
) -> Option<Page> {
    let page_low_idx = page_idx;
    let mut page_high_idx = page_end_idx - page_idx + page_start_idx -1;
    let binding_at_middle = binding_rule.binding_at_middle;
    if page_low_idx >= page_high_idx {
        // 本册结束了
        return None;
    }
    if !binding_at_middle {
        page_high_idx = (page_end_idx - page_start_idx) / 2 + page_idx;
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
    // 3mm
    let margin = 3.0 * 72.0 / 25.4;
    // Create a page
    let mut page1 = Page::a4();
    let (w, h) = (page1.width(), page1.height());
    let margin_tb = margin * h / w / 2.0;

    let (img_bottom, img_bottom_idx, img_top, img_top_idx) = if binding_rule.binding_at_middle {
        (img_low, page_low_idx, img_high, page_high_idx)
    } else {
        (img_high, page_high_idx, img_low, page_low_idx)
    };

    if let Some(img) = img_bottom {
        page1.add_image(format!("{}", img_bottom_idx), img);
        page1
            .draw_image(
                format!("{}", img_bottom_idx).as_str(),
                margin,
                margin_tb,
                w - margin * 4.0,
                h / 2.0 - margin_tb,
            )
            .unwrap();
    }
    if let Some(img) = img_top {
        page1.add_image(format!("{}", img_top_idx), img);
        page1
            .draw_image(
                format!("{}", img_top_idx).as_str(),
                margin,
                h / 2.0 + margin_tb,
                w - margin * 4.0,
                h / 2.0 - margin_tb,
            )
            .unwrap();
    }

    let dot_space = margin * 5.0;
    let ((start_x, start_y), (to_x, to_y)) = if is_sheet_back {
        ((dot_space, h / 2.0), (w, h / 2.0))
    } else {
        ((w - dot_space, h / 2.0), (0.0, h / 2.0))
    };
    page1
        .graphics()
        .set_stroke_color(Color::Gray(0.3))
        .move_to(start_x, start_y)
        .line_to(to_x, to_y)
        .set_line_dash_pattern(LineDashPattern::dotted(1.0, dot_space))
        .stroke();
    if !is_sheet_back {
        let _ = page1
            .text()
            .set_font(Font::TimesRoman, 6.0)
            .at(w / 2.0 + margin, h / 2.0)
            .write(format!("^- {} -^", booklet_num).as_str());
    }

    // 在页面垂直方向中间绘制虚线
    // 使用一系列短线段模拟虚线效果
    // draw_vertical_dashed_line(&mut page1, w / 2.0, margin, h - margin, 5.0, 5.0);

    return Some(page1);
}

/// 绘制垂直方向虚线
///
/// 使用 move_to 和 line_to 方法绘制一系列短线段来模拟虚线效果
///
/// # 参数
/// * `page` - 页面对象
/// * `x` - 虚线的x坐标（垂直线位置）
/// * `y_start` - 起始y坐标
/// * `y_end` - 结束y坐标
/// * `dash_length` - 每段虚线长度
/// * `gap_length` - 虚线间隔长度
fn draw_vertical_dashed_line(
    page: &mut Page,
    x: f64,
    y_start: f64,
    y_end: f64,
    dash_length: f64,
    gap_length: f64,
) {
    let graphics = page.graphics();
    let mut current_y = y_start;

    while current_y < y_end {
        let segment_end = (current_y + dash_length).min(y_end);
        // 使用路径绘制线段
        graphics.move_to(x, current_y);
        graphics.line_to(x, segment_end);
        graphics.stroke();
        current_y += dash_length + gap_length;
    }
}

/// 绘制水平方向虚线
///
/// 使用 move_to 和 line_to 方法绘制一系列短线段来模拟虚线效果
///
/// # 参数
/// * `page` - 页面对象
/// * `y` - 虚线的y坐标（水平线位置）
/// * `x_start` - 起始x坐标
/// * `x_end` - 结束x坐标
/// * `dash_length` - 每段虚线长度
/// * `gap_length` - 虚线间隔长度
fn draw_horizontal_dashed_line(
    page: &mut Page,
    y: f64,
    x_start: f64,
    x_end: f64,
    dash_length: f64,
    gap_length: f64,
) {
    let graphics = page.graphics();
    let mut current_x = x_start;

    while current_x < x_end {
        let segment_end = (current_x + dash_length).min(x_end);
        // 使用路径绘制线段
        graphics.move_to(current_x, y);
        graphics.line_to(segment_end, y);
        graphics.stroke();
        current_x += dash_length + gap_length;
    }
}
