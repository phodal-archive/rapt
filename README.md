# Rust with AAPT

## Crunch Images

Android Libs:

 - libpng 是官方的便携式网络图形参考库。它是一个独立于平台的库，其中包含用于处理PNG图像的C函数。它支持几乎所有PNG的功能，并且是可扩展的，并且已经被广泛使用和测试了超过23年。libpng依赖zlib进行数据压缩和解压缩例程。
 - zlib 是提供数据压缩之用的库，由Jean-loup Gailly与Mark Adler所开发，初版0.9版在1995年5月1日发表。zlib使用抽象化的DEFLATE算法，最初是为libpng库所写的，后来普遍为许多软件所使用。
 
其它压缩库：

 - Pngrewrite is command-line utility that reduces the unnecessarily large palettes that some programs write into PNG files. 
 - pngcrush 是一个免费的开源命令行实用程序，用于优化PNG图像文件。它会无损地减小文件的大小-也就是说，生成的“压缩”图像将具有与源图像相同的质量。pngcrush的主要目的是通过尝试各种压缩方法和增量过滤器的组合来减小PNG IDAT数据流的大小。
 - optipng 是一个PNG优化器，可将图像文件重新压缩为更小尺寸，而不会丢失任何信息。
 - pngout
 
算法：

 - [pngquant](https://pngquant.org/) 是一个命令行工具和一个用于有损压缩PNG图像的库。 转换显着减少文件大小（通常高达70％），并保留完整的alpha透明度。 
 - Zopfli 是一个数据压缩算法，可以将数据压缩为 DEFLATE、gzip 或 zli b格式。Zopfli 被认为是当前压缩率最高的 DEFLATE 压缩算法。
 
PNG 压缩算法：

 - [Color quantization](https://en.wikipedia.org/wiki/Color_quantization)
 - [Indexed color](https://en.wikipedia.org/wiki/Indexed_color) 
 
Documents:

 - [优化包大小-PNG部分](https://juejin.im/post/5de77f37e51d45583317d73a)
 - [关于Android图片资源瘦身的奇思妙想](https://cloud.tencent.com/developer/article/1004331) 演示：Pngout

Rust Library:

 - [Oxipng](https://github.com/shssoichiro/oxipng) is a multithreaded lossless PNG compression optimizer. It can be used via a command-line interface or as a library in other Rust programs.
 
9Patch -> NinePatch -> 九宫格

 