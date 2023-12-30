# `SText` 文字

It is the simplest and most common component in SurrealismUI

It can display text , SText inherits Text and add theme property

## properties inherits Text

- in property <Themes> theme : Surrealism themes

## callbacks: 

## functions:

- pure public function get()->string : get content
- public function set(content:string) : set content

## example

```slint
import {SText} from "../../index.slint";
import {Themes} from "../../use/index.slint";


component TestText inherits Window {
  height: 400px;
  width: 400px;
  VerticalLayout {
    padding: 20px;
    SText {
      color: #f60;
      text: "use special color";
    }
    SText {
      theme: Primary;
      text: "use theme-primary";
    }
    SText {
      theme: Dark;
      text: "use theme-dark";
    }
    SText {
      text: "中文文字";
    }
  }
}
```

# `SButton` 按钮

SButton is a button component that you can freely perform regular properties operations on

## properties inherits SCard

- in property <image> icon : button icon in the left;
- in property <bool> show-icon : show icon or not;
- out property <bool> has-hover : hover button or not;
- in-out property <string> text : text display in button;
- in property <length> letter-spacing : text letter-spacing;
- in property <bool> round  : button is round or not;

## functions

## callbacks 

- clicked : run if you click the button

## example

```slint
import {SButton} from "../../index.slint";
import {Themes} from "../../use/index.slint";

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
      self.text = "clicked"
    }
  }
  SButton {
    x: 260px;
    y: 10px;
    round : true;
    text:"Save";
    clicked => {
      self.text = "clicked";
    }
  }
  SButton {
    x: 20px;
    y: 100px;
    text:"Success";
    theme:Themes.Success;
  }
  SButton {
    x: 20px;
    y: 200px;
    text:"Primary";
    theme:Themes.Primary;
  }
  SButton {
    x: 20px;
    y: 300px;
    text:"Info";
    theme:Themes.Info;
  }
  SButton {
    x: 200px;
    y: 100px;
    text:"Error?";
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
# `SDivider` 分割线

A divider groups sections of content to create visual rhythm and hierarchy.
 
Use dividers along with spacing and headers to organize content in your layout. 

## properties inherits SCard
## functions
## callbacks
## exmaple
```slint
import {SDivider} from "../../index.slint";
import {Themes} from "../../use/index.slint";
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
    theme:Themes.Error;
  }
  SDivider {
    y: 180px;
    width: 380px;
    theme:Themes.Primary;
  }
  SDivider {
    y: 240px;
    width: 380px;
    theme:Themes.Light;
  }
}
```
# `SIcon` 图标

this is a icon container, better than Image

## properties:

- in property <MouseCursor> mouse-cursor : mouse cursor of the icon
- in property <Themes> theme : SurrealismUI themes
- in property <image> source : icon source
- in-out property <brush> colorize : icon color
- in property <ImageFit> image-fit : icon image fit
- in property <ImageRendering> image-rendering : image rendering
- in-out property <RotationProps> rotation : image rotation 
- in property <int> source-clip-x : icon clip x
- in property <int> source-clip-y : icon clip y
- in property <int> source-clip-height : icon clip height
- in property <int> source-clip-width : icon clip width
- out property <bool> has-hover : icon has hover

## functions

- pure public function get-colorize()->brush : get icon color

## callbacks: 
- callback clicked : run if you click the icon

## example

```slint
SIcon{
    height: 30px;
    width: 30px;
    colorize: self.get-colorize();
    source: @image-url("../../icons/sd-card.svg");
    theme: Themes.Primary;
}
```

# `SCard` 卡片

A very simple universal card without any layout or restrictions

you can add anything you want to the card

## properties inherits Rectangle
- in property <Themes> theme : Surrealism theme;
- in property <length> card-height: card height (not contain padding);
- in property <length> card-width: card width (not contain padding);
- in property <PaddingType> padding-type : padding type;
- in property <ShadowType> shadow-type : shadow type;
- in property <BorderType> border-type : border type;
- in property <int> font-weight : font weight;
- in property <length> font-size: font size;
- in property <brush> font-color : font color;
- in property <bool> font-italic : font italic;
- in property <string> font-family : font family;
- in-out property <PaddingProps> card-padding : inner card padding struct;
- in-out property <BorderProps> card-border : inner card border struct;
- in-out property <ShadowProps> card-shadow : inner card shadow struct;
## example

```slint
import {SButton,SCard,SText} from "../../index.slint";
import {Themes} from "../../use/index.slint";


export component TestCard inherits Window {
  height: 400px;
  width: 600px;
  VerticalLayout {
    padding: 20px;
    spacing: 20px;
    SCard {
      SText {
        text: "card";
      }
    }
    SCard {
      card-height: 36px;
      card-width: 168px;
      theme: Light;
    }
    SCard {
      card-height: 124px;
      width: 400px;
      theme: Primary;
    }
  }
}
```
