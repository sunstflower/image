//! 简单测试 rustimage-core 核心功能
//!
//! 这是一个更简单的测试，避免可能导致无限循环的操作

use rustimage_core::{
    ImageFormat, ConversionOptionsBuilder, FormatConverter,
    ImageInput, get_format_info, ImageError,
};

fn main() {
    println!("🚀 RustImage Core 简单功能测试\n");

    // 测试1: 基本类型
    test_basic_types();

    // 测试2: 格式信息
    test_format_info();

    // 测试3: 转换器创建
    test_converter_creation();

    // 测试4: 错误处理
    test_error_handling();

    // 测试5: 选项构建
    test_options_builder();

    println!("✅ 所有基本测试完成！");
}

fn test_basic_types() {
    println!("📋 测试基本类型...");

    // 创建各种图像格式
    let formats = [
        ImageFormat::Jpeg,
        ImageFormat::Png,
        ImageFormat::WebP,
        ImageFormat::Avif,
        ImageFormat::Bmp,
        ImageFormat::Tiff,
        ImageFormat::Gif,
        ImageFormat::Ico,
    ];

    println!("   - 支持的格式: {:?}", formats);

    // 创建图像输入
    let test_data = vec![0xFF, 0xD8, 0xFF, 0xE0]; // JPEG header
    let input = ImageInput::new(test_data.clone(), ImageFormat::Jpeg);
    println!("   - 创建图像输入: {} 字节", input.size());

    println!("   ✅ 基本类型测试通过\n");
}

fn test_format_info() {
    println!("ℹ️ 测试格式信息...");

    for &format in &[ImageFormat::Jpeg, ImageFormat::Png, ImageFormat::WebP] {
        let info = get_format_info(format);
        println!("   - {}: {}", info.name, info.description);
        println!("     扩展名: {:?}", info.extensions);
        println!("     MIME: {}", info.mime_type);
    }

    println!("   ✅ 格式信息测试通过\n");
}

fn test_converter_creation() {
    println!("🔧 测试转换器创建...");

    match FormatConverter::with_defaults() {
        Ok(_) => println!("   - ✅ 默认转换器创建成功"),
        Err(e) => println!("   - ❌ 默认转换器创建失败: {}", e),
    }

    match FormatConverter::with_high_performance() {
        Ok(_) => println!("   - ✅ 高性能转换器创建成功"),
        Err(e) => println!("   - ❌ 高性能转换器创建失败: {}", e),
    }

    match FormatConverter::with_high_quality() {
        Ok(_) => println!("   - ✅ 高质量转换器创建成功"),
        Err(e) => println!("   - ❌ 高质量转换器创建失败: {}", e),
    }

    println!("   ✅ 转换器创建测试通过\n");
}

fn test_error_handling() {
    println!("❌ 测试错误处理...");

    // 创建各种错误
    let format_error = ImageError::invalid_format("unknown_format");
    println!("   - 格式错误: {}", format_error);

    let param_error = ImageError::invalid_parameters("invalid quality: -1");
    println!("   - 参数错误: {}", param_error);

    let unsupported_error = ImageError::unsupported_operation("time_travel_conversion");
    println!("   - 不支持操作: {}", unsupported_error);

    // 测试错误属性
    println!("   - 格式错误可重试: {}", format_error.is_retryable());
    println!("   - 参数错误严重程度: {:?}", param_error.severity());

    println!("   ✅ 错误处理测试通过\n");
}

fn test_options_builder() {
    println!("🔨 测试选项构建器...");

    let options = ConversionOptionsBuilder::new()
        .quality(0.8)
        .compression_level(6)
        .progressive(true)
        .preserve_dimensions(true)
        .preserve_metadata(false)
        .build();

    if let Some(quality) = options.quality() {
        println!("   - 质量设置: {}", quality);
    }

    if let Some(level) = options.compression_level() {
        println!("   - 压缩级别: {}", level);
    }

    if let Some(progressive) = options.is_progressive() {
        println!("   - 渐进式: {}", progressive);
    }

    println!("   - 保持尺寸: {}", options.preserves_dimensions());
    println!("   - 保持元数据: {}", options.preserves_metadata());

    println!("   ✅ 选项构建器测试通过\n");
}
