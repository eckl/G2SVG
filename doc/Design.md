# G2SVG 软件设计说明书

## 1. 文档概述

### 1.1 软件目标与背景
本软件旨在构建一个基于 **Rust** 语言开发的高性能、高可靠性电力系统图形转换工具 **G2SVG**。其核心任务是依据国际电工委员会（IEC）制定的标准：
- **IEC 61970-453:2018** (*Energy management system application program interface (EMS-API) - Part 453: Diagram layout profile*)
- **IEC TS 61970-556:2016** (*Energy management system application program interface (EMS-API) - Part 556: CIM based graphic exchange format (CIM/G)*)

将电力调度与能量管理系统（EMS/SCADA）中广泛使用的 **CIM/G** 格式图形文件（`.g` 文件及关联定义文件）精确转换为符合 W3C SVG 规范且保留 CIM 语义元数据的 **CIM/SVG** 文件，实现电力系统接线图、厂站图、电网拓扑图在现代 Web 浏览器、跨平台监控客户端以及矢量图形环境中的无损渲染与交互。

### 1.2 遵循的标准规范体系
1. **IEC TS 61970-556:2016**：
   - 规定了 CIM/G 文件的 XML 语法结构（`<G>` 根节点、视口属性 `viewbox`、背景 `background`、图层 `<Layer>`、动态数据列表 `<DataList>` 等）。
   - 规定了基础绘图图元（`<rect>`, `<circle>`, `<ellipse>`, `<line>`, `<polyline>`, `<polygon>`, `<path>`, `<text>`, `<image>`）及简写属性规范（如 `w`, `h`, `lc`, `lw`, `ls`, `fc`, `fm`, `tf`, `fs`, `ff` 等）。
   - 规定了图元定义文件（`Element.d`, `Color.d`, `Style.d`, `Menu.d`）与图元模板引用机制（`loc`, `show`, `data`, `glue`, `box`, `A` 等）。
   - 提供了 Annex A（电力系统标准图元定义）、Annex B（电压等级与颜色代码映射）、Annex C（默认图形对象样式）、Annex D（发电厂单线图典型范例）。
2. **IEC 61970-453:2018**：
   - 定义了 CIM 图形布局模型（Diagram Layout Profile），包括 `Diagram`, `DiagramObject`, `DiagramObjectPoint`, `DiagramObjectGluePoint`, `DiagramObjectStyle`, `VisibilityLayer`, `TextDiagramObject`。
   - 明确了图形布局坐标系（左手系屏幕坐标 vs 右手系笛卡尔坐标）、图元与物理设备（IdentifiedObject / PSR）的关联映射（`mRID` 绑定）、图元绘制顺序（`drawingOrder`）、图层控制等。
3. **W3C Scalable Vector Graphics (SVG) 1.1 / 2.0**：
   - 目标文件为规范的矢量图形，支持 `<defs>`, `<use>`, CSS 样式表、矢量变换（`transform`）、命名空间元数据（`cims:`, `cimg:`, `cim:`）。

---

## 2. 系统总体架构设计

### 2.1 开发与运行环境
- **开发语言**：Rust (2024 Edition)
- **运行平台**：Windows 11 及全平台（支持 Linux, macOS 等操作系统）
- **核心依赖库**：
  - `roxmltree` / `quick-xml`：高性能只读与事件式 XML DOM 解析。
  - `encoding_rs`：支持 GBK / GB2312 / GB18030 / UTF-8 / ISO-8859-1 等多字符编码自动检测与解码。
  - `clap`：现代化命令行参数解析器。
  - `thiserror`：强类型结构化错误处理。

### 2.2 模块划分与架构层次

G2SVG 采用模块化分层设计，确保高内聚、低耦合：

```
                    ┌────────────────────────────────────────┐
                    │            CLI (main.rs)               │
                    │   命令行交互、批量处理、参数配置       │
                    └───────────────────┬────────────────────┘
                                        │
                    ┌───────────────────▼────────────────────┐
                    │           G2SVG API (lib.rs)           │
                    │   文件转换入口、定义解析协调调度       │
                    └─────────┬───────────────────┬──────────┘
                              │                   │
      ┌───────────────────────▼────────┐ ┌────────▼────────────────────────┐
      │       解析层 (parser)          │ │       转换引擎 (converter)       │
      │ ├─ encoding: 编码检测与解码    │ │ ├─ g2svg: 树形变换与渲染生成     │
      │ ├─ coords: 坐标/属性规整化     │ │ ├─ config: 符号内联/复用转换策略 │
      │ ├─ defs_parser: 模板定义解析   │ └──────────────────────────────────┘
      │ └─ cimg_parser: CIM/G语法分析  │                  ▲
      └───────────────┬────────────────┘                  │
                      │ 注册到注册表                      │
                      ▼                                   │
      ┌────────────────────────────────┐                  │
      │        模型层 (model)          │                  │
      │ ├─ cimg: CIM/G AST模型         │                  │
      │ └─ defs: 图元/色彩/样式定义    ├──────────────────┘
      └───────────────▲────────────────┘
                      │ 初始化预置资源
      ┌───────────────┴────────────────┐
      │       内置规范库 (builtin)     │
      │ ├─ elements: Annex A 规范图元  │
      │ ├─ colors: Annex B 电压颜色表  │
      │ └─ styles: Annex C 默认样式集  │
      └────────────────────────────────┘
```

---

## 3. 标准模型分析与属性映射规则

### 3.1 CIM/G 与 SVG 基础绘图属性对照映射（IEC TS 61970-556 Table 2）

CIM/G 标准为提升数据传输和存储效率，对 SVG 的通用属性进行了缩写。转换引擎执行无损规范化映射：

| CIM/G 缩写属性 | SVG 规范属性名 | 说明与转换逻辑 |
| :--- | :--- | :--- |
| `w` | `width` | 宽度（数值单位：px） |
| `h` | `height` | 高度（数值单位：px） |
| `lc` | `stroke` | 线条颜色（Line Color） |
| `lw` | `stroke-width` | 线条宽度（Line Width） |
| `ls` | `stroke-dasharray` | 虚线线型（Line Style） |
| `fc` | `fill` | 填充颜色（Fill Color） |
| `fm` | `fill-rule` | 填充规则（`0` 映射为 `evenodd`，非0为 `nonzero`） |
| `tf` | `transform` | 几何变换（平移、旋转、缩放） |
| `fs` | `font-size` | 字体大小（Font Size） |
| `ff` | `font-family` | 字体族（如 Arial, 宋体等） |
| `value` (在 `<text>` 中) | 节点文本内容 | `<text x="..." y="..." value="ABC"/>` 转换为 `<text x="..." y="...">ABC</text>` |

### 3.2 坐标系与视口转换（IEC 61970-453 Clause 5.2.2 & 556 Clause 6.2）
- **CIM/G 根节点**：`<G viewbox="x, y, w, h" background="r, g, b">`
- **SVG 根节点**：
  ```xml
  <svg viewBox="x y w h" width="w" height="h"
       xmlns="http://www.w3.org/2000/svg"
       xmlns:xlink="http://www.w3.org/1999/xlink"
       xmlns:cims="http://iec.ch/TC57/1999/rdf-schema-extensions-1999#"
       xmlns:cimg="http://iec.ch/TC57/61970-556#">
  ```
- **背景转换**：将 `background` 渲染为全视口矩形 `<rect class="cimg-background" x="x" y="y" width="w" height="h" fill="rgb(r, g, b)"/>`。

### 3.3 图元实例化与几何变换计算（IEC TS 61970-556 Clause 8.3）
在 CIM/G 中，图元定义在 `Element.d`（或 `<defs id="element">`）中声明尺寸 `box="X, Y, W, H"`；在图形中通过 `<Breaker id="CB01" loc="x, y w, h" data="#mRID" show="Q,T,F,S"/>` 进行引用。
- **实际坐标计算**：
  $$\text{x\_real} = X_{\text{definition}} + x_{\text{reference}}$$
  $$\text{y\_real} = Y_{\text{definition}} + y_{\text{reference}}$$
- **缩放因子计算**：
  $$S_x = \frac{w}{W},\quad S_y = \frac{h}{H}$$
  若引用尺寸与定义尺寸不同，转换引擎自动附加 `transform="translate(x, y) scale(Sx, Sy)"`。
- **母线与线路维度保护**：当水平母线高度 $h=0$ 时，防止垂直方向被压平，自动保持 $S_y=1.0$；垂直母线自动识别并施加正交旋转/缩放。

### 3.4 拓扑状态与电压等级颜色映射（IEC TS 61970-556 Annex B）

系统严格内建 Annex B Table B.1 电压与颜色基准映射，同时支持用户在 `Color.d` 中自定义覆盖：

| 代码 (T Code) | 电压等级 | 标称颜色 | RGB 颜色值 | CSS 类名 |
| :---: | :---: | :---: | :---: | :---: |
| 1 | 1000 kV | Blue | `rgb(0, 0, 255)` | `.volt-1000kV` |
| 2 | 800 kV | Blue | `rgb(0, 0, 255)` | `.volt-800kV` |
| 3 | 750 kV | Orange | `rgb(250, 128, 10)` | `.volt-750kV` |
| 4 | 660 kV | Orange | `rgb(250, 128, 10)` | `.volt-660kV` |
| 5 | 500 kV | Red | `rgb(255, 0, 0)` | `.volt-500kV` |
| 6 | 400 kV | Red | `rgb(255, 0, 0)` | `.volt-400kV` |
| 7 | 330 kV | Bright blue | `rgb(30, 144, 255)` | `.volt-330kV` |
| 8 | 220 kV | Purple | `rgb(128, 0, 128)` | `.volt-220kV` |
| 9 | 110 kV | Vermeil | `rgb(240, 65, 85)` | `.volt-110kV` |
| 10 | 66 kV | Gold | `rgb(255, 204, 0)` | `.volt-66kV` |
| 11 | 35 kV | Yellow | `rgb(255, 255, 0)` | `.volt-35kV` |
| 12 | 20 kV | Brown | `rgb(226, 172, 6)` | `.volt-20kV` |
| 13 | 15 kV | Dark green | `rgb(0, 128, 0)` | `.volt-15kV` |
| 14 | 13 kV | Light green | `rgb(0, 210, 0)` | `.volt-13kV` |
| 15 | 10 kV | Crimson | `rgb(185, 72, 66)` | `.volt-10kV` |
| 16 | 6 kV | Dark blue | `rgb(0, 0, 139)` | `.volt-6kV` |
| 17 | 0 kV | Grey | `rgb(128, 128, 128)` | `.volt-0kV` |

在渲染时，图元的 `show="Q,T,F,S"` 中的 `T` 值优先解析为拓扑电压颜色；容器元素（如 `<VoltageLevel id="500kV">`）的电压属性向下传递继承。

### 3.5 IEC 61970-453 语义元数据保留
输出的 CIM/SVG 不仅可被常规矢量图像工具查看，还在节点中保留了 IEC 61970-453 CIM 关联信息：
- `cims:data`：设备关联的物理 CIM 对象 ID（`mRID`）。
- `cims:show`：四字节状态掩码（质量 Q、拓扑 T、闪烁 F、形态 S）。
- `cims:A`：关联交互动作（锚点链接）。
- `cims:connect`：连线与端子连接关系（连接点 `glue` 与设备）。
- `<metadata><cimg:Diagram>...<cimg:DataList>...</cimg:Diagram></metadata>`：图表级与动态刷新数据集清单。

---

## 4. 关键功能与实现细节

### 4.1 符号渲染双模式设计
1. **模式一：标准符号复用模式（Default, `<use>`）**
   - 转换器将所有设备模板放置于 SVG 的 `<defs>` 中；
   - 引用处生成 `<g transform="..."> <use xlink:href="#symbol_id" /> </g>`；
   - 优点：SVG 体积极小、结构清晰、便于动态批量修改样式。
2. **模式二：直接几何内联模式（`--inline-symbols`）**
   - 将设备内部的所有基本几何形状展开克隆到具体的 `<g>` 容器中，直接应用缩放与平移变换；
   - 优点：具备极高的跨平台兼容性，在部分对 `<use>` 样式穿透支持较弱的嵌入式矢量绘图引擎或旧版 CAD 浏览器中可完美显示。

### 4.2 字符编码自动侦测与转码
工业 EMS 导出的 CIM/G 文件常采用 GBK/GB2312 编码。解析器首先检测 UTF-8 BOM，再嗅探 XML 声明中的 `encoding` 属性，结合 `encoding_rs` 进行平滑无损转码，并在生成的 SVG 中统一规范输出为标准 UTF-8。

---

## 5. 验证与测试

测试套件已完整覆盖标准附录与工业场景：
1. **Annex D 发电厂单线图转换测试**：完整解析附录 D 范例中包含的双母线、3/2断路器接线 Bay、发电机、变压器、交流线路及动态数据清单，验证 SVG 语法合法性及节点属性完备性。
2. **变电站与双电压等级图测试（Figure 16）**：验证 500kV / 220kV 多级容器嵌套、母线缩放及连接线（Link）呈现。
3. **符号内联展开与 `<use>` 模式双向测试**：确保两种模式下的图形输出一致且有效。
4. **GBK 汉字编码测试**：验证中文字符（如电网厂站名称）在转换过程中的编解码准确性。
5. **电压拓扑着色测试**：验证根据 `show` 属性正确匹配 Annex B 规定的 RGB 颜色值。

---

## 6. 软件使用指南

### 6.1 编译与构建
```bash
# 运行单元与集成测试
cargo test

# 构建 Release 高性能二进制
cargo build --release
```

### 6.2 命令行接口（CLI）使用方式
```bash
# 1. 转换单个 CIM/G 文件到 SVG
g2svg input.g -o output.svg

# 2. 批量转换整个目录下的所有 .g 文件
g2svg ./diagrams -o ./svg_out

# 3. 指定自定义图元定义文件目录 (Element.d, Color.d, Style.d)
g2svg input.g -o output.svg -d ./custom_defs

# 4. 启用符号内联模式（展开 <use> 为独立矢量图形）
g2svg input.g -o output.svg --inline-symbols

# 5. 指定输出 SVG 宽高与详细日志
g2svg input.g -o output.svg --width 1920 --height 1080 --verbose
```