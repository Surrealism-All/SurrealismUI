# SurrealismUI

SurrealismUI是一个完全使用Slint进行构建的Slint第三方组件库

SurrealismUI is a third-party component library built entirely using Slint

## About Doc Icon

- ⛔ ： do not use

## Themes

在SurrealismUI中内置了6个主题色

- primary
- success
- info
- warning
- error
- dark

Built in 6 theme colors in SurrealismUI

- primary
- success
- info
- warning
- error
- dark 

### themes-color

#### primary

1. opacity：#1A5BE988
2. font：#bbdbf6
3. weakest：#96C4ED
4. weaker：\#4584E9
5. normal：#1A5BE9
6. deeper：#0F3CC9
7. deepest：\#1d2f7a

![image-20230907010143373](.\README\imgs\image-20230907010143373.png)

#### success

1. opacity：\#7de39187
2. font：\#e0fcf7
3. weakest：#B0E5DC
4. weaker：#7FD5A2
5. normal：\#66CD7A
6. deeper：\#4aa949
7. deepest：\#33956B

![image-20230907010935247](.\README\imgs\image-20230907010935247.png)

#### info

1. opacity：\#d7d7d788
2. font：\#ffffff
3. weakest：\#E6E5E6
4. weaker：\#DEDDDE
5. normal：\#d7d7d7
6. deeper：\#bcbcbc
7. deepest：\#878787

![image-20230907011024366](.\README\imgs\image-20230907011024366.png)

#### warning

1. opacity：\#f06b4288
2. font：\#fdd1c3
3. weakest：\#e48d73
4. weaker：\#f07651
5. normal：\#f06b42
6. deeper：\#e95a2e
7. deepest：\#e63819

![image-20230907011054687](.\README\imgs\image-20230907011054687.png)

#### error

1. opacity：\#e34e4788
2. font：\#fbe3d4
3. weakest：\#e9a9a7
4. weaker：\#DC8472
5. normal：\#e34e47
6. deeper：\#D03D46
7. deepest：\#9e2929

![image-20230907011118330](.\README\imgs\image-20230907011118330.png)

#### dark

1. opacity：#262a3987
2. font：#73788c
3. weakest：#2f323d
4. weaker：#171922
5. normal：#1a1c26
6. deeper：#0f121c
7. deepest：#101114

![image-20230907011139226](.\README\imgs\image-20230907011139226.png)

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
import {SURText} from "../../components/index.slint";
import {Themes} from "../../components/themes/index.slint";

component TestWindow inherits Window {
  height: 400px;
  width: 400px;
  SURText {
    x: 100px;
    y: 0;
    content: "hello world";
  }
  SURText {
    x:100px;
    y:100px;
    theme:Themes.Error;
  }
 
}
```

![image-20230907013446133](.\README\imgs\image-20230907013446133.png)

 ### SURIcon
 there are 2658 different icons in SURIcon from : https://github.com/bytedance/iconpark
 #### properties:
 - `in property <Icons> icon` : icon types
 - `in property <Themes> theme` : Surrealism theme
 - `in-out property <brush> icon-color` : icon color 
 - `private property <[IconItem]> icon-datas` : source icon datas ⛔
 #### callbacks: 
 - `callback clicked` : run if you click the icon
 #### functions:
 - `pure function get_icon(item:IconItem)->image` : get icon src from for iter item ⛔

#### example

```
import {SURIcon} from "../../components/index.slint";
import {Icons,Size,Themes} from "../../components/themes/index.slint";
component TestWindow inherits Window {
  height: 400px;
  width: 400px;
  SURIcon{
    x: 10px;
    y: 20px;
    icon: Icons.Abnormal;
    theme: Themes.Primary;
  }
  SURIcon{
    x: 60px;
    y: 20px;
    icon: Icons.Add;
    theme: Themes.Success;
  }
  SURIcon{
    x: 100px;
    y: 20px;
    height: 30px;
    width: 30px;
    icon: Icons.Baby-car-seat;
    theme: Themes.Error;
    
  }
  SURIcon{
    x: 10px;
    y: 100px;
    icon: Icons.T-shirt;
    theme: Themes.Dark;
    height: 30px;
    width: 30px;
  }
  SURIcon{
    height: 24px;
    width: 24px;
    x: 60px;
    y: 100px;
    icon: Icons.Baby-meal;
    theme: Themes.Info;
  }
  SURIcon{
    height: 24px;
    width: 24px;
    x: 100px;
    y: 100px;
    icon: Icons.Vacation;
    theme: Themes.Warning;
    clicked=>{
      debug("clicked");
      self.theme= Themes.Error;
      self.height += 2px;
      self.width += 2px;
    }
  }
}
```

![image-20230907014520907](.\README\imgs\image-20230907014520907.png)

 ### SURCard
 A very simple universal card without any layout or restrictions
 you can add anything you want to the card
 #### properties
 - `in property <Themes> theme` : Surrealism theme
 #### functions
 - `pure public function count-height(h:length) -> length` : a cheap way to calculate height
 - `pure public function count-width(w:length) -> length` : a cheap way to calculate width

#### example

```
import {SURButton,SURCard} from "../../components/index.slint";
import {Themes,Icons} from "../../components/themes/index.slint";

component TestWindow inherits Window {
  height: 400px;
  width: 400px;
  SURCard { 
    y: 20px;
    height: 40px;
    width: 160px;
   }
   SURCard { 
    y: 80px;
    height: 40px;
    width: 160px;
    theme: Themes.Warning;
   }
   SURCard { 
    y: 200px;
    height: self.count-height(160px);
    width: self.count-width(200px);
    theme: Themes.Error;
    SURButton {

    }
   }
}
```

![image-20230907143804602](.\README\imgs\image-20230907143804602.png)

 ### SURButton 
 SURButton is a button component that you can freely perform regular attribute operations on
 #### properties
 - `in property <Themes> theme`: Surrealism Themes
 - `in property <Icons> icon` : Icons.Null : do button has icon
 - `in-out property <brush> font-color `: button content color
 - `in-out property <brush> icon-color` : button icon color
 - `in property <length> font-size` : font size
 - `in property <int> font-weight` : font weight
 - `in property <bool> font-italic`: font italic
 - `in property <string> font-family` : font family
 - `in property <bool> circle` : set the button as a rounded button
 - `private property <length> letter-spacing` : content letter-spacing ⛔
 - `in-out property <string> content` : the content of the button 
 #### functions
 #### callbacks 
 - `clicked` : run if you click the button

#### example

```
import {SURButton} from "../../components/index.slint";
import {Themes,Icons} from "../../components/themes/index.slint";
component TestWindow inherits Window {
  height: 400px;
  width: 400px;
  
  SURButton {
    x: 20px;
    y: 10px;
    font-size: 20px;
    font-weight:700;
    theme:Themes.Dark;
    icon:Icons.Mini-sd-card;
    clicked => {
      self.content = "clicked"
    }
  }
  SURButton {
    x: 20px;
    y: 100px;
    font-size: 20px;
    font-weight:700;
    theme:Themes.Success;
    circle:true;
  }
  SURButton {
    x: 20px;
    y: 200px;
    font-size: 20px;
    font-weight:700;
    theme:Themes.Primary;
  }
  SURButton {
    x: 20px;
    y: 300px;
    font-weight:700;
    theme:Themes.Info;
  }
  SURButton {
    x: 200px;
    y: 100px;
    font-size: 12px;
    font-weight:700;
    theme:Themes.Error;
    icon:Icons.Magic-hat;
  }
  SURButton {
    x: 200px;
    y: 200px;
    font-size: 20px;
    font-weight:700;
    theme:Themes.Warning;
  }
}
```

![image-20230907144512434](.\README\imgs\image-20230907144512434.png)

 ### SURInput

 This is a basic input box, often used in forms, divided into two types` : text and password
 #### properties :
 - `in property <string> placeholder` : default placeholder which you wanna show when no content
 - `in property <Themes> theme` : Surrealism themes
 - `in property <Icons> icon` : icon you wanna show in front (use >= v0.1.0) ⛔
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
import {SURText,SURInput,SURButton, SURIcon} from "../../components/index.slint";
import {Themes} from "../../components/themes/index.slint";

component TestWindow inherits Window {
  height: 400px;
  width: 400px;
  
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

![image-20230907012550038](.\README\imgs\image-20230907012550038.png)

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
 #### callbacks
 - `callback clicked(float,float)` : get how many whole stars and half stars

#### example

```
import {SURStar} from "../../components/index.slint";
import {Themes,Icons} from "../../components/themes/index.slint";

component TestWindow inherits Window {
  height: 400px;
  width: 400px;
  SURStar {
    y: 20px;
  }
  SURStar {
    score: 4.2;
    y: 60px;
    theme: Error;
  }
  SURStar {
    score : 3.8;
    disabled: true;
    y: 100px;
    theme: Success;
  }
  SURStar {
    max-score : 7;
    score : 6.8;
    y: 140px;
    theme: Info;
  }
  SURStar {
    max-score : 10;
    score : 7.2;
    y: 180px;
    no-theme:true;
    clicked(whole,half) => {
      t.n = whole;
      t.m = half;
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

![image-20230907143639667](.\README\imgs\image-20230907143639667.png)

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
import {SURTag} from "../../components/index.slint";
import {Themes,Icons} from "../../components/themes/index.slint";

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
    font-color:#3670d5;
    theme:Themes.Success;
  }
}
```

![image-20230907171859046](E:\Rust\try\surrealism-ui\README\imgs\image-20230907171859046.png)
