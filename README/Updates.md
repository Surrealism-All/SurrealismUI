## V0.3.0 (Slint 1.3.2)
- 中文
  - 所有组件更名`SUR`为`S`
  - 默认文字采用`Arial`
  - 重写所有组件 (`SMenu` 除外)
  - 重构`themes/index.slint`（用于导出内置Schema，内置Global）
  - 增加use方式导出内置方法，内置结构体，内置枚举等
  - 使用use方式对组件进行插槽预备
  - 修改内置主题色
  - 重构项目结构
- English
  - Renaming all components from `SUR` to `S`
  - Default text adopts `Arial`
  - Rewrite all components（except `SMenu`）
  - Refactoring `themes/index. slint` (used to export built-in schemas, built-in Global)
  - Add use mode to export built-in functions, built-in struct, built-in enum, etc
  - Prepare slots for components using the use method
  - Modify built-in theme colors
  - Refactoring project structure
## V0.2.2 （Slint 1.3.0）

- 中文：
  - 优化内置Global：
    - 修复标准内置方法：`get-padding()`
    - 增加`PaddingType Enum`类型`PaddingType.Tag`
    - 增加标准内置方法`get-color()`
    - 增加标准内置枚举`ColorLevel`
  - 优化`SText`
    - 修改属性名`content -> text`
  - 优化`STag`：
    - 修复`STag`样式异常
    - `STag` remove content property , please use text (as Builtin `Text`)
    - `callback clicked(string)`增加返回参数(`tag text`)
  - 优化`SIcon`
    - 修改属性名`icon -> source`
    - 移除`get-icon()`
  - 优化`SButton`
    - 增加`show-icon`属性控制是否加载图片
    - 修复按钮异常
    - 修改属性名`content -> text`
  - 优化`SLink`
    - 修改属性名`content -> text`
    - `callback clicked(string)`增加返回参数(`link text`)
    - 增加hover控制下划线触发效果
    - 增加`underline`属性控制下划线显示
  - 修复`SURAvatar`默认Icon消失问题
- English
  - Optimize built-in Global:
    - Fix standard built-in methods: ` get padding ()`
    - Add `PaddingType Enum` type `PaddingType.Tag`
    - Add Standard Built-in Method ` get color()`
    - Add Standard Built-in Enumeration ` ColorLevel`
  - Optimize ` SText`
    - Modify Attribute Name ` content ->text`
  - Optimize `STag`:
    - Fix `STag` style anomalies
    - `STag` remove content property, please use text (as Built in `Text`)
    - `callback clicked (string)` Add return parameter (`tag text`)
  - Optimize ` SIcon`
    - Modify Attribute Name ` icon ->source`
    - Remove ` get icon ()`
  - Optimize ` SButton`
    - Add the `show icon` attribute to control whether to load images
    - Fix button error
    - Modify Attribute Name ` content ->text`
  - Optimize ` SLink`
    - Modify Attribute Name ` content ->text`
    - `callback clicked (string)` Add return parameters (`link text`)
    - Add hover control underline trigger effect
    - Add the `underline` attribute to control the display of underscores
  - Fix the issue of `SURAvatar` default Icon disappearing

## V0.2.1
  - add `SURTree`
  - add `SURFile`

## V0.2.0
  - add `SURSwitchOption`
  - add `SURSwitchGroup`
  - optimize `SURInput`

## V0.1.7
  - add `SURSwitch`
  - add `SURDrawer`
  - add `SURAlert`

## V0.1.6
  - solve `SURLoading` animation!

## V0.1.5
  - add `SURMenu`
  - enhance `SURTip` (the location of the tip can be changed now  and you can show it with hover ! )

## V0.1.4
  - add `SURTip`
  - add `SURLoading`
  - add `SURDialog`

## V0.1.3
  - add `SURBadge`
  - add `Progress`
  - add `Persona`

## V0.1.2
  - rebuild components (have `SURIcon`)
  - rebuild `SURIcon`
  - rebuild file structure
  - solve memery overflow issue
  - use minimize import principle (remove inner loop to judge component show!)❗
  - test use Rust✅

## V0.1.1
  - add `SURRadio`
  - add `SURDivider`
  - add `SURCollection`
  - add `SURPopup`

## V0.1.0
  - Adopting Fluent2's component design style
  - Multiple default methods are provided for consumers to call (see index.slint which on the outermost side)
  - Decoupling functions and components
  - Fix some style errors
  - add `SURLink` and `SURAvatar`
