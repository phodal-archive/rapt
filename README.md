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
 - [pngquant with rust](https://pngquant.org/install.html)

Rust Library:

 - [Citrus](https://gitlab.com/citrus-rs/citrus)  is a tool that helps convert C programs to Rust programs. It transforms C syntax to Rust syntax, but ignores C semantics.
 - [LodePNG](https://github.com/kornelski/lodepng-rust)  is a pure Rust PNG image decoder and encoder. Allows easy reading and writing of PNG files without any system dependencies.
 - [Oxipng](https://github.com/shssoichiro/oxipng) is a multithreaded lossless PNG compression optimizer. It can be used via a command-line interface or as a library in other Rust programs.
 
Benchmark

 - [RGBA Structural Similarity](https://github.com/kornelski/dssim)  Image similarity comparison simulating human perception (multiscale SSIM in Rust) 
 
9Patch -> NinePatch -> 九宫格

## Notes

### AAPT2

aapt2将原先的资源编译打包过程拆分成了两部分，即编译和链接。

 - 编译：将资源文件编译为二进制格式文件
 - 链接：将编译后的所有文件合并，打包成一个单独文件

#### Format


### Format
The file starts with a simple header. All multi-byte fields are little-endian.

| Size (in bytes) | Field         | Description                                          |
|:----------------|:--------------|:-----------------------------------------------------|
| `4`             | `magic`       | The magic bytes must equal `'AAPT'` or `0x54504141`. |
| `4`             | `version`     | The version of the container format.                 |
| `4`             | `entry_count` | The number of entries in this container.             |

This is followed by `entry_count` of the following data structure. It must be aligned on a 32-bit
boundary, so if a previous entry ends unaligned, padding must be inserted.

| Size (in bytes) | Field          | Description                                                                                               |
|:----------------|:---------------|:----------------------------------------------------------------------------------------------------------|
| `4`             | `entry_type`   | The type of the entry. This can be one of two types: `RES_TABLE (0x00000000)` or `RES_FILE (0x00000001)`. |
| `8`             | `entry_length` | The length of the data that follows.  Do not use if `entry_type` is `RES_FILE`; this value may be wrong.  |
| `entry_length`  | `data`         | The payload. The contents of this varies based on the `entry_type`.                                       |

If the `entry_type` is equal to `RES_TABLE (0x00000000)`, the `data` field contains a serialized
[aapt.pb.ResourceTable](Resources.proto).

If the `entry_type` is equal to `RES_FILE (0x00000001)`, the `data` field contains the following:


| Size (in bytes) | Field            | Description                                                                                               |
|:----------------|:-----------------|:----------------------------------------------------------------------------------------------------------|
| `4`             | `header_size`    | The size of the `header` field.                                                                           |
| `8`             | `data_size`      | The size of the `data` field.                                                                             |
| `header_size`   | `header`         | The serialized Protobuf message [aapt.pb.internal.CompiledFile](ResourcesInternal.proto).                 |
| `x`             | `header_padding` | Up to 3 bytes of zeros, if padding is necessary to align the `data` field on a 32-bit boundary.           |
| `data_size`     | `data`           | The payload, which is determined by the `type` field in the `aapt.pb.internal.CompiledFile`. This can be a PNG file, binary XML, or [aapt.pb.XmlNode](Resources.proto). |
| `y`             | `data_padding`   | Up to 3 bytes of zeros, if `data_size` is not a multiple of 4.                                            |


### Android AAPT Compile config

libs:

```
"libandroidfw",
"libutils",
"liblog",
"libcutils",
"libexpat",
"libziparchive",
"libpng",
"libbase",
"libprotobuf-cpp-lite",
"libz",
"libbuildversion",
```

### Readable

```

FIND_PACKAGE(JAVA REQUIRED)
INCLUDE_DIRECTORIES(${JAVA_INCLUDE_DIRS})
LINK_DIRECTORIES(${JAVA_LIBRARY_DIRS})

#FIND_PACKAGE(JNI REQUIRED)
#INCLUDE(UseJava)

INCLUDE_DIRECTORIES(~/studio-master-dev/prebuilts/studio/jdk/mac/Contents/Home/include/darwin)
INCLUDE_DIRECTORIES(~/studio-master-dev/prebuilts/studio/jdk/mac/Contents/Home/include)
LINK_DIRECTORIES(~/studio-master-dev/prebuilts/studio/jdk/mac/Contents/Home)

INCLUDE_DIRECTORIES(~/studio-master-dev/external/libpng/include)
LINK_DIRECTORIES(~/studio-master-dev/external/libpng)

INCLUDE_DIRECTORIES(~/frameworks/base/libs/androidfw/include)
LINK_DIRECTORIES(~/frameworks/base/libs/androidfw)

INCLUDE_DIRECTORIES(~/system/core/include)
LINK_DIRECTORIES(~/frameworks/base/libs)

INCLUDE_DIRECTORIES(~/system/libbase/include)
LINK_DIRECTORIES(~/system/libbase)

INCLUDE_DIRECTORIES(~/studio-master-dev/external/googletest/googlemock/include)
LINK_DIRECTORIES(~/studio-master-dev/external/googlemock)

INCLUDE_DIRECTORIES(~/studio-master-dev/external/googletest/googletest/include)
LINK_DIRECTORIES(~/studio-master-dev/external/googletest)

INCLUDE_DIRECTORIES(~/frameworks/native/include)
LINK_DIRECTORIES(~/frameworks/native)

INCLUDE_DIRECTORIES(~/system/libziparchive/include)
LINK_DIRECTORIES(~/system/libziparchive)

FIND_PACKAGE(PROTOBUF REQUIRED)
INCLUDE_DIRECTORIES(${PROTOBUF_INCLUDE_DIRS})
LINK_DIRECTORIES(${PROTOBUF_LIBRARY_DIRS})
ADD_DEFINITIONS(${PROTOBUF_DEFINITIONS})
```

add `build/version.h`

```c++
#ifndef BUILD_VERSION_H
#define BUILD_VERSION_H

#include <string>

namespace android {
namespace build {

std::string GetBuildNumber();

} // namespace build
} // namespace android

#endif  // BUILD_VERSION_H
```

### Compilable

[MacOS X 编译Android源码](https://juejin.im/post/5b690cb46fb9a04fd044885d)

[https://android.googlesource.com/platform/build/soong/](https://android.googlesource.com/platform/build/soong/)

[Android 镜像使用帮助](https://mirror.tuna.tsinghua.edu.cn/help/AOSP/)

```
mkdir ~/bin
PATH=~/bin:$PATH
curl https://storage.googleapis.com/git-repo-downloads/repo > ~/bin/repo
chmod a+x ~/bin/repo
```

```
wget -c https://mirrors.tuna.tsinghua.edu.cn/aosp-monthly/aosp-latest.tar # 下载初始化包
tar xf aosp-latest.tar
cd AOSP   # 解压得到的 AOSP 工程目录
# 这时 ls 的话什么也看不到，因为只有一个隐藏的 .repo 目录
repo sync # 正常同步一遍即可得到完整目录
# 或 repo sync -l 仅checkout代码
```

#### Mount

[Establishing a Build Environment](https://source.android.com/setup/build/initializing)

resize:

```bash
hdiutil resize -size <new-size-you-want>g ~/android.dmg.sparseimage
```

mount 

```bash
hdiutil attach ~/android.dmg -mountpoint /Volumes/android;
```

unmount

```bash
hdiutil detach /Volumes/android;
```


bash 

```bash
mountAndroid() { hdiutil attach ~/android.dmg -mountpoint /Volumes/android; }
umountAndroid() { hdiutil detach /Volumes/android; }
```

## Development

CLION import

[CLion project generator](https://android.googlesource.com/platform/build/soong/+/HEAD/docs/clion.md)

```
$ export SOONG_GEN_CMAKEFILES=1
$ export SOONG_GEN_CMAKEFILES_DEBUG=1

# all

$ make -j64

# some
$ make frameworks/native/service/libs/ui
```

### Proto

```
brew cask install protobuf
```


```bash
cargo install protobuf-codegen
```

### Compile AAPT

[如何在不影响原有aapt源码的情况下编译定制版aapt](https://blog.csdn.net/sbsujjbcy/article/details/51418336)

[如何在Majove下编译aapt2](https://www.jianshu.com/p/03576605f436)

