<img src="https://img.shields.io/badge/SurrealismUI-0.2.2-orange?style=flat-square&logo=rust&logoColor=%23fff&labelColor=%23DEA584&color=%23DEA584">  <img src="https://img.shields.io/badge/License-MIT-orange?style=flat-square&logoColor=%23fff&labelColor=%2323B898&color=%2323B898">

# SurrealismUI

- author：syf20020816@outlook.com
- createDate：20230908
- updateDate：202301112
- version：0.2.2
- email：syf20020816@outlook.com

<img src="https://github.com/syf20020816/SurrealismUI/blob/main/README/imgs/logo.png">

SurrealismUI是一个完全使用Slint进行构建的Slint第三方组件库

SurrealismUI is a third-party component library built entirely using Slint

## About Doc Icon

- ⛔ ： do not use
- 👍 ： Recommended use

## Themes

Built in 7 theme colors in SurrealismUI

- primary
- success
- info
- warning
- error
- dark 
- light

### themes-color

```
			  default
————————————————————————————————————
|  logic control layer (Rust|C++)  |
————————————————————————————————————
				⇕
————————————————————————————————————
|    UI layer (write components)   |
————————————————————————————————————

		   SurrealismUI
————————————————————————————————————
|  logic control layer (Rust|C++)  |
————————————————————————————————————
				⇕
————————————————————————————————————
|      UI Styles Wrapper layer     |   <-- What SurrealismUI do , see ①
————————————————————————————————————
|   UI layer (write components)    |
————————————————————————————————————

①：define a lot replaceable theme styles and binding styles use theme property , can be customized in slint file or logic control layer , means: all system components are wrapped (Customizing themes in third-party component libraries is very affordable as it acts on the UI layer. SLINT is like an integration of HTML and CSS, so I use this way)(By binding global singleton variables to styles, any component that uses variables can change styles simultaneously)

				System support (like iced)
————————————————————————————————————      ————————————————
|           logic control          | -->  | Theme::Light |
————————————————————————————————————      ————————————————
|             UI layer             |     		  ↓
———————————————————————————————————— 	    |————————————|
						 ↑			     ↓            ↓
				import	  ← Light_Theme Styles   Dark_Theme Styles

## Diff
Slint differs from other GUI frameworks in that the UI layer is completed through. slint, which I believe is good and brings many advantages (compatibility with different platforms, instant preview, maintainability, parallel development, etc.). But this also leads to SLIT being unable to easily customize the theme of the component. Theme customization and switching are dynamic to static processes, which require a lot of logical processing, and this is also same as (HTML+CSS+js | ts)
## Slint be careful
Slint's work on topic definition will simultaneously affect the built-in components currently provided and other languages' API. Although this feature may seem simple, it may bring significant risks, which I believe need to be weighed and considered. Is this necessary? Because for third-party component libraries, although the workload of helping users define themes is large, it is not complex. As the author of SurrealismUI, it is evident that we have successfully implemented the definition of themes in static slint language and can be modified at the logical level. Users customize themes through static file overwriting.
```



#### primary

![image-20230910102452817](https://github.com/syf20020816/SurrealismUI/blob/main/README/imgs/image-20230910102452817.png)

#### success

![image-20230910102504405](https://github.com/syf20020816/SurrealismUI/blob/main/README/imgs/image-20230910102504405.png)

#### info

![image-20230910102558381](https://github.com/syf20020816/SurrealismUI/blob/main/README/imgs/image-20230910102558381.png)

#### warning

![image-20230910102611556](https://github.com/syf20020816/SurrealismUI/blob/main/README/imgs/image-20230910102611556.png)

#### error

![image-20230910102624332](https://github.com/syf20020816/SurrealismUI/blob/main/README/imgs/image-20230910102624332.png)

#### dark

![image-20230910102637280](https://github.com/syf20020816/SurrealismUI/blob/main/README/imgs/image-20230910102637280.png)

#### light

![image-20230910102413761](https://github.com/syf20020816/SurrealismUI/blob/main/README/imgs/image-20230910102413761.png)

## Components

 ### SText
 It is the simplest and most common component in SurrealismUI
 #### properties:
 - `in property <Themes> theme` : Surrealism themes
 #### callbacks: 
 #### functions:
 - `pure public function get()->string` : get content
 - `public function set(content:string)` : set content

#### example

```
import {SText} from "../../index.slint";
import {Themes} from "../../use/index.slint";

component TestWindow inherits Window {
  height: 400px;
  width: 400px;
  
  SText {
    x: 100px;
    y: 20px;
    text: "hello world";
  }
  SText {
    x: 100px;
    theme: Primary;
    text: "hello world";
  }
}
```

![image-20231112134855878](https://github.com/syf20020816/SurrealismUI/blob/main/README/imgs/image-20231112134855878.png)

 ### SIcon
 there are 2658 different icons in SIcon from : https://github.com/bytedance/iconpark
 #### properties:
 - `in property <image> source` : icon source
 - `out property <bool> has-hover` : has hover or not
 - `in property <Themes> theme` : Surrealism theme
 - `in-out property <brush> icon-color` : icon color 
 - `private property <[IconItem]> icon-datas` : source icon datas ⛔
 #### callbacks: 
 - `callback clicked` : run if you click the icon
 #### functions:
 - `pure function get_icon(item:IconItem)->image` : get icon src from for iter item ⛔

#### example

```
import {SIcon} from "../../index.slint";
import {UseIcons,Size,Themes} from "../../themes/index.slint";
export component TestIcon inherits Window {
  height: 400px;
  width: 400px;
  GridLayout {
    spacing: 40px;
    Row{
      
      SIcon{
        height: 30px;
        width: 30px;
        source: @image-url("../../icons/sd-card.svg");
        theme: Themes.Primary;
      }
      SIcon{
        height: 30px;
        width: 30px;
        source: @image-url("../../icons/add-computer.svg");
        theme: Themes.Success;
      }
      SIcon{
        height: 30px;
        width: 30px;
        source: @image-url("../../icons/yep.svg");
        theme: Themes.Error;
      }
      SIcon{
        source: @image-url("../../icons/t-shirt.svg");
        theme: Themes.Dark;
        height: 30px;
        width: 30px;
      }
      SIcon{
        height: 30px;
        width: 30px;
        source: @image-url("../../icons/video-conference.svg");
        theme: Themes.Info;
      }
      SIcon{
        height: 30px;
        width: 30px;
        source:@image-url("../../icons/vacation.svg");
        theme: Themes.Warning;
        clicked=>{
          debug("clicked");
          self.theme= Themes.Error;
          self.height += 2px;
          self.width += 2px;
        }
      }
    }
    Row{
      SIcon{
        height: 30px;
        width: 30px;
        source: @image-url("../../icons/cake-five.svg");
        theme: Themes.Primary;
      }
      SIcon{
        height: 30px;
        width: 30px;
        source: @image-url("../../icons/label.svg");
        theme: Themes.Success;
      }
      SIcon{
        height: 30px;
        width: 30px;
        source: @image-url("../../icons/wifi.svg");
        theme: Themes.Error;
      }
      SIcon{
        source: @image-url("../../icons/wallet-one.svg");
        theme: Themes.Dark;
        height: 30px;
        width: 30px;
      }
      SIcon{
        height: 30px;
        width: 30px;
        source: @image-url("../../icons/game-console.svg");
        theme: Themes.Info;
      }
      SIcon{
        height: 30px;
        width: 30px;
        source: @image-url("../../icons/qiyehao.svg");
        theme: Themes.Warning;
      }
    }
    Row{
      SIcon{
        height: 30px;
        width: 30px;
        source: @image-url("../../icons/scanning-two.svg");
        theme: Themes.Primary;
      }
      SIcon{
        height: 30px;
        width: 30px;
        source: @image-url("../../icons/oceanengine.svg");
        theme: Themes.Success;
      }
      SIcon{
        height: 30px;
        width: 30px;
        source: @image-url("../../icons/zoom-internal.svg");
        theme: Themes.Error;
      }
      SIcon{
        source: @image-url("../../icons/zip.svg");
        theme: Themes.Dark;
        height: 30px;
        width: 30px;
      }
      SIcon{
        height: 30px;
        width: 30px;
        source: @image-url("../../icons/f-eight-key.svg");
        theme: Themes.Info;
      }
      SIcon{
        height: 30px;
        width: 30px;
        source: @image-url("../../icons/pacifier.svg");
        theme: Themes.Warning;
      }
    }
  }
}
```

![image-20230913035043208](https://github.com/syf20020816/SurrealismUI/blob/main/README/imgs/image-20230913035043208.png)

### SCard
A very simple universal card without any layout or restrictions
you can add anything you want to the card

#### properties
- `in property <Themes> theme` : Surrealism Themes
- `in property <length> card-height `: card height 👍
- `in property <length> card-width` : card width 👍
- `in property <PaddingType> padding-type` : card padding size
- `in property <ShadowType> shadow` : card shadow type
- `in property <BorderType> border` : card border type
- `in-out property <PaddingProps> card-padding` : card padding 

#### example

```
import {SButton,SCard,SText} from "../../index.slint";
import {Themes} from "../../use/index.slint";

component TestCard inherits Window {
  height: 560px;
  width: 900px;
  background: #F5F5F5;
  
  SCard { 
    x:20px;
    y: 20px;
    card-width:text.width;
    text:=SText {
      text: "SCard";
    }
   }
   SCard { 
    x:400px;
    y: 20px;
    card-width:240px;
    card-height:120px;
    theme: Themes.Warning;
   }
   SCard { 
    x:20px;
    y: 240px;
    theme: Themes.Dark;
    card-width:240px;
    card-height:120px;
    border: X-Large;
   }

   SCard { 
    x:400px;
    y: 240px;
    theme: Themes.Primary;
    card-width:240px;
    card-height:120px;
    border: Large;
   }
}
```

![image-20230910103552426](https://github.com/syf20020816/SurrealismUI/blob/main/README/imgs/image-20230910103552426.png)

### SButton 
SButton is a button component that you can freely perform regular attribute operations on
#### properties (card + icon + text)
- `in property <image> icon `: Button icon
- `in property <length> font-size`: button font size
- `in property <length> letter-spacing `: button letter spacing
- `in property <bool> font-italic` : button font italic
- `in property <int> font-weight`: button font weight
- `in property <string> font-family`: button font family
- `in-out property <string> content` : the content of the button;
- `in property <bool> show-icon` : control  the icon load or not
#### functions
#### callbacks 
- `clicked` : run if you click the button

#### example

```
import {SButton} from "../../index.slint";
import {Themes,UseIcons} from "../../themes/index.slint";
component TestButton inherits Window {
  height: 400px;
  width: 400px;
  SButton {
    x: 20px;
    y: 10px;
    show-icon:true;
    theme:Themes.Dark;
    icon:@image-url("../../icons/safe-retrieval.svg");
    clicked => {
      self.content = "clicked"
    }
  }
  SButton {
    x: 260px;
    y: 10px;

    content:"Save";
    clicked => {
      self.content = "clicked";
      
    }
  }
  SButton {
    x: 20px;
    y: 100px;
    content:"Success";
    theme:Themes.Success;

  }
  SButton {
    x: 20px;
    y: 200px;
    content:"Primary";
    theme:Themes.Primary;
  }
  SButton {
    x: 20px;
    y: 300px;
    content:"Info";
    theme:Themes.Info;
  }
  SButton {
    x: 200px;
    y: 100px;
    content:"Error?";
    theme:Themes.Error;
    icon:@image-url("../../icons/magic-hat.svg");
  }
  SButton {
    x: 200px;
    y: 200px;
    theme:Themes.Warning;
  }
}
```

![image-20230913035128832](https://github.com/syf20020816/SurrealismUI/blob/main/README/imgs/image-20230913035128832.png)

 ### SInput

 This is a basic input box, often used in forms, divided into two types : text and password
 #### properties :
 - `in property <string> placeholder` : default placeholder which you wanna show when no content
 - `in property <Themes> theme` : Surrealism themes
 - `in property <int> font-weight` : font weight for input
 - `in property <length> input-width` : Please do not use width to adjust the length of the input box , use this property to instead
 - `in property <length> font-size` : font size 
 - `in property <bool> disabled` : can input be edited
 - `in property <bool> clearable` : can input be cleared
 - `in property <bool> password` : can the password input display the password
 - `out property <bool> has-focus ` : input is focused or not
 - `private property <brush> placeholder-color ` : placeholder color
 - `in-out property <InputType> type ` : input type (text or password)
 - `in-out property <brush> font-color ` : font color
 - `in-out property <brush> icon-color ` : icon color
 - `in-out property <string> content ` : the content of the input
 #### functions :
 - `pure public function get() ->string ` : get content
 - `public function set(content` :string) ` : set content
- `public function clear()` : clear content
- `public function select-all()` : select all 
- `public function clear-selection()` : clears the selection
- `public function cut()` : copies the selected text to the clipboard and removes it from the editable area
- `public function copy() `: copies the selected text to the clipboard
* `public function paste()` : pastes the text content of the clipboard at the cursor position
 #### callbacks :
 - `callback accepted(string) ` : run when pressed down Enter key
 - `callback changed(string) ` : run when content changed

#### example

```slint
import {SText,SInput,SButton, SIcon,SPopup} from "../../index.slint";
import {Themes} from "../../use/index.slint";
import { TextEdit , LineEdit} from "std-widgets.slint";
import { Invoke } from "./invoke_input.slint";

export component TestInput inherits Window {
  height: 500px;
  width: 600px;
  p:=SPopup {
    Invoke {}
  }
  SInput{
    y: 20px;
    width: 60%;
    placeholder :"please enter your username";
    input-width:300px;
    clearable: true;
    accepted(res)=>{
      debug("content in input:" + res);
      p.open();
    }
    changed(change-res)=>{
      debug(change-res);
    }
    
  }
 
  w:=SInput{
    y: 80px;
    width: 60%;
    theme:Themes.Success;
    type:InputType.password;
    password:true;
  }
  SInput{
    y: 140px;
    width: 60%;
    theme:Themes.Error;
    disabled:true;
    content:"disabled";
  }
  SInput{
    y: 200px;
    width: 60%;
    theme:Themes.Dark;
  }

  SInput{
    y: 260px;
    width: 60%;
    theme:Themes.Warning;
    clearable:true;
  }
  SInput{
    y: 320px;
    // width: 60%;
    theme:Themes.Info;
    type:InputType.password;
    clearable:true;
    password:true;
  }
  
}
```

![image-20231105000702807](https://github.com/syf20020816/SurrealismUI/blob/main/README/imgs/image-20231105000702807.png)

 ### SStar
 SStar is a scoring component
 #### properties
 - `in property <bool> no-theme` : use Surrealism Theme or not
 - `in property <float> score` : the real score
 - `in property <Themes> theme` : Themes.Primary;
 - `in property <bool> disabled` : can be scored if disabled is false
 - `in property <float> max-score` : max score (how many stars you wanna show)
 #### functions
 - `pure function get-half-stars()->bool `: count the number of half stars ⛔
 - `pure function get-whole-stars()->int` : count the number of whole stars ⛔
 - `pure function get-empty-stars()->int` : count the number of empty stars ⛔
 - ` public function full()` : star all 👍
 - `public function clear()` : no star 👍
 - `public function add-one()` : add one star 👍
 - `public function add-half()` : add half stars 👍
 #### callbacks
 - `callback clicked(float,float)` : get how many whole stars and half stars

#### example

```
import {SStar,SButton} from "../../index.slint";
import {Themes,UseIcons} from "../../themes/index.slint";

component TestWindow inherits Window {
  height: 400px;
  width: 400px;
  SStar {
    y: 20px;
  }
  hs:=SStar {
    score: 2.2;
    y: 60px;
    theme: Error;
    
  }
  SButton {
    y: 320px;
    x:10px;
    text: "add half";
    clicked => {
      hs.add-half();
    }
  }
  SStar {
    score : 3.8;
    disabled: true;
    y: 100px;
    theme: Success;
  }
  os:=SStar {
    max-score : 7;
    score : 2.8;
    y: 140px;
    theme: Info;
  }
  SButton {
    y: 320px;
    x: 115px;
    text: "add one";
    clicked => {
      os.add-one();
    }
  }
  fs:=SStar {
    max-score : 10;
    score : 7.2;
    y: 180px;
    no-theme:true;
    clicked(whole,half) => {
      t.n = whole;
      t.m = half;
    }
  }
  SButton {
    y: 320px;
    x: 220px;
    text: "get A";
    clicked => {
      fs.full();
    }
  }
  SButton {
    y: 320px;
    x: 305px;
    text: "clear";
    clicked => {
      fs.clear();
    }
  }
  t:=Text{
    y: 250px;
    font-size: 18px;
    in-out property <int> n;
    in-out property <int> m;
    text: "whole stars:"+ n + " half stars:" + m;
  }
}
```

![image-20230910105550811](https://github.com/syf20020816/SurrealismUI/blob/main/README/imgs/image-20230910105550811.png)

 ### STag
 A small tag used to display data
 #### properties
 - `in property <string> text` : the text of the tag
 - `in property <brush> font-color` : tag font color
 - `in property <length> font-size` : tag font size
 #### functions
 see card's functions
 #### callbacks
 - `callback clicked(string)` : run if you click the tag

#### example

```
import {STag} from "../../index.slint";
import {Themes} from "../../use/index.slint";

component TestWindow inherits Window {
  height: 400px;
  width: 400px;
  STag {
    text:"default";
    y: 40px;
  }
  STag {
    text:"error!";
    y:80px;
    theme:Themes.Error;
  }
  STag {
    text:"callback";
    y:120px;
    theme:Themes.Dark;
    clicked(text)=>{
      self.font-color= #ddff00;
    }
  }
  STag {
    text:"success";
    y:160px;
   
    theme:Themes.Success;
  }
}
```

![image-20231112134533009](https://github.com/syf20020816/SurrealismUI/blob/main/README/imgs/image-20231112134533009.png)

 ### SHeader
 SHeader is a simple header component that is generated based on routing information
 #### properties
 - `in property <length> spacing` : spacing of the header ⛔
 - `in property <Route> route` : detail routes , like:`{home:"Surrealism",routes:["user","info"]};`
 - `in property <length> font-size` : font size
 #### functions
 #### callbacks
 - `callback to(int,string)` : to page (it depends on you)
 - `callback back()` : back to main page (it depends on you)

#### example

```
import {SHeader,Route} from "../../index.slint";
import {Themes} from "../../use/index.slint";

component TestWindow inherits Window {
  height: 400px;
  width: 660px;
  SHeader {
    x:10px;
    y: 40px;
  }
  SHeader {
    x:10px;
    y: 100px;
    theme: Error;
    route:{
      home:"slint",routes:["components","dist","v1.2.0"]
    };
  }
  SHeader {
    x:10px;
    y: 160px;
    theme: Primary;
    to(index,route)=>{
      txt.name = route;
      txt.index = index;
    }
    back=>{
      txt.name = "back";
    }
  }
  txt:=Text{
    y: 260px;
    font-size: 18px;
    in-out property <int> index;
    in-out property <string> name;
    text: "route-index:" + index + " route-name:" + name;
  }
}
```

![image-20230910105709278](https://github.com/syf20020816/SurrealismUI/blob/main/README/imgs/image-20230910105709278.png)

 ### STable
This is the outter of the Table, and the column data of the table is separated from the outter
The outter only serves as a standard layout , this is a zero cost construction

 #### properties
 - see SCard
 #### functions
 - see SCard
 #### callbacks
 - see SCard

### STableColumn
STableColumn is a component of STable, and each STableColumn forms a complete column of the table
If it's gone, the table will become a card with a horizontal layout
#### properties
- `in property <bool> border` : add border or not
- `in property <string> name` : table header name
- `in property <[string]> datas` : table datas
- `in property <brush> header-background` : define header background
- `in property <brush> row-background` : define row background
- `in property <Themes> theme` : Surrealism Themes
- `in property <length> header-height `: define header height
- `in property <length> row-height` : define each row height
- `in property <bool> operation-enabled`: enable operation
- `in property <[{name:string,theme:Themes}]> operation` : the operations you wanna do
#### functions
- `function count() ->int` : count the number of row ⛔
- `pure public function get-height()->length` : auto count the height of the table and return height
#### callbacks
- `callback clicked(int,string)` : run if operation-enabled is true , you will get which operation button you clicked

#### example

```
import {STable,STableColumn} from "../../index.slint";
import {Themes} from "../../use/index.slint";

export component TestTable inherits Window {
  height: 500px;
  width: 500px;
  t1:=STable {

    y: 10px;
    // you can use this way to get height
    // it depends on how many datas in column
    height: col1.get-height();
    width: 300px;
    
    col1:=STableColumn {
      border:false;
      theme:Themes.Error;
      width: 100px;
      name:"id";
      // row-height:60px;
      datas: ["101","102","103"];
    }
    STableColumn {
      theme:Themes.Error;
      width: 100px;
      name:"name";
      datas: ["Mat","Jarry","Kaven"];
    }
    STableColumn {
      theme:Themes.Error;
      width: 100px;
      name:"age";
      datas: ["16","23","18"];
    }
  }
  t2:=STable {

    y: t1.height + 40px;
    // you can use this way to get height
    // it depends on how many datas in column
    height: tcol1.get-height();
    width: 350px;
    
    tcol1:=STableColumn {
      border:false;
      theme:Themes.Primary;
      width: 100px;
      name:"id";
      // row-height:60px;
      datas: ["101","102","103"];
    }
    STableColumn {
      theme:Themes.Primary;
      width: 100px;
      name:"name";
      datas: ["Mat","Jarry","Kaven"];
    }
    STableColumn {
      theme:Themes.Primary;
      width: 150px;
      name:"Operations";
      // cheat datas
      datas: [" "," "," "];
      operation-enabled:true;
    }
  }
}
```

![image-20230910105946372](https://github.com/syf20020816/SurrealismUI/blob/main/README/imgs/image-20230910105946372.png)

### SCollapse
SCollapse is a foldable panel

This is the outter of the Collapse, what really works is SCollapseItem

The outter only serves as a standard layout , this is a zero cost construction

#### properties
- see SCard
#### functions
- see SCard
#### callbacks
- see SCard

### SCollapseItem
SCollapseItem is a component of SCollapse, without which SCollapse will not work
You can customize the components or use the default text display method in it
#### properties
- `in property <length> item-height`: set height of detail
- `in property <string> name` : collapse header;
- `in property <string> detail` : the content of detail
- `in property <bool> define` : define detail or not (if you wanan show something special use true!)
- `in property <Themes> theme` : Surrealism Themes
- `private property <bool> show` : show details or not ⛔
#### functions
- `pure public function get-height()->length`: get collapse header height
#### callbacks
- `callback clicked()` : run if you show collapse detail

#### example

```
import {SCollapse,SCollapseItem,SButton,STable,STableColumn} from "../../index.slint";
import {Themes,UseIcons} from "../../themes/index.slint";


component TestWindow inherits Window {
  height: 500px;
  width: 400px;
  SCollapse {
    y: 10px;
    // you can set 0 , it has no impact
    // recommend use the following way
    height: item1.get-height() * 2;
    width: 360px;
    theme: Primary;
    item1:=SCollapseItem {
      name:"Feedback";
      detail:" Operation feedback: enable the users to clearly perceive their operations by style updates and interactive effects";
      
    }
    SCollapseItem {
      theme: Themes.Error;
      define:true;
      SButton { 

      }
    }
    SCollapseItem {
      name:"table";
      theme: Themes.Dark;
      define:true;
      item-height:280px;
      STable {
        
        height: col1.get-height();
        width: 300px;
        col1:=STableColumn {
          border:false;
          theme:Themes.Error;
          width: 100px;
          name:"id";
          // row-height:60px;
          datas: ["101","102","103"];
        }
        STableColumn {
          theme:Themes.Success;
          width: 100px;
          name:"name";
          datas: ["Mat","Jarry","Kaven"];
        }
        STableColumn {
          theme:Themes.Error;
          width: 100px;
          name:"age";
          datas: ["16","23","18"];
        }
      }
    }
  }
}
```

![image-20230910110027145](https://github.com/syf20020816/SurrealismUI/blob/main/README/imgs/image-20230910110027145.png)

### SResult
SResult helps you easily build a quick prompt , you can build it in popup window
#### properties
- `in property <length> icon-size`: icon size
- `in-out property <string> btn `: the content of the button
- `in-out property <string> content` : content of the result
- `in property <ResultType> res-type` : Result type
- `in-out property <Icons> icon`: Icon of the result
#### functions
#### callbacks
- `callback clicked()` : run if you click the button

```
import {SResult} from "../../index.slint";
import {Themes,ResultType} from "../../themes/index.slint";

export component TestResult inherits Window {
  height: 500px;
  width: 800px;
  SResult {
    x: 10px;
    y: 10px;
  }
  SResult {
    x: 220px;
    y: 10px;
    res-type:ResultType.Primary;
  }
  SResult {
    x: 220px;
    y: 260px;
    res-type:ResultType.Info;
  }
  SResult {
    x: 10px;
    y: 260px;
    res-type:ResultType.Warning;
  }

  SResult {
    x: 440px;
    y: 10px;
    res-type:ResultType.Error;
  }
  SResult {
    x: 440px;
    y: 260px;
    res-type:ResultType.Help;
  }
}
```

![image-20230910110056779](https://github.com/syf20020816/SurrealismUI/blob/main/README/imgs/image-20230910110056779.png)

### SSelect
SSelect is a selector that provides three types of optional input parameter values
#### properties
- `in property <Themes> theme` : Surrealism Themes
- `in property <[{id:int,label:string,value:string}]> ranges-string` : select list range (type string)
- `in property <[{id:int,label:string,value:int}]> ranges-int` :  select list range (type int)
- `in property <[{id:int,label:string,value:float}]> ranges-float` :  select list range (type float)
- `in property <string> placeholder` : placeholder of the select
- `private property <brush> input-color` : the color of the select content ⛔
- `private property <bool> open` : open the select list or not ⛔
- `private property <int> range-type` : the type of the range value ⛔
#### functions
- `pure public function count-width(len:length)->length` : auto count the width of the select
#### callbacks
- `callback changed(int,int,string,string,ValueType)` : run if you choose an item of list

#### example

```
import {SSelect,ValueType} from "../../index.slint";
import {Themes} from "../../use/index.slint";

component TestWindow inherits Window {
  height: 440px;
  width: 400px;
  SSelect {
    y: 20px;
    ranges-string: [
      {id:0,label:"Shangai",value:"s01"},
      {id:1,label:"Los Angeles",value:"l02"},
      {id:2,label:"New York",value:"n03"},
      {id:3,label:"Hong Kong",value:"h04"},
    ];
  }
  SSelect {
    y: 200px;
    theme: Error;
    ranges-float: [
      {id:0,label:"Shangai",value:0.1},
      {id:1,label:"Los Angeles",value:0.2},
      {id:2,label:"New York",value:0.3},
      {id:3,label:"Hong Kong",value:0.4},
    ];
    changed(index,id,label,value,value-type)=>{
      if(value-type==ValueType.String){
        t.vt = "string";
      }else if(value-type==ValueType.Float){
        t.vt = "float"
      }else{
        t.vt = "int"
      }
      t.index = index;
      t.id = id;
      t.label = label;
      t.value = value;
    }
  }
  t:=Text{
    y: 400px;
    font-size: 16px;
    in-out property <int> index;
    in-out property <int> id;
    in-out property <string> label;
    in-out property <string> vt;
    in-out property <string> value;
    text: @tr("Index:{} Id:{} Label:{} Value:{} ValueType:{}",index,id,label,value,vt);
  }
}
```

![image-20230910110204450](https://github.com/syf20020816/SurrealismUI/blob/main/README/imgs/image-20230910110204450.png)

### SLink

SLink is commonly used to represent text connections or sharing

#### properties

* `in property <image> icon` : share icon you can use whatever you want
* `in property <bool> funny` : Easter egg just funny
* `in property <bool> underline` : has underline
* `out property <bool> has-hover` : link has been hover or not
* `in property <MouseCursor> mouse-cursor `: mouse cursor
* `in property <Themes> theme` : Surrealism Theme
* `in property <length> font-size` : link font size
* `in-out property <string> text` : link text
* `private property <brush> text-color`: text color⛔

#### callbacks

* `callback clicked(string)` :  run if you click share icon

#### exeample

```
import {SLink} from "../../index.slint";
import {Themes,UseIcons} from "../../themes/index.slint";

component TestWindow inherits Window {
  height: 420px;
  width: 400px;
  
  SLink {
    y: 100px;
    theme: Dark;
    text: "no underline";
    underline: false;
  }
  SLink {
    y: 160px;
    funny:true;
    theme: Warning;
    text: "funny for link!";
  }
  SLink {
    y: 220px;
    theme: Primary;
    icon: @image-url("../../icons/share-one.svg");
    text: "share one";
  }
  SLink {
    y: 280px;
    funny:true;
    theme: Error;
    icon : @image-url("../../icons/share-sys.svg");
    font-size: 24px;
    text: "share sys";
    clicked=>{
      debug("share sys!")
    }
  }
}
```

![image-20231112134443536](https://github.com/syf20020816/SurrealismUI/blob/main/README/imgs/image-20231112134443536.png)

### SURAvatar

SURAvatar is a avatar component that defaults to Icons.Avatar when there are no images available

#### properties

* `in property <length> avatar-size` : avatar size
* `in property <image> avatar` : avatar image

#### example

```
import {SURAvatar} from "../../index.slint";
import {Themes,UseIcons,ROOT-STYLES} from "../../themes/index.slint";

component TestWindow inherits Window {
  height: 400px;
  width: 400px;
  background: #F5F5F5;
  SURAvatar {
    x: 20px;
    y: 100px;
  }
  SURAvatar {
    x:20px;
    y: 200px;
    avatar-size : ROOT-STYLES.sur-size.small * 2;
    padding-type : Small;
    theme: Primary;
  }
  SURAvatar {
    x: 200px;
    y: 100px;
    theme: Warning;
  }
  SURAvatar {
    x: 200px;
    y: 200px;
    avatar-size : ROOT-STYLES.sur-size.small * 2;
    padding-type : Small;
    theme: Error;
  }
  SURAvatar {
    y: 300px;
    avatar-size : ROOT-STYLES.sur-size.large * 2;
    padding-type : Large;
    theme: Dark;
    avatar:@image-url("../.https://github.com/syf20020816/SurrealismUI/blob/main/README/imgs/logo.png");
  }
  
}
```

![image-20230910110253626](https://github.com/syf20020816/SurrealismUI/blob/main/README/imgs/image-20230910110253626.png)

### SRadio

Radio let people select a single item

#### properties (card)

* `in-out property <bool> has-clicked` : the radio is clicked or not
* `in-out property <brush> active-color`: radio activecolor

#### functions

#### callbacks

* `callback clicked()` : run if you click the radio

#### example

```
import {SRadio} from "../../index.slint";
import {Themes} from "../../use/index.slint";

component TestCollection inherits Window {
  height: 560px;
  width: 600px;
  
  SRadio{
    y: 60px;
  }

  SRadio{
    y: 180px;
    active-color : #4affae;
    theme:Primary;
  }
}
```

![image-20230912155049511](https://github.com/syf20020816/SurrealismUI/blob/main/README/imgs/image-20230912155049511.png)

### SPopup

A masked pop-up layer appears in the current window

And users will not be able to use the pop-up layer to cover the components under it. Clicking on the pop-up layer again will close it

#### properties

* `in-out property <bool> is-show` : the popup layer is show or not
* `in property <Themes> theme` : Surrealism Themes

#### functions

- `public function open()` : open the popup

- `public function close()` : close the popup

#### callbacks

#### example

```
import {SPopup,SButton} from "../../index.slint";
import {Themes} from "../../use/index.slint";

component TestPopup inherits Window {
  height: 800px;
  width: 800px;
  background: #535353;
 
  SButton {
    
    text: "show";
    clicked => {
      p.open();
      
      debug("sds1")
    }
  }
 

  p:=SPopup {
    SButton {
      text: "you can add anything in Popup";
      y: 160px;
    }
  }
}
```

![image-20230912155117323](https://github.com/syf20020816/SurrealismUI/blob/main/README/imgs/image-20230912155117323.png)

### SDivider

A divider groups sections of content to create visual rhythm and hierarchy. 

Use dividers along with spacing and headers to organize content in your layout. 

#### properties

* `in property <string> content `: divider content
* `in property <image> icon` : divider icon
* `in property <Themes> theme` : Surrealism Theme

#### functions

* `function show-what()->int` : show icon or content ⛔

#### example

```
import {SDivider} from "../../index.slint";
import {Themes,UseIcons} from "../../themes/index.slint";

component TestDivider inherits Window {
  height: 400px;
  width: 400px;
  background: #535353;
  
  SDivider {
    y: 60px;
    width: 380px;
  }
  SDivider {
    y: 120px;
    width: 380px;
    icon:@image-url("../../icons/nail-polish-one.svg");
    theme:Themes.Error;
  }
  SDivider {
    y: 180px;
    width: 380px;
    icon:@image-url("../../icons/earth.svg");
    theme:Themes.Dark;
  }
  SDivider {
    y: 240px;
    width: 380px;
    content:"";
    theme:Themes.Light;
  }
}
```

![image-20230913035535856](https://github.com/syf20020816/SurrealismUI/blob/main/README/imgs/image-20230913035535856.png)

### SCollection

SCollection is a grid storage box, but in reality it is not based on grid layout.

It achieves a flexible grid through a combination of dual for loops and horizontal and vertical layouts

Clicking on the pop-up layer again will close it

#### properties (card)

* `in property <length> font-size` : font size
* `in property <int> column-num` :  column number
* `in property <int> row-num` : row number
* `in-out property <[[CollectionData]]>` data : collection data , this is the real data!
* `in property <length> row-height` : row height
* `in property <length> column-width` : column width
* `in property <length> row-spaceing` : row spaceing
* `in property <length> column-spacing`: column spacing

#### functions

#### callbacks

* `clicked(CollectionData)` : run if you click item in SCollection

```
import {SButton,SCollection} from "../../index.slint";
import {Themes} from "../../use/index.slint";

component TestCollection inherits Window {
  height: 560px;
  width: 900px;
  
  SCollection{
    card-height: 300px;
    card-width: 300px;
    column-num: 3;
    font-size : 16px;
    theme: Dark;
    data: [
      [
        {id:0,name:"box1",source:@image-url("./collection_imgs/box1.svg")},
        {id:1,name:"box2",source:@image-url("./collection_imgs/box2.svg")},
        {id:2,name:"box3",source:@image-url("./collection_imgs/box3.svg")}
      ],
      [
        {id:3,name:"box4",source:@image-url("./collection_imgs/box4.svg")},
        
      ],
      [
        {id:4,name:"box6",source:@image-url("./collection_imgs/box6.svg")},
        {id:5,name:"box7",source:@image-url("./collection_imgs/box7.svg")},
      ]
    ];
    clicked(item)=>{
      debug(item.name);
      debug(item.id);
    }
  }
  
}
```

![image-20230912155404367](https://github.com/syf20020816/SurrealismUI/blob/main/README/imgs/image-20230912155404367.png)

### SPersona
This component is used to display simple user introduction information
#### properties (card)
- `in property <string> name `: person name
- `in property <string> des `: person description
- `in property <string> btn `: click button content
- `in property <image> avatar `: avatar image
- `in property <length> name-font-size `: name font size
- `in property <length> des-font-size`: des font size
- `in property <length> avatar-height`: avatar height
- `in property <length> name-height`: name height
- `in property <length> des-height`: des height
- `in property <Themes> avatar-theme `: avatar theme
- `in property <Themes> name-theme`: name theme
- `in property <Themes> des-theme`: des theme
- `in property <Themes> btn-theme`: btn theme
#### functions
#### callbacks
- `callback clicked() `: run if you click the target button

#### example

```
import {SPersona} from "../../index.slint";
import {Themes} from "../../use/index.slint";

component TestCollection inherits Window {
  height: 700px;
  width: 400px;
  
  SPersona {
    y: 10px;
  }
  SPersona {
    y: 350px;
    btn-theme:Dark;
    theme:Themes.Dark;
    name-theme:Themes.Dark;
    des-theme:Themes.Light;
    avatar : @image-url("../.https://github.com/syf20020816/SurrealismUI/blob/main/README/imgs/logo.png");
    avatar-height:160px;
    card-height: 310px;
    clicked=>{
      debug("view page!")
    }
  }
}
```

![image-20230916001748114](https://github.com/syf20020816/SurrealismUI/blob/main/README/imgs/image-20230916001748114.png)

 ### SBadge
 SBadge is a quick way to display user status or events
 #### properties (card)
 - `in property <Position> position` : where the badge show
 - `in-out property <image> icon` : icon of the badge
 - `in property <brush> icon-color` : icon color
 - `in property <brush> font-color` : font color
 - `in property <ResultType> res-type` : icon Type see result!(but you can define without use this property)
 #### functions
 - `pure public function get-x(p_right:length)->length` 👍
 - `pure public function get-y(p_bottom:length)->length` 👍

 #### callbacks

#### example

```
import {SBadge,SURAvatar} from "../../index.slint";
import {Themes} from "../../use/index.slint";

component TestCollection inherits Window {
  height: 460px;
  width: 400px;
  
  b1:=Rectangle {
    
    y: 30px;
    height: avatar.height;
    width: avatar.width;
    avatar:=SURAvatar {
    
    } 
    SBadge {
      x: self.get-x(avatar.width);
      y: self.get-y(avatar.height);
    }
  }
  b2:=Rectangle {
    
    y: 120px;
    height: avatar2.height;
    width: avatar2.width;
    avatar2:=SURAvatar {
    } 
    SBadge {
      x: self.get-x(avatar2.width);
      y: self.get-y(avatar2.height);
      position: Left-Bottom;
      res-type: Success;
    }
  }
  b3:=Rectangle {
   
    y: 210px;
    height: avatar3.height;
    width: avatar3.width;
    avatar3:=SURAvatar {
    
    } 
    SBadge {
      x: self.get-x(avatar3.width);
      y: self.get-y(avatar3.height);
      position: Left-Top;
      res-type: Warning;
      icon-color:#ff0000;
      font-color:#ff0000;
    }
  }
  b4:=Rectangle {
    y: 300px;
    height: avatar4.height;
    width: avatar4.width;
    avatar4:=SURAvatar {
    } 
    SBadge {
      x: self.get-x(avatar4.width);
      y: self.get-y(avatar4.height);
      position: Right-Top;
      res-type: Error;
    }
  }
  
}
```

![image-20230916001834136](https://github.com/syf20020816/SurrealismUI/blob/main/README/imgs/image-20230916001834136.png)

### SProgress
SProgress is commonly used to display download progress or event processing progress
And you can fully control it through the progress property

#### properties
- `in property <Themes> theme` : Surrealism theme
- `in property <string> content` : what you wanna show to others
- `in-out property <float> progress` : progress
- `private property <length> unit` : unit of progress length
#### functions
- `pure public function get-progress()` : get timely progress
- `public function full()` : make progress 100%
- `public function clear()` : : make progress 0%
- `public function add(num:float)` : increase progress

#### callbacks

```
import {SProgress,SButton} from "../../index.slint";
import {Themes} from "../../use/index.slint";

component TestDivider inherits Window {
  height: 400px;
  width: 400px;
  background: #5b64cd;
  SProgress {
    y: 100px;
  }
  a:=SProgress {
    y: 200px;
    theme:Primary;
  }
  SProgress {
    y: 300px;
    theme:Dark;
    progress:86;
  }
  SButton{
    x: 60px;
    y: 340px;
    text: "add";
    clicked => {
      a.add(5);
    }
  }
  SButton{
    x: 160px;
    y: 340px;
    text: "full";
    clicked => {
      a.full();
    }
  }
  SButton{
    x: 260px;
    y: 340px;
    text: "clear";
    clicked => {
      a.clear();
    }
  }
}
```

![image-20230916001718588](https://github.com/syf20020816/SurrealismUI/blob/main/README/imgs/image-20230916001718588.png)

### STip

A tip provides supplemental, contextual information elevated near its target component

#### properties

* `in property <Themes> theme` : Surrealism Theme
* `in property <string> content` : tip content
* `in-out property <bool> show-tip` : hover and tip will show
* `in property <TipPosition> pos` : the position of the tip

#### functions

* `public function open()` : open the tip
* `public function close()` : close the tip

#### callbacks

* `callback clicked()` : use to open|close the tip

#### example

```
import {STip,SButton } from "../../index.slint";
import {Themes} from "../../use/index.slint";


component TestWindow inherits Window {
  height: 400px;
  width: 400px;
  STip{
    y: 80px;
     height:inner0.height;
     width: inner0.width;
     theme: Dark;
     pos:Top;
     content:"this is a \n........tip window";
     show-tip:inner0.has-hover;
     inner0:=SButton { 
       content: "click";
     }
   }
  STip{
    height:inner.height;
    width: inner.width;
    theme: Dark;
    pos:Left;
    content:"this is a \n........tip window";
    inner:=SButton { 
      content: "click";
      clicked => {
        parent.clicked();
      }
    }
  }
  STip{
   y: 300px;
    height:inner2.height;
    width: inner2.width;
    theme: Dark;
    pos:Top;
    content:"this is a \n........tip window";
    inner2:=SButton { 
      content: "click";
      clicked => {
        parent.clicked();
      }
    }
  }
 
}
```

![image-20230930183024974](https://github.com/syf20020816/SurrealismUI/blob/main/README/imgs/image-20230930183024974.png)

### SLoading (some error in animation < version V0.1.6)

This is a loading component that you can embed anywhere you want to add a loading animation (now animation have some error)

#### properties

* `in-out property <bool> is-show` : the popup layer is show or not
* `in property <Themes> theme` : Surrealism Themes
* `in property <image> loading-icon `: loading icon
* `in property <duration> duration `: animation duration
* `in property <bool> an` : open animation or not (error : https://github.com/slint-ui/slint/issues/3494) (solve in V0.1.6)
* `in property <string> content` : loading content

#### functions

#### callbacks

* `callback open() `: open the loading
* `callback close() `: close the loading

#### example

```
import {SLoading,SButton,SCard} from "../../index.slint";

export component TestLoading inherits Window {
    height: 600px;
    width: 400px;
    SButton {
      y: 100px;
      text: "show";
      clicked => {
        p.open();
      }
    }
    SButton {
      y: 160px;
      text: "close";
      clicked => {
        p.close();
      }
    }
    SCard{
      y: 260px;
      clip: true;
      card-height: 260px;
      card-width: 180px;
      p:=SLoading { }
    }
}
```

![image-20230918220357041](https://github.com/syf20020816/SurrealismUI/blob/main/README/imgs/image-20230918220357041.png)

### SDialog
Dialogs are used to confirm messages or events and display content
#### properties
- `in property <string> dialog-title` : dialog title;
- `in property <length> dialog-title-size` : dialog title font size;
- `in property <string> dialog-details` : content information in the dialog box;
- `in property <Themes> cancel-btn-theme` : cancel button theme;
- `in property <Themes> confirm-btn-theme` : confirm button theme;
- `in property <string> cancel-btn-content` : cancel button content;
- `in property <string> confirm-btn-content` : confirm button content;
- `in-out property <bool> is-show` : show dialog or not;
- `in property <Themes> theme` : Surrealism Themes
- `in property <float> dialog-height` : Dialog height proportion
- `in property <float> dialog-width` :  Dialog width proportion
#### functions
- `public function open()` : open dialog
- `public function close()` : close dialog
#### callbacks
- `callback confirm()` : run after confirm button click
- `callback cancel()` : run after cancel button click

#### example

```
import {SDialog,SButton,STable,STableColumn} from "../../index.slint";
import {Themes} from "../../use/index.slint";

component TestDialog inherits Window {
  height: 800px;
  width: 800px;
  background: #535353;
 
  SButton {
    
    text: "show";
    clicked => {
      p.open();
    }
  }
 

  p:=SDialog {
    dialog-details : "";
    confirm-btn-theme: Success;
    dialog-width:80%;
    dialog-height:52%;
    // do after confirm btn clicked
    confirm=>{
      debug("confirm btn clicked~!")
    }
    STable {
      // you can use this way to get height
      // it depends on how many datas in column
      height: tcol1.get-height();
      width: 350px;
      
      tcol1:=STableColumn {
        border:false;
        theme:Themes.Primary;
        width: 100px;
        name:"id";
        // row-height:60px;
        datas: ["101","102","103"];
      }
      STableColumn {
        theme:Themes.Primary;
        width: 100px;
        name:"name";
        datas: ["Mat","Jarry","Kaven"];
      }
      STableColumn {
        theme:Themes.Primary;
        width: 150px;
        name:"Operations";
        // cheat datas
        datas: [" "," "," "];
        operation-enabled:true;
      }
    }
  }
}
```

![image-20230919091100568](https://github.com/syf20020816/SurrealismUI/blob/main/README/imgs/image-20230919091100568.png)

### SMenu
SMenu is a menu bar located on the left side that you can quickly generate through the menu-data property
#### properties
- `in-out property <length> icon-box-size` : menu item size ⛔
- `in-out property <length> icon-size` : menu item icon size ⛔;
- `in property <[MenuData]> menu-data` : menu item data (generate menus through it)
- `in-out property <int> active` : which item is active
- `private property <brush> hover-icon-color` : menu item icon color changed when hover
#### callbacks
- `callback change(int,MenuData)` : run if you click menu item
- `callback clicked-account()` : run if you click account icon
- `callback clicked-setting()` : run if you click setting icon

#### example

```
import { SMenu , SIcon} from "../../index.slint";
import {UseIcons} from "../../use/index.slint";

component TestMenu inherits Window {
    height: 600px;
    width: 300px;
    Rectangle {
      x: 0;
      y: 0;
      height:parent.height;
      width: menu.width;
      menu:=SMenu {
        theme: Dark;
        change(index,item)=>{
          debug(index);
          debug(item);
        }
        clicked-account()=>{
          debug("clicked account");
        }
      }
    }
}
```

![image-20230930181846475](https://github.com/syf20020816/SurrealismUI/blob/main/README/imgs/image-20230930181846475.png)

### SSwitch

SSwitch is a switch used for simple judgment scenarios

#### properties

* `in-out property <bool> active` : active option;
* `in property <brush> switch-background-color `: switch circle background color;
* `in property <brush> switch-border-color` : switch circle border color
* `in property <color> switch-drop-shadow-color` : switch circle drop shadow color

#### callbacks

* `callback clicked(bool)` : run if you click the switch

#### example

```
import { SSwitch } from "../../index.slint";

component TestSwitch inherits Window {
  height: 400px;
  width: 400px;
  SSwitch {
    y: 30px;
  }
  SSwitch {
    theme: Primary;
    y: 80px;
    switch-background-color:#ddd;
    switch-border-color:#00ff00;
  }
  SSwitch {
    y: 130px;
    theme: Dark;
    clicked(active-or-not)=>{
      debug(active-or-not);
    }
  }
  SSwitch {
    y: 180px;
    theme: Warning;
  }
  SSwitch {
    y: 230px;
    theme: Error;
  }
  SSwitch {
    y: 280px;
    theme: Info;
  }
}
```

![image-20231018185602735](https://github.com/syf20020816/SurrealismUI/blob/main/README/imgs/image-20231018185602735.png)

### SSwitchGroup

SSwitchGroup switch group can contain more switch cases

#### properties

* `in-out property <bool> active` : active option index;
* `in-out property <[string]> switchs` : switch options
* `in property <length> font-size` : font size , it will effect switch component height
* `private property <brush> theme-color` : inner theme color ⛔

#### callbacks

* `callback clicked(int,string)` : run if you click the switch , it will back option index and option name

#### example

```
import { SSwitchGroup } from "../../index.slint";

component TestSwitchGroup inherits Window {
  height: 400px;
  width: 400px;
  SSwitchGroup {
   theme: Primary;
    clicked(i,name) => {
      debug(i);
      debug(name);
    }
  }
  SSwitchGroup {
    y: 120px;
    theme:Dark;
    switchs:["1","2","3","4"];
     clicked(i,name) => {
       debug(i);
       debug(name);
     }
   }
}
```

![image-20231104201007098](https://github.com/syf20020816/SurrealismUI/blob/main/README/imgs/image-20231104201007098.png)

### SSwitchOption

SSwitchOption can show option info

#### properties

* `in-out property <bool> active` : active option;
* `in property <string> left` : left option;
* `in property <string> right` : right option;
* `in property <length> font-size` : font size , it will effect switch component height;
* `in property <brush> switch-background-color` : switch circle background color;
* `in property <brush> switch-border-color` : switch circle border color
* `in property <color> switch-drop-shadow-color` : switch circle drop shadow color

#### callbacks

* `callback clicked(bool)` : run if you click the switch

#### example

```
import { SSwitchOption } from "../../index.slint";

component TestSwitchOption inherits Window {
  height: 400px;
  width: 400px;
  SSwitchOption {
    y: 30px;
    left:"surrealism";
    right:"slint";
    clicked(res) => {
      debug(res)
    }
  }
  SSwitchOption {
    y: 100px;
    theme: Primary;
    left:"surrealism";
    right:"slint";
  }
  SSwitchOption {
    y: 170px;
    theme: Dark;
    left:"surrealism";
    right:"slint";
  }
}
```

![image-20231104201049848](https://github.com/syf20020816/SurrealismUI/blob/main/README/imgs/image-20231104201049848.png)

### SDrawer

Sometimes, the Dialogue component does not meet our needs

such as your form being too long, or if you need to temporarily display some documents, please use the SDrawer

#### properties

* `in property <Themes> drawer-theme` : drawer theme
* `in property <brush> drawer-background-color` : drawer background color
* `in property <CommonPosition > pos` : drawer position (Left, Right, Top, Bottom)
* `in property <percent> proportion` : drawer proportion 

#### functions

* `function default-height-width()->{height:percent,width:percent}` : count drawer height and width ⛔
* `function get-pos()->{x:length,y:length}` : count position ⛔

#### example

```
import {SDrawer,SButton, SInput} from "../../index.slint";
import {Themes} from "../../use/index.slint";

component TestDrawer inherits Window {
  height: 800px;
  width: 800px;
  background: #535353;
 
  SButton {
    
    content: "show";
    clicked => {
      p.open();
      
      debug("sds1")
    }
  }
 

  p:=SDrawer {
    proportion:36%;
    SButton {
      theme: Dark;
    }
    SInput { 
      y: 30px;
     }
  }
}
```

![image-20231018200348306](https://github.com/syf20020816/SurrealismUI/blob/main/README/imgs/image-20231018200348306.png)

### SAlert

SAlert is used to display important prompt information on the page

#### properties

* `private property <Themes> theme` : Surrealism theme ⛔
* `in-out property <string> content` :  alert content you want to display
* `in-out property <bool> is-show` : show the alert or not
* `in property <ResultType> res-type` : result type👍

#### functions

* `public function open()` : open alert
* `public function close()` : close alert

#### example

```
import {SButton, SAlert} from "../../index.slint";
import {Themes,ResultType} from "../../themes/index.slint";

component TestAlert inherits Window {
  height: 400px;
  width: 600px;
  background: #535353;
 
  SButton {
    
    text: "show";
    clicked => {
      p.open();
      
      debug("sds1")
    }
  }
 

  p:=SAlert { 
    res-type:ResultType.Success ;
    content:"this is a success message!";
  }
}
```

![image-20231018203824259](https://github.com/syf20020816/SurrealismUI/blob/main/README/imgs/image-20231018203824259.png)

### STree

STree can be used to display directory structure, forming a parent-child relationship, and can be easily displayed

#### properties

* `in-out property <TreeData> tree-data` : the data to be displayed

#### callbacks

* `callback clicked(int,string,string)` : run after you click an item

### example

```
import {STree } from "../../index.slint";


component TestTree inherits Window {
  height: 400px;
  width: 400px;
  STree{
    y: 10px;
    theme: Dark;
    height: 45%;
    width: 96%;
    tree-data:{
      icon : UseIcons.icons.Folder_filled,
      label: "SurrealismUI",
      extra:"",
      children:[
        {
          icon:UseIcons.icons.FileCode,
          label:"slint.slint",
          extra:"12KB", 
        },
        {
          icon:UseIcons.icons.FileCode,
          label:"surrealism.slint",
          extra:"126KB", 
        },
        {
          icon:@image-url("../../icons/file-jpg.svg"),
          label:"icon.jpg",
          extra:"196KB", 
        },
        {
          icon:@image-url("../../icons/file-gif.svg"),
          label:"ui.gif",
          extra:"91KB", 
        },
        {
          icon:@image-url("../../icons/file-gif.svg"),
          label:"ui2.gif",
          extra:"107KB", 
        }
      ]
    };
    clicked(i,n,e)=>{
      debug(n);
    }
  }
  STree {
    y: 200px;
    height: 46%;
    width: 96%;
  }
}
```

![image-20231105160644598](https://github.com/syf20020816/SurrealismUI/blob/main/README/imgs/image-20231105160644598.png)

### SFile

SFile can help users present file selectors GUI

#### properties

* `in property <[TabItem]> tabs` : tabs will be displayed 
* `in property <TabConfigs> tab-configs` : configurations of the tab
* `in-out property <[FileItem]> files` : files and folders details
* `in property <ItemConfigs> item-configs` : configurations of files and folders details

#### callbacks

* `callback tab-clicked(int,TabItem)` : run if you click the tab
* `callback item-clicked(int,FileItem)` : run if you click a file item

#### example

```
import {SFile,ItemConfigs,FileItem,ItemConfigs} from "../../index.slint";
import { Themes,PaddingType,UseIcons} from "../../themes/index.slint";

export component TestFile inherits Window {
  height: 400px;
  width: 800px;
  SFile{
    theme: Dark;
    width: 90%;
    height: 46%;
    tab-configs : {
      height:16px,
      font-size:14px,
      padding-type:PaddingType.Tip,
      theme: Themes.Dark,
      column-width:[200px,100px,100px,80px]
    };
    item-configs : {
      height:16px,
      font-size:12px,
      padding-type:PaddingType.Normal,
      theme: Themes.Dark,
      icon-size:16px
    };
    files : [
      {icon:UseIcons.icons.Folder-filled , name : "font" , datetime : "2023-11-06" , file-type : "folder" , size : "900KB"},
      {icon:UseIcons.icons.FileCode , name : "index.slint" , datetime : "2023-11-06" , file-type : "SLINT file" , size : "3KB"},
      {icon:UseIcons.icons.FileCode , name : "LICENSE" , datetime : "2023-11-06" , file-type : "file" , size : "2KB"},
      {icon:UseIcons.icons.FileCode , name : "LICENSE" , datetime : "2023-11-06" , file-type : "file" , size : "2KB"},
      {icon:UseIcons.icons.FileCode , name : "LICENSE" , datetime : "2023-11-06" , file-type : "file" , size : "2KB"}
    ];
    tab-clicked(index,item)=>{
      debug(index);
      debug(item);
    }
    item-clicked(index,item)=>{
      debug(index);
      debug(item);
    }
  }
}
```

![image-20231105160623486](https://github.com/syf20020816/SurrealismUI/blob/main/README/imgs/image-20231105160623486.png)

## Updates

- V0.3.0 (Slint 1.3.2)
  - 中文
    - 所有组件更名`SUR`为`S`
    - 默认文字采用`Alimama_DongFangDaKai_Regular.ttf` (若你不需要请注意release的包名)
    - 重写`SHeader`
    - 优化`STip`：
      - 修改属性名`content -> text`
    - 
  
- V0.2.2（Slint 1.3.0）
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

- V0.2.1
  - add `STree`
  - add `SFile`

- V0.2.0
  - add `SSwitchOption`
  - add `SSwitchGroup`
  - optimize `SInput`

- V0.1.7
  - add `SSwitch`
  - add `SDrawer`
  - add `SAlert`

- V0.1.6
  - solve `SLoading` animation!

- V0.1.5
  - add `SMenu`
  - enhance `STip` (the location of the tip can be changed now  and you can show it with hover ! )

- V0.1.4
  - add `STip`
  - add `SLoading`
  - add `SDialog`

- V0.1.3
  - add `SBadge`
  - add `Progress`
  - add `Persona`
- V0.1.2
  - rebuild components (have `SIcon`)
  - rebuild `SIcon`
  - rebuild file structure
  - solve memery overflow issue
  - use minimize import principle (remove inner loop to judge component show!)❗
  - test use Rust✅
- V0.1.1
  - add `SRadio`
  - add `SDivider`
  - add `SCollection`
  - add `SPopup`
- V0.1.0
  - Adopting Fluent2's component design style
  - Multiple default methods are provided for consumers to call (see index.slint which on the outermost side)
  - Decoupling functions and components
  - Fix some style errors
  - add `SLink` and `SURAvatar`
