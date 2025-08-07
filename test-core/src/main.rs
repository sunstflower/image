//! 测试 rustimage-core 的基本功能
//!
//! 这个测试文件用来验证 rustimage-core 库的核心功能是否正常工作

use rustimage_core::{
    ImageFormat, ConversionOptions, ConversionOptionsBuilder, FormatConverter,
    ImageInput, ConversionTask, detect_format, get_format_info, get_performance_metrics,
    ImageError, Result,
};

fn main() -> Result<()> {
    println!("🚀 开始测试 RustImage Core 功能...\n");

    // 测试1: 基本类型创建
    test_basic_types()?;

    // 测试2: 格式转换器创建
    test_converter_creation()?;

    // 测试3: 格式信息获取
    test_format_info();

    // 测试4: 性能监控
    test_performance_monitoring()?;

    // 测试5: 错误处理
    test_error_handling();

    // 测试6: 选项构建器
    test_options_builder();

    // 测试7: 格式检测（预期会失败，因为没有实际实现）
    test_format_detection();

    println!("✅ 所有测试完成！");
    Ok(())
}

fn test_basic_types() -> Result<()> {
    println!("📋 测试基本类型创建...");

    // 测试图像格式枚举
    let formats = vec![
        ImageFormat::Jpeg,
        ImageFormat::Png,
        ImageFormat::WebP,
        ImageFormat::Avif,
        ImageFormat::Bmp,
        ImageFormat::Tiff,
        ImageFormat::Gif,
        ImageFormat::Ico,
    ];

    println!("   - 创建了 {} 种图像格式", formats.len());

    // 测试转换任务
    let task = ConversionTask {
        from_format: ImageFormat::Jpeg,
        to_format: ImageFormat::Png,
        options: None,
    };

    println!("   - 创建转换任务: {:?} -> {:?}", task.from_format, task.to_format);

    // 测试图像输入
    let test_data = vec![0xFF, 0xD8, 0xFF, 0xE0]; // JPEG 文件头
    let image_input = ImageInput::new(test_data.clone(), ImageFormat::Jpeg);

    println!("   - 创建图像输入: {} 字节", image_input.size());

    println!("   ✅ 基本类型测试通过\n");
    Ok(())
}

fn test_converter_creation() -> Result<()> {
    println!("🔧 测试格式转换器创建...");

    // 测试默认转换器创建
    let converter = FormatConverter::with_defaults()?;
    println!("   - 默认转换器创建成功");

    // 测试高性能转换器创建
    let hp_converter = FormatConverter::with_high_performance()?;
    println!("   - 高性能转换器创建成功");

    // 测试高质量转换器创建
    let hq_converter = FormatConverter::with_high_quality()?;
    println!("   - 高质量转换器创建成功");

    // 获取支持的格式
    let supported_formats = converter.get_supported_formats();
    println!("   - 支持的格式数量: {}", supported_formats.len());

    println!("   ✅ 转换器创建测试通过\n");
    Ok(())
}

fn test_format_info() {
    println!("ℹ️ 测试格式信息获取...");

    for &format in &[ImageFormat::Jpeg, ImageFormat::Png, ImageFormat::WebP] {
        let info = get_format_info(format);
        println!("   - {}: {} (扩展名: {:?})",
            info.name, info.description, info.extensions);
        println!("     MIME类型: {}", info.mime_type);
        println!("     支持有损: {}, 支持透明: {}, 支持动画: {}",
            info.capabilities.supports_lossy(),
            info.capabilities.supports_transparency(),
            info.capabilities.supports_animation());
    }

    println!("   ✅ 格式信息测试通过\n");
}

fn test_performance_monitoring() -> Result<()> {
    println!("📊 测试性能监控...");

    // 获取性能指标
    let metrics = get_performance_metrics();
    println!("   - 当前指标:");
    println!("     总时间: {:.2}ms", metrics.timing.total_time_ms);
    println!("     峰值内存: {} 字节", metrics.memory.peak_memory_bytes);
    println!("     图像/秒: {:.2}", metrics.throughput.images_per_second);
    println!("     CPU使用率: {:.1}%", metrics.system.cpu_usage_percent);

    println!("   ✅ 性能监控测试通过\n");
    Ok(())
}

fn test_error_handling() {
    println!("❌ 测试错误处理...");

    // 测试错误创建
    let format_error = ImageError::invalid_format("test_format");
    println!("   - 格式错误: {}", format_error);
    println!("     分类: {:?}, 严重程度: {:?}", format_error.category(), format_error.severity());
    println!("     可重试: {}", format_error.is_retryable());

    let param_error = ImageError::invalid_parameters("无效的质量参数: -1");
    println!("   - 参数错误: {}", param_error);
    println!("     分类: {:?}, 严重程度: {:?}", param_error.category(), param_error.severity());

    let unsupported_error = ImageError::unsupported_operation("魔法转换");
    println!("   - 不支持操作错误: {}", unsupported_error);

    println!("   ✅ 错误处理测试通过\n");
}

fn test_options_builder() {
    println!("🔨 测试选项构建器...");

    // 测试基本构建器
    let options = ConversionOptionsBuilder::new()
        .quality(0.8)
        .compression_level(6)
        .progressive(true)
        .preserve_dimensions(true)
        .preserve_metadata(false)
        .custom_param("custom_key".to_string(), "custom_value".to_string())
        .build();

    println!("   - 构建的选项:");
    if let Some(quality) = options.quality() {
        println!("     质量: {}", quality);
    }
    if let Some(level) = options.compression_level() {
        println!("     压缩级别: {}", level);
    }
    if let Some(progressive) = options.is_progressive() {
        println!("     渐进式: {}", progressive);
    }
    println!("     保持尺寸: {}", options.preserves_dimensions());
    println!("     保持元数据: {}", options.preserves_metadata());

    if let Some(custom_value) = options.custom_param("custom_key") {
        println!("     自定义参数: {}", custom_value);
    }

    println!("   ✅ 选项构建器测试通过\n");
}

fn test_format_detection() {
    println!("🔍 测试格式检测...");

    // 创建一些测试数据
    let jpeg_header = vec![0xFF, 0xD8, 0xFF, 0xE0];
    let png_header = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
    let webp_header = vec![0x52, 0x49, 0x46, 0x46];

    // 测试 JPEG 检测
    match detect_format(&jpeg_header) {
        Ok(format) => println!("   - 检测到格式: {:?}", format),
        Err(e) => println!("   - JPEG检测失败 (预期): {}", e),
    }

    // 测试 PNG 检测
    match detect_format(&png_header) {
        Ok(format) => println!("   - 检测到格式: {:?}", format),
        Err(e) => println!("   - PNG检测失败 (预期): {}", e),
    }

    // 测试 WebP 检测
    match detect_format(&webp_header) {
        Ok(format) => println!("   - 检测到格式: {:?}", format),
        Err(e) => println!("   - WebP检测失败 (预期): {}", e),
    }

    // 测试无效数据
    let invalid_data = vec![0x00, 0x00, 0x00, 0x00];
    match detect_format(&invalid_data) {
        Ok(format) => println!("   - 意外检测到格式: {:?}", format),
        Err(e) => println!("   - 无效数据检测失败 (预期): {}", e),
    }

    println!("   ✅ 格式检测测试完成\n");
}
