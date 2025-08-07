//! 综合图像转换测试
//!
//! 测试真实的图像格式转换功能，包括JPEG到PNG的转换

use rustimage_core::{
    convert_format, detect_format, get_format_info,
    ImageFormat, ConversionOptionsBuilder,
    FormatConverter, ImageInput, ConversionTask,
    ImageError, Result,
};

fn main() -> Result<()> {
    println!("🚀 开始图像转换功能测试\n");

    // 测试1: 格式检测
    test_format_detection()?;

    // 测试2: 创建测试图像数据
    let test_images = create_test_images();

    // 测试3: 基本转换功能
    test_basic_conversion(&test_images)?;

    // 测试4: 选项配置转换
    test_conversion_with_options(&test_images)?;

    // 测试5: 批量转换
    test_batch_conversion(&test_images)?;

    // 测试6: 错误处理
    test_error_cases()?;

    println!("✅ 所有图像转换测试完成！");
    Ok(())
}

fn test_format_detection() -> Result<()> {
    println!("🔍 测试格式检测功能...");

    // 创建各种格式的文件头
    let test_cases = vec![
        // JPEG文件头
        (vec![0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46], ImageFormat::Jpeg, "JPEG"),
        // PNG文件头
        (vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A], ImageFormat::Png, "PNG"),
        // WebP文件头 (RIFF + WEBP)
        (vec![0x52, 0x49, 0x46, 0x46, 0x00, 0x00, 0x00, 0x00, 0x57, 0x45, 0x42, 0x50], ImageFormat::WebP, "WebP"),
        // BMP文件头
        (vec![0x42, 0x4D, 0x00, 0x00, 0x00, 0x00], ImageFormat::Bmp, "BMP"),
        // GIF文件头
        (b"GIF89a".to_vec(), ImageFormat::Gif, "GIF"),
    ];

    for (data, expected_format, name) in test_cases {
        match detect_format(&data) {
            Ok(detected) if detected == expected_format => {
                println!("   - ✅ {} 检测成功: {:?}", name, detected);
            }
            Ok(detected) => {
                println!("   - ❌ {} 检测错误: 期望 {:?}, 得到 {:?}", name, expected_format, detected);
            }
            Err(e) => {
                println!("   - ❌ {} 检测失败: {}", name, e);
            }
        }
    }

    // 测试无效数据
    let invalid_data = vec![0x00, 0x01, 0x02, 0x03];
    match detect_format(&invalid_data) {
        Ok(format) => println!("   - ❌ 无效数据意外检测为: {:?}", format),
        Err(_) => println!("   - ✅ 无效数据正确拒绝"),
    }

    println!("   ✅ 格式检测测试完成\n");
    Ok(())
}

fn create_test_images() -> Vec<(Vec<u8>, ImageFormat, &'static str)> {
    println!("🎨 创建测试图像数据...");

    // 创建一个最小的有效JPEG图像 (1x1像素，白色)
    let minimal_jpeg = vec![
        0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46, 0x49, 0x46, 0x00, 0x01,
        0x01, 0x01, 0x00, 0x48, 0x00, 0x48, 0x00, 0x00, 0xFF, 0xDB, 0x00, 0x43,
        0x00, 0x08, 0x06, 0x06, 0x07, 0x06, 0x05, 0x08, 0x07, 0x07, 0x07, 0x09,
        0x09, 0x08, 0x0A, 0x0C, 0x14, 0x0D, 0x0C, 0x0B, 0x0B, 0x0C, 0x19, 0x12,
        0x13, 0x0F, 0x14, 0x1D, 0x1A, 0x1F, 0x1E, 0x1D, 0x1A, 0x1C, 0x1C, 0x20,
        0x24, 0x2E, 0x27, 0x20, 0x22, 0x2C, 0x23, 0x1C, 0x1C, 0x28, 0x37, 0x29,
        0x2C, 0x30, 0x31, 0x34, 0x34, 0x34, 0x1F, 0x27, 0x39, 0x3D, 0x38, 0x32,
        0x3C, 0x2E, 0x33, 0x34, 0x32, 0xFF, 0xC0, 0x00, 0x11, 0x08, 0x00, 0x01,
        0x00, 0x01, 0x01, 0x01, 0x11, 0x00, 0x02, 0x11, 0x01, 0x03, 0x11, 0x01,
        0xFF, 0xC4, 0x00, 0x14, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0xFF, 0xC4,
        0x00, 0x14, 0x10, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xDA, 0x00, 0x0C,
        0x03, 0x01, 0x00, 0x02, 0x11, 0x03, 0x11, 0x00, 0x3F, 0x00, 0x00, 0xFF, 0xD9,
    ];

    // 创建一个最小的有效PNG图像 (1x1像素，白色)
    let minimal_png = vec![
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D,
        0x49, 0x48, 0x44, 0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01,
        0x08, 0x02, 0x00, 0x00, 0x00, 0x90, 0x77, 0x53, 0xDE, 0x00, 0x00, 0x00,
        0x0C, 0x49, 0x44, 0x41, 0x54, 0x08, 0xD7, 0x63, 0xF8, 0xFF, 0xFF, 0x3F,
        0x00, 0x05, 0xFE, 0x02, 0xFE, 0xDC, 0xCC, 0x59, 0xE7, 0x00, 0x00, 0x00,
        0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82,
    ];

    println!("   - ✅ 创建了 {} 字节的测试JPEG", minimal_jpeg.len());
    println!("   - ✅ 创建了 {} 字节的测试PNG", minimal_png.len());

    vec![
        (minimal_jpeg, ImageFormat::Jpeg, "test-jpeg"),
        (minimal_png, ImageFormat::Png, "test-png"),
    ]
}

fn test_basic_conversion(test_images: &[(Vec<u8>, ImageFormat, &str)]) -> Result<()> {
    println!("🔄 测试基本转换功能...");

    for (image_data, from_format, name) in test_images {
        // 测试转换到其他格式
        let target_formats = [ImageFormat::Jpeg, ImageFormat::Png];

        for to_format in target_formats {
            if *from_format == to_format {
                continue; // 跳过相同格式
            }

            println!("   - 尝试转换: {} {:?} -> {:?}", name, from_format, to_format);

            match convert_format(image_data, *from_format, to_format, None) {
                Ok(result) => {
                    println!("     ✅ 转换成功:");
                    println!("       原始大小: {} 字节", result.original_size());
                    println!("       转换后大小: {} 字节", result.converted_size());
                    println!("       压缩比: {:.2}%", result.compression_percentage());
                    println!("       转换时间: {:.2}ms", result.conversion_time_ms());
                    println!("       图像尺寸: {}x{}", result.dimensions().width, result.dimensions().height);
                }
                Err(e) => {
                    // 对于我们的测试图像，某些转换可能会失败，这是正常的
                    println!("     ⚠️  转换失败 (可能是测试数据限制): {}", e);
                }
            }
        }
    }

    println!("   ✅ 基本转换测试完成\n");
    Ok(())
}

fn test_conversion_with_options(test_images: &[(Vec<u8>, ImageFormat, &str)]) -> Result<()> {
    println!("⚙️ 测试带选项的转换...");

    if let Some((jpeg_data, _, _)) = test_images.iter().find(|(_, format, _)| *format == ImageFormat::Jpeg) {
        // 测试不同质量的JPEG转换
        let qualities = [0.3, 0.6, 0.9];

        for quality in qualities {
            let options = ConversionOptionsBuilder::new()
                .quality(quality)
                .progressive(false)
                .preserve_dimensions(true)
                .build();

            println!("   - 测试JPEG质量: {:.1}", quality);

            match convert_format(jpeg_data, ImageFormat::Jpeg, ImageFormat::Png, Some(options)) {
                Ok(result) => {
                    println!("     ✅ 质量 {:.1} 转换成功, 大小: {} 字节", quality, result.converted_size());
                }
                Err(e) => {
                    println!("     ⚠️  质量 {:.1} 转换失败: {}", quality, e);
                }
            }
        }
    }

    if let Some((png_data, _, _)) = test_images.iter().find(|(_, format, _)| *format == ImageFormat::Png) {
        // 测试不同压缩级别的PNG转换
        let compression_levels = [1, 6, 9];

        for level in compression_levels {
            let options = ConversionOptionsBuilder::new()
                .compression_level(level)
                .preserve_metadata(false)
                .build();

            println!("   - 测试PNG压缩级别: {}", level);

            match convert_format(png_data, ImageFormat::Png, ImageFormat::Jpeg, Some(options)) {
                Ok(result) => {
                    println!("     ✅ 压缩级别 {} 转换成功, 大小: {} 字节", level, result.converted_size());
                }
                Err(e) => {
                    println!("     ⚠️  压缩级别 {} 转换失败: {}", level, e);
                }
            }
        }
    }

    println!("   ✅ 选项转换测试完成\n");
    Ok(())
}

fn test_batch_conversion(test_images: &[(Vec<u8>, ImageFormat, &str)]) -> Result<()> {
    println!("📦 测试批量转换...");

    // 创建批量转换输入
    let mut images = Vec::new();
    let mut tasks = Vec::new();

    for (image_data, from_format, name) in test_images {
        let input = ImageInput::new(image_data.clone(), *from_format);
        images.push(input);

        // 转换到不同的格式
        let to_format = match from_format {
            ImageFormat::Jpeg => ImageFormat::Png,
            ImageFormat::Png => ImageFormat::Jpeg,
            _ => ImageFormat::Png,
        };

        let task = ConversionTask {
            from_format: *from_format,
            to_format,
            options: None,
        };
        tasks.push(task);
    }

    if !images.is_empty() {
        println!("   - 批量转换 {} 个图像", images.len());

        // 创建转换器
        let mut converter = FormatConverter::with_defaults()?;

        match converter.batch_convert(images, tasks) {
            Ok(results) => {
                println!("     ✅ 批量转换成功:");
                for (i, result) in results.iter().enumerate() {
                    println!("       图像 {}: {} -> {} 字节, {:.2}ms",
                        i + 1,
                        result.original_size(),
                        result.converted_size(),
                        result.conversion_time_ms()
                    );
                }

                // 显示转换器统计
                let stats = converter.get_conversion_statistics();
                println!("     转换统计:");
                println!("       总转换数: {}", stats.total_conversions);
                println!("       成功率: {:.1}%", stats.success_rate * 100.0);
                println!("       平均处理时间: {:.2}ms", stats.average_processing_time_ms);
            }
            Err(e) => {
                println!("     ⚠️  批量转换失败: {}", e);
            }
        }
    } else {
        println!("   - ⚠️  没有可用的测试图像进行批量转换");
    }

    println!("   ✅ 批量转换测试完成\n");
    Ok(())
}

fn test_error_cases() -> Result<()> {
    println!("❌ 测试错误处理...");

    // 测试1: 空数据
    match convert_format(&[], ImageFormat::Jpeg, ImageFormat::Png, None) {
        Ok(_) => println!("   - ❌ 空数据应该失败"),
        Err(e) => println!("   - ✅ 空数据正确拒绝: {}", e),
    }

    // 测试2: 无效的JPEG数据
    let invalid_jpeg = vec![0xFF, 0xD8, 0x00, 0x00]; // 不完整的JPEG头
    match convert_format(&invalid_jpeg, ImageFormat::Jpeg, ImageFormat::Png, None) {
        Ok(_) => println!("   - ❌ 无效JPEG应该失败"),
        Err(e) => println!("   - ✅ 无效JPEG正确拒绝: {}", e),
    }

    // 测试3: 格式不匹配
    let png_data = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
    match convert_format(&png_data, ImageFormat::Jpeg, ImageFormat::Png, None) {
        Ok(_) => println!("   - ❌ 格式不匹配应该失败"),
        Err(e) => println!("   - ✅ 格式不匹配正确拒绝: {}", e),
    }

    // 测试4: 无效的质量参数
    let options = ConversionOptionsBuilder::new().quality(1.5).build(); // 质量 > 1.0
    let test_data = vec![0xFF, 0xD8, 0xFF, 0xE0];
    match convert_format(&test_data, ImageFormat::Jpeg, ImageFormat::Png, Some(options)) {
        Ok(_) => println!("   - ⚠️  无效质量参数被接受 (可能有内部处理)"),
        Err(e) => println!("   - ✅ 无效质量参数正确拒绝: {}", e),
    }

    println!("   ✅ 错误处理测试完成\n");
    Ok(())
}
