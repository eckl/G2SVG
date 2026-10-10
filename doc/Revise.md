# IEC标准修订文档
实际工程文件(CIM/G文件与CIM/SVG文件)与IEC标准存在差异，需要修订。 

## 1. 对IEC 61970-453的修订



## 2. 对IEC 61970-556的修订

- **<G> 根节点**
  形如:
  <G w="1600.0" h="1400.0" bgc="0,0,0" bgf="0" Substation="123456789012345678::西安.长安变">
  其中, 只关心w, h, bgc, Substation属性
        w为宽度, h为高度, bgc为背景色(R,G,B), Substation为厂站编号::地域.厂站名

- **<Layer> 层节点**：

- **元素节点**
  - **<DText>节点** Ignored
  - **<Text>节点**
    ts文本内容, x, y, fc, ff, fs, lc, ls, lw, FontInterval字符间距 RowInterval行间距, tfr文本几何变换
  - **<ACLine>节点**及**<ACLineEnd>节点**
    线路名(key_name, key_name1, key_name2, keyidDesc, keyid1Desc), d线段, x, y, fc, lc, ls, lw, tfr
  - **<Arrester>节点** Ignored
  - **<Ascoil>节点**
    电感器名(key_name, keyidDesc), x, y, fc, lc, ls, lw, tfr
  - **<Bay>节点** Ignored
  - **<Bus>节点**
    母线名(key_name, keyidDesc), d, x, y, x1, y1, x2, y2, fc, lc, ls, lw, tfr
  - **<Capacitor>节点**及**<Capacitor_P>节点**
    电容器名(key_name, keyidDesc), x, y, fc, lc, ls, lw, tfr
  - **<CBreaker>节点**
    断路器名(key_name, keyidDesc), x, y, fc, lc, ls, lw, tfr
  - **<ConnectLine>节点**
    d, fc, lc, ls, lw, tfr
  - **<Disconnector>节点**
    刀闸名(key_name, keyidDesc), x, y, fc, lc, ls, lw, tfr
  - **<DollyBreaker>节点**
    小车开关名(key_name1, key_name2, keyidDesc), x, y, fc, lc, ls, lw, tfr
  - **<EnergyConsumer>节点** Ignored
  - **<GroundDisconnector>节点**
    接地刀闸名(key_name, keyidDesc), x, y, fc, lc, ls, lw, tfr
  - **<Gzp>节点** Ignored
  - **<image>节点** Ignored
  - **<line>节点**
    x1, y1, x2, y2, fc, lc, ls, lw, tfr
  - **<Merge>节点** Ignored
  - **<Other>节点** Ignored
  - **<poke>节点** Ignored
  - **<PT>节点**
    电压互感器名(keyidDesc), x, y, fc, lc, ls, lw, tfr
  - **<rect>节点**
    w, h, x, y, fc, lc, ls, lw, tfr
  - **<Terminal>节点**
    接地变/所变名(keyidDesc), x, y, fc, lc, ls, lw, tfr
  - **<transformer2>节点**
    双圈变名(key_name1, key_name2, keyidDesc), x, y, fc, lc, ls, lw, tfr
  - **<transformer3>节点**
    三圈变名(key_name1, key_name2, key_name3, keyidDesc), x, y, fc, lc, ls, lw, tfr
  - **<triangle>节点**
    w, h, x, y, fc, lc, ls, lw, tfr
  - **<Zxddd>节点**
    地刀名(key_name), x, y, fc, lc, ls, lw, tfr

## 3. 转换内容
仅转换与图形显示有关的内容，即可以显示的元素及其与显示有关的属性。



    


    
  
