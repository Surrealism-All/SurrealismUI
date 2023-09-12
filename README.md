<img src="https://img.shields.io/badge/SurrealismUI-0.1.1-orange?style=flat-square&logo=rust&logoColor=%23fff&labelColor=%23DEA584&color=%23DEA584">  <img src="https://img.shields.io/badge/License-MIT-orange?style=flat-square&logoColor=%23fff&labelColor=%2323B898&color=%2323B898">

# SurrealismUI

- author：syf20020816@outlook.com
- createDate：20230908
- updateDate：20230910
- version：0.1.1
- email：syf20020816@outlook.com

<img src="./README/imgs/logo.png">

SurrealismUI是一个完全使用Slint进行构建的Slint第三方组件库

SurrealismUI is a third-party component library built entirely using Slint

## About Doc Icon

- ⛔ ： do not use
- 👍 ： Recommended use

### Updates

- V0.1.2
  - rebuild components (have `SURIcon`)
  - rebuild `SURIcon`
  - rebuild file structure
  - solve memery overflow issue
  - use minimize import principle (remove inner loop to judge component show!)❗
  - test use Rust✅

- V0.1.1
  - add `SURRadio`
  - add `SURDivider`
  - add `SURCollection`
  - add `SURPopup`

- V0.1.0
  - Adopting Fluent2's component design style
  - Multiple default methods are provided for consumers to call (see index.slint which on the outermost side)
  - Decoupling functions and components
  - Fix some style errors
  - add `SURLink` and `SURAvatar`

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

#### primary

![image-20230910102452817](./README/imgs/image-20230910102452817.png)

#### success

![image-20230910102504405](./README/imgs/image-20230910102504405.png)

#### info

![image-20230910102558381](./README/imgs/image-20230910102558381.png)

#### warning

![image-20230910102611556](./README/imgs/image-20230910102611556.png)

#### error

![image-20230910102624332](./README/imgs/image-20230910102624332.png)

#### dark

![image-20230910102637280](./README/imgs/image-20230910102637280.png)

#### light

![image-20230910102413761](./README/imgs/image-20230910102413761.png)

## Components

 ### SURText
 It is the simplest and most common component in SurrealismUI
 #### properties:
 - `in property <Themes> theme` : Surrealism themes
 - `in-out property <string> content` : the content in SURText
 #### callbacks: 
 #### functions:
 - `pure public function get()->string` : get content
 - `public function set(content:string)` : set content

#### example

```
import {SURText} from "../../index.slint";
import {Themes} from "../../themes/index.slint";

component TestWindow inherits Window {
  height: 400px;
  width: 400px;
  background:#fff;
  SURText {
    x: 100px;
    y: 20px;
    content: "hello world";
  }
  SURText {
    x:100px;
    y:100px;
    theme:Themes.Error;
  }
 
}
```

![image-20230910102940392](./README/imgs/image-20230910102940392.png)

 ### SURIcon
 there are 2658 different icons in SURIcon from : https://github.com/bytedance/iconpark
 #### properties:
 - `in property <image> icon` : icon types
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
import {SURIcon} from "../../index.slint";
import {IconSources,Size,Themes} from "../../themes/index.slint";
export component TestIcon inherits Window {
  height: 400px;
  width: 400px;
  GridLayout {
    spacing: 40px;
    Row{
      SURIcon{
        height: 30px;
        width: 30px;
        icon: @image-url("../../icons/sd-card.svg");
        theme: Themes.Primary;
      }
      SURIcon{
        height: 30px;
        width: 30px;
        icon: @image-url("../../icons/add-computer.svg");
        theme: Themes.Success;
      }
      SURIcon{
        height: 30px;
        width: 30px;
        icon: @image-url("../../icons/yep.svg");
        theme: Themes.Error;
      }
      SURIcon{
        icon: @image-url("../../icons/t-shirt.svg");
        theme: Themes.Dark;
        height: 30px;
        width: 30px;
      }
      SURIcon{
        height: 30px;
        width: 30px;
        icon: @image-url("../../icons/video-conference.svg");
        theme: Themes.Info;
      }
      SURIcon{
        height: 30px;
        width: 30px;
        icon:@image-url("../../icons/vacation.svg");
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
      SURIcon{
        height: 30px;
        width: 30px;
        icon: @image-url("../../icons/cake-five.svg");
        theme: Themes.Primary;
      }
      SURIcon{
        height: 30px;
        width: 30px;
        icon: @image-url("../../icons/label.svg");
        theme: Themes.Success;
      }
      SURIcon{
        height: 30px;
        width: 30px;
        icon: @image-url("../../icons/wifi.svg");
        theme: Themes.Error;
      }
      SURIcon{
        icon: @image-url("../../icons/wallet-one.svg");
        theme: Themes.Dark;
        height: 30px;
        width: 30px;
      }
      SURIcon{
        height: 30px;
        width: 30px;
        icon: @image-url("../../icons/game-console.svg");
        theme: Themes.Info;
      }
      SURIcon{
        height: 30px;
        width: 30px;
        icon: @image-url("../../icons/qiyehao.svg");
        theme: Themes.Warning;
      }
    }
    Row{
      SURIcon{
        height: 30px;
        width: 30px;
        icon: @image-url("../../icons/scanning-two.svg");
        theme: Themes.Primary;
      }
      SURIcon{
        height: 30px;
        width: 30px;
        icon: @image-url("../../icons/oceanengine.svg");
        theme: Themes.Success;
      }
      SURIcon{
        height: 30px;
        width: 30px;
        icon: @image-url("../../icons/zoom-internal.svg");
        theme: Themes.Error;
      }
      SURIcon{
        icon: @image-url("../../icons/zip.svg");
        theme: Themes.Dark;
        height: 30px;
        width: 30px;
      }
      SURIcon{
        height: 30px;
        width: 30px;
        icon: @image-url("../../icons/f-eight-key.svg");
        theme: Themes.Info;
      }
      SURIcon{
        height: 30px;
        width: 30px;
        icon: @image-url("../../icons/pacifier.svg");
        theme: Themes.Warning;
      }
    }
  }
}
```

![image-20230913035043208](E:\Rust\try\surrealism-ui\README\imgs\image-20230913035043208.png)

### SURCard
A very simple universal card without any layout or restrictions
you can add anything you want to the card

#### properties
- `in property <Themes> theme` : Surrealism Themes
- `in property <length> card-height `: card height 👍
- `in property <length> card-width` : card width 👍
- `in property <PaddingSize> padding-size` : card padding size
- `in property <Shadows> shadow` : card shadow type
- `in property <Borders> border` : card border type
- `in-out property <PaddingItem> card-padding` : card padding 

#### example

```
import {SURButton,SURCard,SURText} from "../../index.slint";
import {Themes} from "../../themes/index.slint";

component TestCard inherits Window {
  height: 560px;
  width: 900px;
  background: #F5F5F5;
  
  SURCard { 
    x:20px;
    y: 20px;
    card-width:text.width;
    text:=SURText {
      content: "SURCard";
    }
   }
   SURCard { 
    x:400px;
    y: 20px;
    card-width:240px;
    card-height:120px;
    theme: Themes.Warning;
   }
   SURCard { 
    x:20px;
    y: 240px;
    theme: Themes.Dark;
    card-width:240px;
    card-height:120px;
    border: X-Large;
   }

   SURCard { 
    x:400px;
    y: 240px;
    theme: Themes.Primary;
    card-width:240px;
    card-height:120px;
    border: Large;
   }
}
```

![image-20230910103552426](./README/imgs/image-20230910103552426.png)

### SURButton 
SURButton is a button component that you can freely perform regular attribute operations on
#### properties (card + icon + text)
- `in property <image> icon `: Button icon
- `in property <length> font-size`: button font size
- `in property <length> letter-spacing `: button letter spacing
- `in property <bool> font-italic` : button font italic
- `in property <int> font-weight`: button font weight
- `in property <string> font-family`: button font family
- `in-out property <string> content` : the content of the button;
#### functions
#### callbacks 
- `clicked` : run if you click the button

#### example

```
import {SURButton} from "/index.slint";
import {Themes,IconSources} from "/themes/index.slint";
component TestButton inherits Window {
  height: 400px;
  width: 400px;
  SURButton {
    x: 20px;
    y: 10px;
    
    theme:Themes.Dark;
    icon:@image-url("../../icons/safe-retrieval.svg");
    clicked => {
      self.content = "clicked"
    }
  }
  SURButton {
    x: 260px;
    y: 10px;

    content:"Save";
    clicked => {
      self.content = "clicked"
    }
  }
  SURButton {
    x: 20px;
    y: 100px;
    content:"Success";
    theme:Themes.Success;

  }
  SURButton {
    x: 20px;
    y: 200px;
    content:"Primary";
    theme:Themes.Primary;
  }
  SURButton {
    x: 20px;
    y: 300px;
    content:"Info";
    theme:Themes.Info;
  }
  SURButton {
    x: 200px;
    y: 100px;
    content:"Error?";
    theme:Themes.Error;
    icon:@image-url("../../icons/magic-hat.svg");
  }
  SURButton {
    x: 200px;
    y: 200px;
    
    theme:Themes.Warning;
  }
}
```

![image-20230913035128832](E:\Rust\try\surrealism-ui\README\imgs\image-20230913035128832.png)

 ### SURInput

 This is a basic input box, often used in forms, divided into two types : text and password
 #### properties :
 - `in property <string> placeholder` : default placeholder which you wanna show when no content
 - `in property <Themes> theme` : Surrealism themes
 - `in property <image> icon` : icon you wanna show in front (use >= v0.1.0) ⛔
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
 - `pure public function count-width()->length ` : count input real width ⛔
 #### callbacks :
 - `callback accepted(string) ` : run when pressed down Enter key
 - `callback changed(string) ` : run when content changed
 - `callback clear() ` : empty content

#### example

```slint
import {SURText,SURInput,SURButton, SURIcon} from "../../index.slint";
import {Themes} from "../../themes/index.slint";

export component TestInput inherits Window {
  height: 500px;
  width: 600px;
  
  SURInput{
    y: 20px;
    placeholder :"please enter your username";
    input-width:360px;
    accepted(res)=>{
      debug("content in input:" + res);
    }
    changed(change-res)=>{
      debug(change-res);
    }
    
  }
  w:=SURInput{
    y: 80px;
    theme:Themes.Success;
    type:InputType.password;
    password:true;
  }
  SURInput{
    y: 140px;
    theme:Themes.Error;
    disabled:true;
    content:"disabled";
  }
  SURInput{
    y: 200px;
    theme:Themes.Dark;
  }

  SURInput{
    y: 260px;
    theme:Themes.Warning;
    clearable:true;
  }
  SURInput{
    y: 320px;
    theme:Themes.Info;
    type:InputType.password;
    clearable:true;
    password:true;
  }
  
}
```

![image-20230910104857757](./README/imgs/image-20230910104857757.png)

 ### SURStar
 SURStar is a scoring component
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
import {SURStar,SURButton} from "../../index.slint";
import {Themes,IconSources} from "../../themes/index.slint";

component TestWindow inherits Window {
  height: 400px;
  width: 400px;
  SURStar {
    y: 20px;
  }
  hs:=SURStar {
    score: 2.2;
    y: 60px;
    theme: Error;
    
  }
  SURButton {
    y: 320px;
    x:10px;
    content: "add half";
    clicked => {
      hs.add-half();
    }
  }
  SURStar {
    score : 3.8;
    disabled: true;
    y: 100px;
    theme: Success;
  }
  os:=SURStar {
    max-score : 7;
    score : 2.8;
    y: 140px;
    theme: Info;
  }
  SURButton {
    y: 320px;
    x: 115px;
    content: "add one";
    clicked => {
      os.add-one();
    }
  }
  fs:=SURStar {
    max-score : 10;
    score : 7.2;
    y: 180px;
    no-theme:true;
    clicked(whole,half) => {
      t.n = whole;
      t.m = half;
    }
  }
  SURButton {
    y: 320px;
    x: 220px;
    content: "get A";
    clicked => {
      fs.full();
    }
  }
  SURButton {
    y: 320px;
    x: 305px;
    content: "clear";
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

![image-20230910105550811](./README/imgs/image-20230910105550811.png)

 ### SURTag
 A small tag used to display data
 #### properties
 - `in property <string> content` : the content of the tag
 - see card's properties
 #### functions
 see card's functions
 #### callbacks
 - `callback clicked()` : run if you click the tag

#### example

```
import {SURTag} from "../../index.slint";
import {Themes} from "../../themes/index.slint";

component TestWindow inherits Window {
  height: 400px;
  width: 400px;
  SURTag {
    y: 40px;
  }
  SURTag {
    content:"error!";
    y:80px;
    theme:Themes.Error;
  }
  SURTag {
    y:120px;
    theme:Themes.Dark;
    clicked=>{
      self.font-color= #ddff00;
    }
  }
  SURTag {
    content:"success";
    y:160px;
   
    theme:Themes.Success;
  }
}
```

![image-20230910105626765](./README/imgs/image-20230910105626765.png)

 ### SURHeader
 SURHeader is a simple header component that is generated based on routing information
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
import {SURHeader,Route} from "../../index.slint";
import {Themes} from "../../themes/index.slint";

component TestWindow inherits Window {
  height: 400px;
  width: 660px;
  SURHeader {
    x:10px;
    y: 40px;
  }
  SURHeader {
    x:10px;
    y: 100px;
    theme: Error;
    route:{
      home:"slint",routes:["components","dist","v1.2.0"]
    };
  }
  SURHeader {
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

![image-20230910105709278](./README/imgs/image-20230910105709278.png)

 ### SURTable
This is the outter of the Table, and the column data of the table is separated from the outter
The outter only serves as a standard layout , this is a zero cost construction

 #### properties
 - see SURCard
 #### functions
 - see SURCard
 #### callbacks
 - see SURCard

### SURTableColumn
SURTableColumn is a component of SURTable, and each SURTableColumn forms a complete column of the table
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
import {SURTable,SURTableColumn} from "../../index.slint";
import {Themes} from "../../themes/index.slint";

export component TestTable inherits Window {
  height: 500px;
  width: 500px;
  t1:=SURTable {

    y: 10px;
    // you can use this way to get height
    // it depends on how many datas in column
    height: col1.get-height();
    width: 300px;
    
    col1:=SURTableColumn {
      border:false;
      theme:Themes.Error;
      width: 100px;
      name:"id";
      // row-height:60px;
      datas: ["101","102","103"];
    }
    SURTableColumn {
      theme:Themes.Error;
      width: 100px;
      name:"name";
      datas: ["Mat","Jarry","Kaven"];
    }
    SURTableColumn {
      theme:Themes.Error;
      width: 100px;
      name:"age";
      datas: ["16","23","18"];
    }
  }
  t2:=SURTable {

    y: t1.height + 40px;
    // you can use this way to get height
    // it depends on how many datas in column
    height: tcol1.get-height();
    width: 350px;
    
    tcol1:=SURTableColumn {
      border:false;
      theme:Themes.Primary;
      width: 100px;
      name:"id";
      // row-height:60px;
      datas: ["101","102","103"];
    }
    SURTableColumn {
      theme:Themes.Primary;
      width: 100px;
      name:"name";
      datas: ["Mat","Jarry","Kaven"];
    }
    SURTableColumn {
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

![image-20230910105946372](./README/imgs/image-20230910105946372.png)

### SURCollapse
SURCollapse is a foldable panel

This is the outter of the Collapse, what really works is SURCollapseItem

The outter only serves as a standard layout , this is a zero cost construction

#### properties
- see SURCard
#### functions
- see SURCard
#### callbacks
- see SURCard

### SURCollapseItem
SURCollapseItem is a component of SURCollapse, without which SURCollapse will not work
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
import {SURCollapse,SURCollapseItem,SURButton,SURTable,SURTableColumn} from "../../index.slint";
import {Themes,IconSources} from "../../themes/index.slint";


component TestWindow inherits Window {
  height: 500px;
  width: 400px;
  SURCollapse {
    y: 10px;
    // you can set 0 , it has no impact
    // recommend use the following way
    height: item1.get-height() * 2;
    width: 360px;
    theme: Primary;
    item1:=SURCollapseItem {
      name:"Feedback";
      detail:" Operation feedback: enable the users to clearly perceive their operations by style updates and interactive effects";
      
    }
    SURCollapseItem {
      theme: Themes.Error;
      define:true;
      SURButton { 

      }
    }
    SURCollapseItem {
      name:"table";
      theme: Themes.Dark;
      define:true;
      item-height:280px;
      SURTable {
        
        height: col1.get-height();
        width: 300px;
        col1:=SURTableColumn {
          border:false;
          theme:Themes.Error;
          width: 100px;
          name:"id";
          // row-height:60px;
          datas: ["101","102","103"];
        }
        SURTableColumn {
          theme:Themes.Success;
          width: 100px;
          name:"name";
          datas: ["Mat","Jarry","Kaven"];
        }
        SURTableColumn {
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

![image-20230910110027145](./README/imgs/image-20230910110027145.png)

### SURResult
SURResult helps you easily build a quick prompt , you can build it in popup window
#### properties
- `in property <length> icon-size`: icon size
- `in-out property <string> btn `: the content of the button
- `in-out property <string> content` : content of the result
- `in property <ResType> res-type` : Result type
- `in-out property <Icons> icon`: Icon of the result
#### functions
#### callbacks
- `callback clicked()` : run if you click the button

```
import {SURResult,ResType} from "../../index.slint";
import {Themes} from "../../themes/index.slint";

export component TestResult inherits Window {
  height: 500px;
  width: 800px;
  SURResult {
    x: 10px;
    y: 10px;
  }
  SURResult {
    x: 220px;
    y: 10px;
    res-type:ResType.Primary;
  }
  SURResult {
    x: 220px;
    y: 260px;
    res-type:ResType.Info;
  }
  SURResult {
    x: 10px;
    y: 260px;
    res-type:ResType.Warning;
  }

  SURResult {
    x: 440px;
    y: 10px;
    res-type:ResType.Error;
  }
  SURResult {
    x: 440px;
    y: 260px;
    res-type:ResType.Help;
  }
}
```

![image-20230910110056779](./README/imgs/image-20230910110056779.png)

### SURSelect
SURSelect is a selector that provides three types of optional input parameter values
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
import {SURSelect,ValueType} from "../../index.slint";
import {Themes} from "../../themes/index.slint";

component TestWindow inherits Window {
  height: 440px;
  width: 400px;
  SURSelect {
    y: 20px;
    ranges-string: [
      {id:0,label:"Shangai",value:"s01"},
      {id:1,label:"Los Angeles",value:"l02"},
      {id:2,label:"New York",value:"n03"},
      {id:3,label:"Hong Kong",value:"h04"},
    ];
  }
  SURSelect {
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

![image-20230910110204450](./README/imgs/image-20230910110204450.png)

### SURLink

SURLink is commonly used to represent text connections or sharing

#### properties

* `in property <image> icon` : share icon you can use whatever you want
* `in property <bool> funny` : Easter egg just funny

#### callbacks

* `callback clicked()` :  run if you click share icon

#### exeample

```
import {SURLink} from "../../index.slint";
import {Themes,IconSources} from "../../themes/index.slint";

component TestWindow inherits Window {
  height: 420px;
  width: 400px;
  
  SURLink {
    y: 100px;
    theme: Dark;
  }
  SURLink {
    y: 160px;
    funny:true;
    theme: Warning;
    content: "funny for link!";
  }
  SURLink {
    y: 220px;
    theme: Primary;
    icon: @image-url("../../icons/share-one.svg");
    content: "share one";
  }
  SURLink {
    y: 280px;
    funny:true;
    theme: Error;
    icon : @image-url("../../icons/share-sys.svg");
    font-size: 24px;
    content: "share sys";
    clicked=>{
      debug("share sys!")
    }
  }
}
```

![image-20230910110312615](./README/imgs/image-20230910110312615.png)

### SURAvatar

SURAvatar is a avatar component that defaults to Icons.Avatar when there are no images available

#### properties

* `in property <length> avatar-size` : avatar size
* `in property <image> avatar` : avatar image

#### example

```
import {SURAvatar} from "../../index.slint";
import {Themes,IconSources,ROOT-STYLES} from "../../themes/index.slint";

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
    padding-size : Small;
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
    padding-size : Small;
    theme: Error;
  }
  SURAvatar {
    y: 300px;
    avatar-size : ROOT-STYLES.sur-size.large * 2;
    padding-size : Large;
    theme: Dark;
    avatar:@image-url("../../README/imgs/logo.png");
  }
  
}
```

![image-20230910110253626](./README/imgs/image-20230910110253626.png)

### SURRadio

Radio let people select a single item

#### properties (card)

* `in-out property <bool> has-clicked` : the radio is clicked or not
* `in-out property <brush> active-color`: radio activecolor

#### functions

#### callbacks

* `callback clicked()` : run if you click the radio

#### example

```
import {SURRadio} from "../../index.slint";
import {Themes} from "../../themes/index.slint";

component TestCollection inherits Window {
  height: 560px;
  width: 600px;
  
  SURRadio{
    y: 60px;
  }

  SURRadio{
    y: 180px;
    active-color : #4affae;
    theme:Primary;
  }
}
```

![image-20230912155049511](./README/imgs/image-20230912155049511.png)

### SURPopup

A masked pop-up layer appears in the current window

And users will not be able to use the pop-up layer to cover the components under it. Clicking on the pop-up layer again will close it

#### properties

* `in-out property <bool> is-show` : the popup layer is show or not
* `in property <Themes> theme` : Surrealism Themes

#### functions

#### callbacks

* `callback open()` : open the popup
* `callback close()` : close the popup

#### example

```
import {SURPopup,SURButton} from "../../index.slint";
import {Themes} from "../../themes/index.slint";

component TestDivider inherits Window {
  height: 800px;
  width: 800px;
  background: #535353;
 
  SURButton {
    
    content: "show";
    clicked => {
      p.open();
      
      debug("sds1")
    }
  }
 

  p:=SURPopup {
    SURButton {
      content: "you can add anything in Popup";
      y: 160px;
    }
  }
}
```

![image-20230912155117323](./README/imgs/image-20230912155117323.png)

### SURDivider

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
import {SURDivider} from "../../index.slint";
import {Themes,IconSources} from "../../themes/index.slint";

component TestDivider inherits Window {
  height: 400px;
  width: 400px;
  background: #535353;
  
  SURDivider {
    y: 60px;
    width: 380px;
  }
  SURDivider {
    y: 120px;
    width: 380px;
    icon:@image-url("../../icons/nail-polish-one.svg");
    theme:Themes.Error;
  }
  SURDivider {
    y: 180px;
    width: 380px;
    icon:@image-url("../../icons/earth.svg");
    theme:Themes.Dark;
  }
  SURDivider {
    y: 240px;
    width: 380px;
    content:"";
    theme:Themes.Light;
  }
}
```

![image-20230913035535856](E:\Rust\try\surrealism-ui\README\imgs\image-20230913035535856.png)

### SURCollection

SURCollection is a grid storage box, but in reality it is not based on grid layout.

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

* `clicked(CollectionData)` : run if you click item in SURCollection

```
import {SURButton,SURCollection} from "../../index.slint";
import {Themes} from "../../themes/index.slint";

component TestCollection inherits Window {
  height: 560px;
  width: 900px;
  
  SURCollection{
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

![image-20230912155404367](./README/imgs/image-20230912155404367.png)
