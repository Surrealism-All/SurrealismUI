# CHANGELOG

## V0.5.4
### 中文
  - 修复`SNumberInput`并没有在更改input的text时联动更改value值
### English
  - fix `SNumberInput` can not change value when input text has been changed
## V0.5.3
### 中文
  - 优化`SDialog`
  - 修复警告
  - Slint更新1.9.1
### English
  - Optimize `SDialog`
  - fix warning
  - update slint to 1.9.1
## V0.5.2
### 中文
  - 优化`SSelect`层级结构
### English
  - Optimize structure level for `SSelect`
## V0.5.1
### 中文
  - 去除重复的`close-one`图标
  - `SSelect`增加触摸区域
### English
  - remove redundant icon `close-one`
  - Add TouchArea into `SSelect`
## V0.5.0
### 中文
  - 优化`SSwitchGroup`文字
  - 增加`STabbar`点击回调事件
  - 修复`SCollapse`宽度超出
  - 优化`SCollapse`图标旋转代替图标替换
  - 去除`SCollapse` `init callback`
  - 优化`SIcon`旋转动画
  - 增加`SRadio`点击效果
  - 增加`SCheckbox`点击效果
### English
  - Optimize text in `SSwitchGroup`
  - Add `STabbar` clicked callback
  - Fix `SCollapse` width overflow
  - Optimize `SCollapse` rotation icon
  - remove `SCollapse` init callback
  - Optimize `SIcon` rotation animation
  - Add `SRadio` click effect
  - Add `SCheckbox` click effect
## V0.4.5
### 中文
  - 增加`SCatalog`目录
  - 优化`SSwitch`动画
  - 增加`SSelect` `active`
  - 增加`SSlider`滑块
  - 增加`STabbar`文档
  - 修改`SAlert`中`Info`主题文字颜色
  - 优化`SNumberInput`严格模式
  - 修复`SSlider, SInput` `init` 回调崩溃
### English
  - Add `SCatalog`
  - Optimize `SSwitch` animation
  - Add `SSelect` property `active`
  - Add `SSlider` widget
  - Add `STabbar` document
  - fix `SAlert` `info` theme font color
  - Optimize `SNumberInput` strict mode
  - Fix `SSlider, SInput` `init` callback crash
## V0.4.4
### 中文
  - 增加`STabbar`
  - `STag`增加hover
### English
  - Add `STabbar`
  - Add hover to `STag`
## V0.4.3
### 中文
  - 增加`SNumInput`数字输入
  - 增加`SCalendar`时间日期
  - 修复`SAvatar`None异常
  - 修复`SMenu`callback change
### English
  - Add `SNumInput` 
  - Add `SCalendar`
  - Fix `SAvatar` None Option unwrap
  - Fix `SMenu`callback change
## V0.4.2
### 中文
  - 增加`SCarousel`走马灯
  - 增加`STimeLine`时间轴
  - 修复`SCheckbox`和`SRadio`布局异常
### English
  - Add `SCarousel`
  - Add `STimeLine`
  - Fix layout error in `SCheckbox`and`SRadio`
## V0.4.1
### 中文
  - 增加`SStep`进度指向线颜色效果
  - 增加`SPagination`分页器组件
  - 增加`SKeyBoard`虚拟键盘 (可使用子组件任意扩展)
  - 增加`SButton`,`SCheckbox`,`SRadio`禁用选项
  - 增加`SDialog`非触摸位置关闭控制选项:`mask-close`
  - 去除`SLoading`的默认初始化回调
### English
  - Increase the color effect of the progress pointing line in `SStep`
  - Add the `SPagination` paginator component
  - Add `SKeyBoard` virtual keyboard (it can be expanded with any sub components)
  - Add disable options for `SButton`, `SCheckbox`, and `SRadio`
  - Add `SDialog` non touch position close control option: `mask close`
  - Remove the default initialization callback for `SLoading`
## V0.4.0
### 中文
  - `SCheckbox`内部选择区添加 border，优化显示
  - `SProgress`样式优化，增加圆形进度条
  - 修复`SButton`在`STableColumnFlex`中的异常
  - 增加`SPopover`组件（气泡卡片无模态效果）
  - 增加`SStep`组件（按步骤执行）
### English
  - Add a border to the internal selection area of `SCheckbox` to optimize display
  - Style optimization for `SProgress`, adding a circular progress bar
  - Fix exceptions in `SButton` in `STableColumnFlex`
  - Add the `SPopover` component (bubble card has no modal effect)
  - Add the `SStep` component (do something step by step)
## V0.3.5
### 中文
  - 增加`SCheckbox`(当用户需要多选时)
### English
  - Add `SCheckbox`(when people want to select multi items)
## V0.3.4
### 中文
  - 修复`SSelect`组件 icon、文字使用主题色变化
  - 增加`STab`组件提供选项卡功能，以便用户可以在不同的内容板块之间切换
### English
  - Fix changes in the theme color of the 'SSelect' component icon and text usage
  - Add the 'STab' component to provide tab functionality, so that users can switch between different content sections
## V0.3.3
### 中文
  - `SIcon`增加 state,hover 颜色效果
  - 重写`SMenu`
  - 优化`STable`,增加表格列自定义组件 (see wiki [STable](https://github.com/Surrealism-All/SurrealismUI/wiki/07_Data-Components-%E6%95%B0%E6%8D%AE%E7%BB%84%E4%BB%B6#stable-%E8%A1%A8%E6%A0%BC))
### English
  - Add state and hover color effects to `SIcon`
  - Rewrite `SMenu`
  - Optimize `STable` , Add Self DefineComponent (see wiki [STable](https://github.com/Surrealism-All/SurrealismUI/wiki/07_Data-Components-%E6%95%B0%E6%8D%AE%E7%BB%84%E4%BB%B6#stable-%E8%A1%A8%E6%A0%BC))
## V0.3.2

### 中文
  - `SSwitch`和`SSwitchGroup`增加响应式选择（由 active 属性进行控制）
  - 优化`STip`文字显示
### English
  - `SSwitch` and `SSwitchGroup` add responsive selection (controlled by the active property)
  - Optimize text display of `STip`

## V0.3.1
### 中文
  - 补充丢失的 SVG 图片
  - 修复组件中`PaddingType.None`以及`BorderType.None`产生的与 Rust 的`Option` 的编译冲突
  - 修复`SIcon`中 colorize 属性导致的闪烁和 None 冲突
  - `SIcon`弥补方案:`self.get-colorize()`
### English
  - add missing SVG images
  - fix the `PaddingType.None` and `BorderType.None` in the component Compilation conflict with Rust `Option`
  - fix None conflict and flicker causes by colorize property in `SIcon`
  - `SICon` Remedial solution:`self.get-colorize()`
## V0.3.0 (Slint 1.3.2)
### 中文
  - 所有组件更名`SUR`为`S`
  - 默认文字采用`Arial`
  - 重写所有组件 (`SMenu` 除外)
  - 重构`themes/index.slint`（用于导出内置 Schema，内置 Global）
  - 增加 use 方式导出内置方法，内置结构体，内置枚举等
  - 使用 use 方式对组件进行插槽预备
  - 修改内置主题色
  - 重构项目结构
### English
  - Renaming all components from `SUR` to `S`
  - Default text adopts `Arial`
  - Rewrite all components（except `SMenu`）
  - Refactoring `themes/index. slint` (used to export built-in schemas, built-in Global)
  - Add use mode to export built-in functions, built-in struct, built-in enum, etc
  - Prepare slots for components using the use method
  - Modify built-in theme colors
  - Refactoring project structure
