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

![image-20230907010143373](E:\Rust\try\surrealism-ui\README\imgs\image-20230907010143373.png)

#### success

1. opacity：\#7de39187
2. font：\#e0fcf7
3. weakest：#B0E5DC
4. weaker：#7FD5A2
5. normal：\#66CD7A
6. deeper：\#4aa949
7. deepest：\#33956B

![image-20230907010935247](E:\Rust\try\surrealism-ui\README\imgs\image-20230907010935247.png)

#### info

1. opacity：\#d7d7d788
2. font：\#ffffff
3. weakest：\#E6E5E6
4. weaker：\#DEDDDE
5. normal：\#d7d7d7
6. deeper：\#bcbcbc
7. deepest：\#878787

![image-20230907011024366](E:\Rust\try\surrealism-ui\README\imgs\image-20230907011024366.png)

#### warning

1. opacity：\#f06b4288
2. font：\#fdd1c3
3. weakest：\#e48d73
4. weaker：\#f07651
5. normal：\#f06b42
6. deeper：\#e95a2e
7. deepest：\#e63819

![image-20230907011054687](E:\Rust\try\surrealism-ui\README\imgs\image-20230907011054687.png)

#### error

1. opacity：\#e34e4788
2. font：\#fbe3d4
3. weakest：\#e9a9a7
4. weaker：\#DC8472
5. normal：\#e34e47
6. deeper：\#D03D46
7. deepest：\#9e2929

![image-20230907011118330](E:\Rust\try\surrealism-ui\README\imgs\image-20230907011118330.png)

#### dark

1. opacity：#262a3987
2. font：#73788c
3. weakest：#2f323d
4. weaker：#171922
5. normal：#1a1c26
6. deeper：#0f121c
7. deepest：#101114

![image-20230907011139226](E:\Rust\try\surrealism-ui\README\imgs\image-20230907011139226.png)

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

![image-20230907013446133](E:\Rust\try\surrealism-ui\README\imgs\image-20230907013446133.png)

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

![image-20230907014520907](E:\Rust\try\surrealism-ui\README\imgs\image-20230907014520907.png)

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

![image-20230907012550038](E:\Rust\try\surrealism-ui\README\imgs\image-20230907012550038.png)
