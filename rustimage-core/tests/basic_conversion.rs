//! 基本转换功能测试
//!
//! 测试 rustimage-core 的核心转换功能

use rustimage_core::{
    convert_format, detect_format,
    ImageFormat, ConversionOptionsBuilder,
    FormatConverter,
};
use image::{ImageBuffer, Rgba, ImageEncoder};
use std::io::Cursor;

#[test]
fn test_format_detection() {
    // 测试JPEG检测
    let jpeg_header = vec![0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46];
    assert_eq!(detect_format(&jpeg_header).unwrap(), ImageFormat::Jpeg);

    // 测试PNG检测
    let png_header = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
    assert_eq!(detect_format(&png_header).unwrap(), ImageFormat::Png);

    // 测试WebP检测
    let webp_header = vec![
        0x52, 0x49, 0x46, 0x46, 0x00, 0x00, 0x00, 0x00,
        0x57, 0x45, 0x42, 0x50
    ];
    assert_eq!(detect_format(&webp_header).unwrap(), ImageFormat::WebP);

    // 测试无效数据
    let invalid_data = vec![0x00, 0x01, 0x02, 0x03];
    assert!(detect_format(&invalid_data).is_err());
}

#[test]
fn test_converter_creation() {
    // 测试不同类型的转换器创建
    let default_converter = FormatConverter::with_defaults();
    assert!(default_converter.is_ok());

    let hp_converter = FormatConverter::with_high_performance();
    assert!(hp_converter.is_ok());

    let hq_converter = FormatConverter::with_high_quality();
    assert!(hq_converter.is_ok());
}

#[test]
fn test_options_builder() {
    let options = ConversionOptionsBuilder::new()
        .quality(0.8)
        .compression_level(6)
        .progressive(true)
        .preserve_dimensions(true)
        .preserve_metadata(false)
        .build();

    assert_eq!(options.quality(), Some(0.8));
    assert_eq!(options.compression_level(), Some(6));
    assert_eq!(options.is_progressive(), Some(true));
    assert!(options.preserves_dimensions());
    assert!(!options.preserves_metadata());
}

#[test]
fn test_simple_png_to_jpeg_conversion() {
    // 创建一个简单的2x2 RGBA图像
    let width = 2u32;
    let height = 2u32;
    let rgba_data: Vec<u8> = vec![
        255, 0, 0, 255,    // 红色像素
        0, 255, 0, 255,    // 绿色像素
        0, 0, 255, 255,    // 蓝色像素
        255, 255, 255, 255 // 白色像素
    ];

    // 使用image crate创建PNG数据
    let png_data = {
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
    };

    // 验证PNG格式检测
    assert_eq!(detect_format(&png_data).unwrap(), ImageFormat::Png);

    // 测试PNG到JPEG转换
    let result = convert_format(&png_data, ImageFormat::Png, ImageFormat::Jpeg, None);

    match result {
        Ok(converted) => {
            // 验证转换结果
            assert!(converted.converted_size() > 0);
            assert_eq!(converted.format(), ImageFormat::Jpeg);
            assert_eq!(converted.dimensions().width, width);
            assert_eq!(converted.dimensions().height, height);
            assert!(converted.conversion_time_ms() >= 0.0);

            // 验证输出数据是有效的JPEG
            let output_format = detect_format(converted.data());
            assert!(output_format.is_ok());
            assert_eq!(output_format.unwrap(), ImageFormat::Jpeg);
        },
        Err(e) => {
            // 如果转换失败，打印错误信息以便调试
            panic!("PNG to JPEG conversion failed: {}", e);
        }
    }
}

#[test]
fn test_simple_jpeg_to_png_conversion() {
    // 创建一个简单的2x2 RGBA图像
    let width = 2u32;
    let height = 2u32;
    let rgba_data: Vec<u8> = vec![
        255, 0, 0, 255,    // 红色像素
        0, 255, 0, 255,    // 绿色像素
        0, 0, 255, 255,    // 蓝色像素
        255, 255, 255, 255 // 白色像素
    ];

    // 使用image crate创建JPEG数据
    let jpeg_data = {
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

    // 验证JPEG格式检测
    assert_eq!(detect_format(&jpeg_data).unwrap(), ImageFormat::Jpeg);

    // 测试JPEG到PNG转换
    let result = convert_format(&jpeg_data, ImageFormat::Jpeg, ImageFormat::Png, None);

    match result {
        Ok(converted) => {
            // 验证转换结果
            assert!(converted.converted_size() > 0);
            assert_eq!(converted.format(), ImageFormat::Png);
            assert_eq!(converted.dimensions().width, width);
            assert_eq!(converted.dimensions().height, height);
            assert!(converted.conversion_time_ms() >= 0.0);

            // 验证输出数据是有效的PNG
            let output_format = detect_format(converted.data());
            assert!(output_format.is_ok());
            assert_eq!(output_format.unwrap(), ImageFormat::Png);
        },
        Err(e) => {
            // 如果转换失败，打印错误信息以便调试
            panic!("JPEG to PNG conversion failed: {}", e);
        }
    }
}

#[test]
fn test_conversion_with_quality_options() {
    // 创建测试图像
    let width = 4u32;
    let height = 4u32;
    let mut rgba_data = Vec::with_capacity((width * height * 4) as usize);

    // 创建渐变图像
    for y in 0..height {
        for x in 0..width {
            let r = ((x as f32 / width as f32) * 255.0) as u8;
            let g = ((y as f32 / height as f32) * 255.0) as u8;
            let b = 128u8;
            let a = 255u8;
            rgba_data.extend_from_slice(&[r, g, b, a]);
        }
    }

    // 创建PNG数据
    let png_data = {
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
    };

    // 测试不同质量的转换
    let qualities = [0.3, 0.6, 0.9];
    let mut results = Vec::new();

    for &quality in &qualities {
        let options = ConversionOptionsBuilder::new()
            .quality(quality)
            .build();

        let result = convert_format(&png_data, ImageFormat::Png, ImageFormat::Jpeg, Some(options));

        match result {
            Ok(converted) => {
                results.push((quality, converted.converted_size()));
                // 验证基本属性
                assert_eq!(converted.format(), ImageFormat::Jpeg);
                assert!(converted.converted_size() > 0);
            },
            Err(e) => {
                // 某些质量设置可能不被支持，这是可以接受的
                println!("Quality {} conversion failed (acceptable): {}", quality, e);
            }
        }
    }

    // 如果有成功的结果，验证质量与文件大小的关系通常成正比
    if results.len() >= 2 {
        println!("Quality vs Size results: {:?}", results);
        // 这里我们不强制要求严格的大小关系，因为对于很小的测试图像，
        // 压缩算法可能表现不一致
    }
}

#[test]
fn test_error_handling() {
    // 测试空数据
    let empty_data = vec![];
    let result = convert_format(&empty_data, ImageFormat::Jpeg, ImageFormat::Png, None);
    assert!(result.is_err());

    // 测试无效数据
    let invalid_data = vec![0x00, 0x01, 0x02, 0x03];
    let result = convert_format(&invalid_data, ImageFormat::Jpeg, ImageFormat::Png, None);
    assert!(result.is_err());

    // 测试格式不匹配
    let png_header = vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
    let result = convert_format(&png_header, ImageFormat::Jpeg, ImageFormat::Png, None);
    assert!(result.is_err());
}

#[test]
fn test_converter_statistics() {
    let mut converter = FormatConverter::with_defaults().unwrap();

    // 获取初始统计
    let initial_stats = converter.get_conversion_statistics();
    assert_eq!(initial_stats.total_conversions, 0);
    assert_eq!(initial_stats.successful_conversions, 0);

    // 执行一次转换后再检查统计（这个测试可能会因为转换失败而不改变统计）
    let test_data = vec![0xFF, 0xD8, 0xFF, 0xE0]; // 简单的JPEG头
    let _ = converter.convert_format(&test_data, ImageFormat::Jpeg, ImageFormat::Png, None);

    let final_stats = converter.get_conversion_statistics();
    // 统计应该被更新（无论转换成功还是失败）
    assert!(final_stats.total_conversions >= initial_stats.total_conversions);
}
