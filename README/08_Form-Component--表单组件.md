# `SInput` 输入框
This is a basic input box, often used in forms, divided into two types: text and password
## properties:
- in property <int> font-weight : font weight for input
- in property <string> placeholder: default placeholder which you wanna show when no text
- in property <Themes> theme: Surrealism themes
- in property <length> input-width: Please do not use width to adjust the length of the input box , use this property to instead
- in property <length> font-size: font size 
- in property <bool> disabled: can input be edited
- in property <bool> clearable: can input be cleared
- in property <bool> password: can the password input display the password
- out property <bool> has-focus : input is focused or not
- private property <brush> placeholder-color : placeholder color
- in-out property <InputType> type : input type (text or password)
- in-out property <brush> font-color : font color
- in-out property <brush> icon-color : icon color
- in-out property <string> text : the text of the input
## functions:
- pure public function get() ->string : get text
- public function set(text:string) : set text
- public function clear() : clear text
- public function select-all() : select all 
- public function clear-selection() : clears the selection
- public function cut() : copies the selected text to the clipboard and removes it from the editable area
- public function copy() : copies the selected text to the clipboard
- public function paste() : pastes the text text of the clipboard at the cursor position
## callbacks:
- callback accepted(string) : run when pressed down enter key
- callback changed(string) : run when text changed
## example
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
    placeholder :"please enter your username";
    card-width:300px;
    clearable: true;
    text:"SurrealismUI - input";
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
    theme:Themes.Success;
    type:InputType.password;
    password:true;
  }
  SInput{
    y: 140px;
    card-width: 20rem;
    theme:Themes.Error;
    disabled:true;
    text:"disabled";
  }
  SInput{
    y: 200px;
    card-width: 18rem;
    theme:Themes.Dark;
  }

  SInput{
    y: 260px;
    card-width: 160px;
    theme:Themes.Warning;
    clearable:true;
  }
  SInput{
    y: 320px;
    card-width: 18rem;
    theme:Themes.Info;
    type:InputType.text;
    clearable:true;
    password:true;
    text:"test password";
    
  }
}
```
# `SStar` 评分
SStar is a scoring component
## properties
- in property <bool> no-theme : use Surrealism Theme or not
- in property <float> score : the real score
- in property <Themes> theme : Themes.Primary;
- in property <bool> disabled : can be scored if disabled is false
- in property <float> max-score : max score (how many stars you wanna show)
## functions
- pure function get-half-stars()->bool : count the number of half stars ⛔
- pure function get-whole-stars()->int : count the number of whole stars ⛔
- pure function get-empty-stars()->int : count the number of empty stars ⛔
- public function full() : star all 👍
- public function clear() : no star 👍
- public function add-one() : add one star 👍
- public function add-half() : add half stars 👍
## callbacks
- callback clicked(float,float) : get how many whole stars and half stars
## example
```slint
import {SStar,SButton} from "../../index.slint";
import {Themes,UseIcons} from "../../use/index.slint";

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
# `SSelect` 选择框
SSelect is a selector that provides three types of optional input parameter values
## properties inherits SCard
- in property <int> item-font-weight : select item font weight
- in property <length> item-font-size: select item font size
- in property <bool> item-font-italic : select item font
- in property <string> item-font-family : select item font
- in property <[SOption]> options : select options
- in property <string> placeholder : select placeholder
- in-out property <bool> is-show : select is show or not
## functions
- public function open() : open select
- public function close() : close select
- public function toggle() : toggle status (if open then close)
## callbacks
- callback changed(int,string,string) : run if you choose an item of list
## example
```slint
import {SSelect} from "../../index.slint";
import {Themes} from "../../use/index.slint";
import { SButton } from "../../src/button/index.slint";

component TestWindow inherits Window {
  height: 440px;
  width: 400px;
  SSelect {
    y: 20px;
    options: [
      {id:0,label:"Shangai",value:"s01"},
      {id:1,label:"Los Angeles",value:"l02"},
      {id:2,label:"New York",value:"n03"},
      {id:3,label:"Hong Kong",value:"h04"},
    ];
  }
  
  SSelect {
    y: 200px;
    font-weight: 700;
    is-show: true;
    theme: Error;
    options: [
      {id:0,label:"Shangai",value:"s01"},
      {id:1,label:"Los Angeles",value:"l02"},
      {id:2,label:"New York",value:"n03"},
      {id:3,label:"Hong Kong",value:"h04"},
    ];
    changed(index,label,value)=>{
      debug(index);
      debug(label);
      debug(value);
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
# `SLink` 链接
SLink is commonly used to represent text connections or sharing
## properties
- in property <image> icon : link share icon
- in property <bool> funny : use funny underline
- in property <bool> underline : has underline or not
- out property <bool> has-hover : link has hover or not 
- in property <MouseCursor> mouse-cursor : link mouse cursor
- in property <Themes> theme : SurrealismUI theme
- in property <length> font-size : link font size
- in-out property <string> text : link text
- in property <int> font-weight : link font weight
- in property <bool> font-italic : link font italic
- in property <string> font-family : link font family
- private property <brush> text-color : link text color
## callbacks
- callback clicked(string) : run if you click share icon
## example
```
import {SLink} from "../../index.slint";
import {Themes,UseIcons} from "../../use/index.slint";

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
    font-italic : true;
    font-weight: 200;
    font-family : "Verdana";
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
    clicked(link-text)=>{
      debug("share sys!")
    }
  }
}
```
# `SRadio` 单选框
SRadio let people select a single item
## properties inherits Rectangle
- in property <int> font-weight : display text font weight
- in property <length> font-size: display text font size
- in property <brush> font-color : display text font color
- in property <bool> font-italic : display text font italic
- in property <string> font-family : display text font family
- in property <Themes> theme : SurrealismUI theme
- in property <length> card-height: radio height (contain padding)
- in property <length> card-width: radio width (contain padding)
- in property <string> text : display text
- in-out property <string> value : radio value
- in-out property <bool> actived : is actived or not
- in-out property <brush> active-color: active radio color
- in property <PaddingType> padding-type : radio padding type
- in property <ShadowType> shadow-type : radio shadow type
- in property <BorderType> border-type : radio border type
## functions
## callbacks
- callback clicked(string,string,bool) : run if you click the radio
## example
```
import {SRadio} from "../../index.slint";
import {Themes} from "../../use/index.slint";
import { SText } from "../../src/text/index.slint";

component TestCollection inherits Window {
  height: 560px;
  width: 600px;
  background: #515c74;
  VerticalLayout {
    spacing: 20px;
    padding: 20px;
    SRadio{
      actived: true;
    }
    SRadio{theme: Light;}
    SRadio{theme: Primary;}
    SRadio {theme: Error;}
    SRadio {
      theme: Warning;
      font-weight: 700;
      text : "Chinese";
      font-color: #ff5e00;
      font-italic: true;
      font-family: "Verdana";
    }
    SRadio {
      theme: Info;
      clicked(text,value,actived) => {
        debug(text);
        debug(value);
        debug(actived);
      }
    }
    SRadio{
      active-color : #4affae;
      theme:Primary;
      actived : true;
    }
  }
}
```
# `SSwitch` 选项
SSwitch is a switch used for simple judgment scenarios
## properties
- in-out property <bool> active : is actived or not
- in property <brush> switch-background-color : switch background color
- in property <brush> switch-border-color : switch border color
- in property <color> switch-drop-shadow-color switch drop shadow color
- in property <length> switch-height : switch height
- in property <length> switch-width : switch width
- in property <PaddingType> switch-padding-type: switch padding type
- in property <ShadowType> switch-shadow-type: switch shadow type
- in property <BorderType> switch-border-type : switch border type
## callbacks
- callback clicked(bool) : run if you click the switch
## example
```
import { SSwitch , SCard} from "../../index.slint";

component TestSwitch inherits Window {
  height: 400px;
  width: 400px;
  
  
    VerticalLayout {
      spacing: 20px;
      padding: 20px;
      SSwitch {
      }
      SSwitch {
        active: true;
        theme: Primary;
        switch-background-color:#ddd;
        switch-border-color:#ff00bb;
      }
      SSwitch {
        theme: Dark;
        active: false;
        clicked(active-or-not)=>{
          debug(active-or-not);
        }
      }
      SSwitch {
        theme: Warning;
      }
      SSwitch {
        theme: Error;
      }
      SSwitch {
        theme: Info;
      }
    }
}
```
# `SSwitchGroup` 选项组
SSwitchGroup switch group can contain more switch cases
## properties inherits SCard
- in-out property <string> active : active option value
- in-out property <[SOption]> switchs : switch options
- in property <length> font-size : font size , it will effect switch component height
- private property <brush> theme-color : inner theme color
## callbacks
- callback clicked(int,SOption) : run if you click the switch , it will back option index and option name
## example
```
import { SSwitchGroup } from "../../index.slint";
import { PaddingType } from "../../use/index.slint";
component TestSwitchGroup inherits Window {
  height: 400px;
  width: 400px;
  SSwitchGroup {
  active: "2";
   theme: Primary;
   width: 240px;
   switchs:[
    {label:"Option1",value:"0"},
    {label:"Option2",value:"2"},
    {label:"Option3",value:"3"},
  ];
    clicked(i,name) => {
      debug(i);
      debug(name);
    }
  }
  SSwitchGroup {
    active: "1";
    y: 120px;
    theme:Dark;
    padding-type: PaddingType.Small;
    switchs:[
      {label:"1",value:"0"},
      {label:"2",value:"2"},
      {label:"3",value:"3"},
      {label:"4",value:"1"},
    ];
     clicked(i,name) => {
       debug(i);
       debug(name);
     }
   }
}
```