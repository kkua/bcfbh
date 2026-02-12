use std::{cell::OnceCell, path::PathBuf};

use oxidize_pdf::operations::rotate;
use pdfium_render::prelude::*;

// static PDFIUM_LIB: Pdfium = Pdfium::new(
//             Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./ffi/"))
//                 .expect("无法绑定到pdfium库")
//         );

/// PDF文档持有者，同时保存Pdfium和PdfDocument以确保生命周期
pub struct PdfDocumentHolder<'a> {
    document: PdfDocument<'a>,
}

impl<'a> PdfDocumentHolder<'a> {
    /// 创建新的PDF文档持有者
    ///
    /// # 参数
    /// * `path` - PDF文件路径
    /// * `password` - 可选的密码
    ///
    /// # Panics
    /// 如果无法绑定到pdfium库或无法读取PDF文件，会触发panic
    pub fn new(pdfium: &'a Pdfium, path: &PathBuf, password: Option<&'a str>) -> Self {
        // 先加载文档
        let document = pdfium
            .load_pdf_from_file(path, password)
            .expect("无法读取PDF文件");

        // 将document转换为'static生命周期
        // let document: PdfDocument<'static> = unsafe { std::mem::transmute(document) };

        Self { document }
    }

    /// 获取页面对象的引用
    pub fn pages(&self) -> &PdfPages<'_> {
        self.document.pages()
    }

    pub fn metadata(&self) -> &PdfMetadata<'_> {
        self.document.metadata()
    }
    /// 获取指定页面的图像数据
    ///
    /// # 参数
    /// * `page_idx` - 页面索引（从0开始）
    ///
    /// # 返回
    /// 返回 (width, height, rgba_bytes) 元组
    pub fn get_page_image(&self, page_idx: u16, reverse_image: bool) -> (u32, u32, Vec<u8>) {
        let rotate = if reverse_image {
            //旋转270°
            PdfPageRenderRotation::Degrees270
        } else {
            // 旋转90°
            PdfPageRenderRotation::Degrees90
        };
        let page = self.pages().get(page_idx).unwrap();
        let render_config = PdfRenderConfig::new()
            .set_target_width(2000)
            .set_maximum_height(2000)
            .rotate(rotate, true);
        let bitmap = page.render_with_config(&render_config).unwrap();
        let width = bitmap.width() as u32;
        let height = bitmap.height() as u32;
        let rgba = bitmap.as_rgba_bytes();
        (width, height, rgba)
    }

    /// 获取PDF总页数
    pub fn get_page_count(&self) -> u16 {
        self.pages().len()
    }
}

/// 获取指定页面的图像数据（独立函数版本）
///
/// # 参数
/// * `pages` - PDF页面对象
/// * `page_idx` - 页面索引（从0开始）
///
/// # 返回
/// 返回RGBA格式的图像字节数据
pub fn get_page_image(pages: &PdfPages, page_idx: u16) -> Vec<u8> {
    let page = pages.get(page_idx).unwrap();
    let render_config = PdfRenderConfig::new()
        .set_target_width(2000)
        .set_maximum_height(2000)
        .rotate(PdfPageRenderRotation::Degrees90, true);
    page.render_with_config(&render_config)
        .unwrap()
        .as_rgba_bytes()
}

pub fn init_pdfium() -> Pdfium {
    Pdfium::new(
        Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./ffi/"))
            .expect("无法绑定到pdfium库"),
    )
}
