## How to use?

0. 设置Huggingface API token环境变量
```
export HF_TOKEN=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
```

1. 从`release`下载程序压缩包，并解压。
2. 使用得到的应用程序，比如：
```
./gcoder "dragon with fire"
```
3. 正常情况下，会输出如下log：
```
jpeg to svg.
Conversion successful.
svg to program.
program to gcode.
Done.
```
4. 并且输出图片（jpeg格式）
![jpeg](/images/foo.jpeg "dragon with fire")

5. 以及对应的SVG图片
![svg](/images/foo.svg "dragon with fire")

6. 最后就是得到的gcode文件，可以使用[gcode查看器](https://ncviewer.com/)检查效果
![gcode](/images/gcode.png "dragon with fire")

TODO:
整合串口输出到设备绘图的部份。
增加AI文生图的其他可选项。
优化文生图的速度，目前可能需要至多1分钟才能拿到图片。


## 如果要自己编译源代码
1. 首先请安装rust编译链：https://www.rust-lang.org/learn/get-started
2. 克隆代码，并进入目录
3. 运行：
```
cargo run "dragon with fire"
```
