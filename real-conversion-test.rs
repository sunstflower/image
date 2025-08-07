//! 真实图像转换测试
//!
//! 使用程序生成的有效图像进行转换测试

use rustimage_core::{
    convert_format, detect_format,
    ImageFormat, ConversionOptionsBuilder,
    FormatConverter, ImageInput, ConversionTask,
    Result,
};

fn main() -> Result<()> {
    println!("🚀 开始真实图像转换测试\n");

    // 测试1: 使用程序生成的图像进行转换
    test_generated_image_conversion()?;

    // 测试2: 测试转换器功能
    test_converter_functionality()?;

    // 测试3: 测试不同选项
    test_conversion_options()?;

    println!("✅ 所有真实转换测试完成！");
    Ok(())
}

fn test_generated_image_conversion() -> Result<()> {
    println!("🎨 测试程序生成图像转换...");

    // 创建一个简单的2x2 RGBA图像
    let width = 2u32;
    let height = 2u32;
    let rgba_data: Vec<u8> = vec![
        255, 0, 0, 255,    // 红色像素
        0, 255, 0, 255,    // 绿色像素
        0, 0, 255, 255,    // 蓝色像素
        255, 255, 255, 255 // 白色像素
    ];

    // 使用image crate创建和编码图像
    println!("   - 创建 {}x{} 测试图像", width, height);

    // 创建PNG格式的图像数据
    let png_data = {
        use image::{ImageBuffer, Rgba, ImageEncoder};
        use std::io::Cursor;

        let img = ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, rgba_data.clone())
            .expect("Failed to create image buffer");

        let mut png_bytes = Vec::new();
        let encoder = image::codecs::png::PngEncoder::new(&mut png_bytes);
        encoder.write_image(
            &rgba_data,
            width,
            height,
            image::ColorType::Rgba8,
        ).expect("Failed to encode PNG");

        png_bytes
    };

    println!("   - ✅ 生成了 {} 字节的PNG图像", png_data.len());

    // 测试格式检测
    match detect_format(&png_data) {
        Ok(detected) => {
            println!("   - ✅ 格式检测成功: {:?}", detected);
            assert_eq!(detected, ImageFormat::Png);
        }
        Err(e) => {
            println!("   - ❌ 格式检测失败: {}", e);
            return Err(e);
        }
    }

    // 测试PNG到JPEG转换
    println!("   - 尝试PNG -> JPEG转换");
    match convert_format(&png_data, ImageFormat::Png, ImageFormat::Jpeg, None) {
        Ok(result) => {
            println!("     ✅ 转换成功!");
            println!("       原始大小: {} 字节", result.original_size());
            println!("       转换后大小: {} 字节", result.converted_size());
            println!("       压缩比: {:.2}%", result.compression_percentage());
            println!("       转换时间: {:.2}ms", result.conversion_time_ms());
            println!("       图像尺寸: {}x{}", result.dimensions().width, result.dimensions().height);

            // 验证转换后的数据格式
            let jpeg_data = result.data();
            match detect_format(jpeg_data) {
                Ok(ImageFormat::Jpeg) => println!("     ✅ 输出格式验证正确"),
                Ok(other) => println!("     ❌ 输出格式错误: {:?}", other),
                Err(e) => println!("     ❌ 输出格式验证失败: {}", e),
            }
        }
        Err(e) => {
            println!("     ❌ 转换失败: {}", e);
        }
    }

    // 创建JPEG格式的图像数据进行反向测试
    let jpeg_data = {
        use image::{ImageBuffer, Rgba, ImageEncoder};

        let img = ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, rgba_data)
            .expect("Failed to create image buffer");

        let mut jpeg_bytes = Vec::new();
        let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut jpeg_bytes, 90);
        encoder.write_image(
            img.as_raw(),
            width,
            height,
            image::ColorType::Rgba8,
        ).expect("Failed to encode JPEG");

        jpeg_bytes
    };

    println!("   - ✅ 生成了 {} 字节的JPEG图像", jpeg_data.len());

    // 测试JPEG到PNG转换
    println!("   - 尝试JPEG -> PNG转换");
    match convert_format(&jpeg_data, ImageFormat::Jpeg, ImageFormat::Png, None) {
        Ok(result) => {
            println!("     ✅ 转换成功!");
            println!("       原始大小: {} 字节", result.original_size());
            println!("       转换后大小: {} 字节", result.converted_size());
            println!("       压缩比: {:.2}%", result.compression_percentage());
            println!("       转换时间: {:.2}ms", result.conversion_time_ms());
        }
        Err(e) => {
            println!("     ❌ 转换失败: {}", e);
        }
    }

    println!("   ✅ 程序生成图像转换测试完成\n");
    Ok(())
}

fn test_converter_functionality() -> Result<()> {
    println!("🔧 测试转换器功能...");

    let mut converter = FormatConverter::with_defaults()?;
    println!("   - ✅ 默认转换器创建成功");

    // 创建测试图像
    let test_image = create_test_image(4, 4);
    println!("   - ✅ 创建了4x4测试图像: {} 字节", test_image.len());

    // 测试支持的格式
    let supported = converter.get_supported_formats();
    println!("   - 支持的格式: {:?}", supported);

    // 测试格式检测
    match converter.detect_format(&test_image) {
        Ok(format) => println!("   - ✅ 检测到格式: {:?}", format),
        Err(e) => println!("   - ❌ 格式检测失败: {}", e),
    }

    // 测试转换统计
    let stats = converter.get_conversion_statistics();
    println!("   - 转换统计:");
    println!("     总转换数: {}", stats.total_conversions);
    println!("     成功率: {:.1}%", stats.success_rate * 100.0);

    println!("   ✅ 转换器功能测试完成\n");
    Ok(())
}

fn test_conversion_options() -> Result<()> {
    println!("⚙️ 测试转换选项...");

    let test_image = create_test_image(8, 8);

    // 测试不同质量设置
    let qualities = [0.3, 0.6, 0.9];

    for &quality in &qualities {
        let options = ConversionOptionsBuilder::new()
            .quality(quality)
            .preserve_dimensions(true)
            .preserve_metadata(false)
            .build();

        println!("   - 测试质量设置: {:.1}", quality);

        match convert_format(&test_image, ImageFormat::Png, ImageFormat::Jpeg, Some(options)) {
            Ok(result) => {
                println!("     ✅ 质量 {:.1}: {} -> {} 字节 ({:.1}% 压缩)",
                    quality,
                    result.original_size(),
                    result.converted_size(),
                    result.compression_percentage()
                );
            }
            Err(e) => {
                println!("     ❌ 质量 {:.1} 转换失败: {}", quality, e);
            }
        }
    }

    // 测试构建器功能
    let complex_options = ConversionOptionsBuilder::new()
        .quality(0.8)
        .compression_level(6)
        .progressive(true)
        .preserve_dimensions(true)
        .preserve_color_space(true)
        .preserve_metadata(false)
        .custom_param("test_param".to_string(), "test_value".to_string())
        .build();

    println!("   - 复杂选项配置:");
    if let Some(q) = complex_options.quality() {
        println!("     质量: {}", q);
    }
    if let Some(cl) = complex_options.compression_level() {
        println!("     压缩级别: {}", cl);
    }
    println!("     保持尺寸: {}", complex_options.preserves_dimensions());
    println!("     保持色彩空间: {}", complex_options.preserves_color_space());
    println!("     保持元数据: {}", complex_options.preserves_metadata());

    if let Some(custom_val) = complex_options.custom_param("test_param") {
        println!("     自定义参数: {}", custom_val);
    }

    println!("   ✅ 转换选项测试完成\n");
    Ok(())
}

fn create_test_image(width: u32, height: u32) -> Vec<u8> {
    // 创建一个渐变图像
    let mut rgba_data = Vec::with_capacity((width * height * 4) as usize);

    for y in 0..height {
        for x in 0..width {
            let r = ((x as f32 / width as f32) * 255.0) as u8;
            let g = ((y as f32 / height as f32) * 255.0) as u8;
            let b = ((((x + y) as f32) / ((width + height) as f32)) * 255.0) as u8;
            let a = 255u8;

            rgba_data.extend_from_slice(&[r, g, b, a]);
        }
    }

    // 编码为PNG
    use image::{ImageBuffer, Rgba, ImageEncoder};

    let img = ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, rgba_data)
        .expect("Failed to create image buffer");

    let mut png_bytes = Vec::new();
    let encoder = image::codecs::png::PngEncoder::new(&mut png_bytes);
    encoder.write_image(
        img.as_raw(),
        width,
        height,
        image::ColorType::Rgba8,
    ).expect("Failed to encode PNG");

    png_bytes
}
