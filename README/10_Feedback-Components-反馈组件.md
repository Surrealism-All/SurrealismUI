# `SResult` 结果
SResult helps you easily build a quick prompt , you can build it in popup window
## properties
- in-out property <string> btn-text : result button text
- in property <length> icon-size : result icon size
- in property <[SButtonProps]> btns : result buttons
- in property <ResultType> result-type: result type
- in-out property <string> text : text of the result
- in-out property <image> icon : result icon
## functions
## callbacks
- callback clicked(SButtonProps) : run if you click the button
## example
```
import {SResult} from "../../index.slint";
import {Themes,ResultType} from "../../use/index.slint";
import {DefaultSButtonProps} from "../../themes/index.slint";
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
    result-type:ResultType.Primary;
    clicked(e) => {
      //没有设置btns获取的是空SButtonProps
      debug(e);
    }
  }
  SResult {
    x: 220px;
    y: 260px;
    card-width: 300px;
    text : "use button slot";
    font-size: 18px;
    font-weight: 700;
    result-type:ResultType.Info;
    btns:[
      {
        font-weight : DefaultSButtonProps.font-weight,
        font-size : DefaultSButtonProps.font-size,
        font-italic : DefaultSButtonProps.font-italic,
        font-family : DefaultSButtonProps.font-family,
        theme : Themes.Success,
        padding-type : DefaultSButtonProps.padding-type,
        shadow-type : DefaultSButtonProps.shadow-type,
        border-type : DefaultSButtonProps.border-type,
        icon : DefaultSButtonProps.icon,
        show-icon : DefaultSButtonProps.show-icon,
        text : "confirm!",
        letter-spacing : DefaultSButtonProps.letter-spacing,
        clip : DefaultSButtonProps.clip,
        round : DefaultSButtonProps.round
      },
      {
        font-weight : DefaultSButtonProps.font-weight,
        font-size : DefaultSButtonProps.font-size,
        font-italic : DefaultSButtonProps.font-italic,
        font-family : DefaultSButtonProps.font-family,
        theme : Themes.Light,
        padding-type : DefaultSButtonProps.padding-type,
        shadow-type : DefaultSButtonProps.shadow-type,
        border-type : DefaultSButtonProps.border-type,
        icon : DefaultSButtonProps.icon,
        show-icon : DefaultSButtonProps.show-icon,
        text : "cancel!",
        letter-spacing : DefaultSButtonProps.letter-spacing,
        clip : DefaultSButtonProps.clip,
        round : true
      }
    ];
  }
  SResult {
    x: 10px;
    y: 260px;
    result-type:ResultType.Warning;
  }

  SResult {
    x: 440px;
    y: 10px;
    result-type:ResultType.Error;
  }
  SResult {
    x: 580px;
    y: 260px;
    result-type:ResultType.Help;
  }
}
```
# `SPopup` 弹出框
A masked popup layer appears in the current window
And users will not be able to use the popup layer to cover the components under it. 
Clicking on the popup layer again will close it
## properties  inherits Window
- in-out property <bool> is-show : popup is show or not
- in property <Themes> theme : SurrealismUI theme
- in property <percent> mask-opacity : popup mask opacity
## functions
- public function open() : open the popup
- public function close() : close the popup
## callbacks
## example
```
import {SPopup,SButton} from "../../index.slint";
import {Themes} from "../../use/index.slint";

component TestPopup inherits Window {
  height: 500px;
  width: 500px;
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
# `STip` 提示
A tip provides supplemental, contextual information elevated near its target component
## properties
- in-out property <string> font-family : tip text font family
- in-out property <int> font-weight :  tip text font weight
- in-out property <length> font-size:  tip text font size
- in-out property <brush> font-color :  tip text font color
- in-out property <bool> font-italic :  tip text font italic
- in-out property <Themes> theme :  tip theme
- in-out property <TextWrap> wrap :  tip text wrap
- in-out property <TextOverflow> overflow :  tip text overflow
- in-out property <length> letter-spacing :  tip text letter spacing
- in-out property <TextHorizontalAlignment> horizontal-alignment :  tip text horizontal alignment
- in-out property <TextVerticalAlignment> vertical-alignment : tip text vertical alignment
- in-out property <Position> position : the position of tip
- in-out property <bool> is-show : tip is show or not
- in-out property <string> text : tip text
- in property <length> tip-width : tip width
## functions
- public function open() : open the tip
- public function close() : close the tip
- pure public function count-x(w:length) ->length : count x for angle
- pure public function count-y(w:length) ->length : count y for angle
## callbacks
- callback clicked() : use to open|close the tip
## example
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
     position:Top;
     text:"this is a \n........tip window";
     is-show:inner0.has-hover;
     inner0:=SButton { 
       text: "hover";
     }
   }
  STip{
    
    height:inner.height;
    width: inner.width;
    theme: Primary;
    position:LeftBottom;
    overflow: TextOverflow.elide;
    tip-width : 120px;
    horizontal-alignment: center;
    text:"this is a ....\n....tip\n window";
    inner:=SButton { 
      text: "click";
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
    position:Right;
    // wrap: TextWrap.no-wrap;
    text:"测试文本文字";
    font-size: 16px;
    inner2:=SButton { 
      text: "click";
      clicked => {
        parent.clicked();
      }
    }
  }
 
}
```
# `SLoading` 加载
This is a loading component that you can embed anywhere you want to add a loading animation
## properties inherits Window
- in property <int> font-weight : loading text font weight
- in property <length> font-size: loading text font size
- in property <brush> font-color : loading text font color
- in property <bool> font-italic : loading text font italic
- in property <string> font-family : loading text font family
- in property <image> loading-icon : loading icon
- in-out property <duration> duration : loading animation duration
- in property <string> text : loading text 
- in-out property <bool> is-show : loading is show or not
- in property <Themes> theme : SurrealismUI theme
- in property <easing> easing : loading animation easing type
- in-out property <int> iteration-count : loading animation iteration count
## functions
## callbacks
- callback open() : open the loading
- callback close() : close the loading
## example
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
      p:=SLoading {
        text : "SurrealismUI";
        font-weight:700;
      }
    }
}
```
# `SDialog` 对话框
SDialogs are used to confirm messages or events and display text
## properties
- in property <Themes> theme : Dialog mask theme
- in property <Themes> cancel-btn-theme : Dialog cancel button theme
- in property <Themes> confirm-btn-theme : Dialog confirm button theme
- in property <string> cancel-btn-text : Dialog cancel button text
- in property <string> confirm-btn-text : Dialog confirm button text
- in-out property <bool> is-show: Dialog is show or not
- in property <percent> mask-opacity : Dialog mask opacity
- in property <length> spacing : Dialog spacing
- in property <int> font-weight : Dialog text font weight
- in property <length> font-size: Dialog text font size;
- in property <brush> font-color : Dialog text font color;
- in property <bool> font-italic : Dialog text font italic;
- in property <string> font-family : Dialog text font family;
- in property <Themes> dialog-theme : Dialog theme 
- in property <string> dialog-title : Dialog title
- in property <length> dialog-title-size : Dialog title size
- in property <string> dialog-details : Dialog detail text
- in property <float> dialog-height : Dialog height
- in property <float> dialog-title-height : Dialog title height
- in property <float> dialog-view-height : Dialog view height
- in property <float> btn-view-height : Dialog button view height
- in property <float> dialog-width : Dialog width
- in property <length> dialog-details-padding-top: Dialog details padding top
- in property <length> dialog-details-padding-bottom: Dialog details padding bottom
- in property <length> dialog-details-padding-left: Dialog details padding left
- in property <length> dialog-details-padding-right: Dialog details padding right
- in property <PaddingType> padding-type: Dialog padding type
- in property <ShadowType> shadow-type: Dialog shadow type
- in property <BorderType> border-type : Dialog border type
- in property <length> viewport-height : Dialog viewport height
- in property <length> viewport-width : Dialog viewport width
- in property <LayoutAlignment> dialog-details-alignment: Dialog details alignment
## functions
- public function open() : open dialog
- public function close() : close dialog
## callbacks
- callback confirm() : run after confirm button click
- callback cancel() : run after cancel button click
## example
```
import {SDialog,SButton,STable,STableColumn} from "../../index.slint";
import {Themes} from "../../use/index.slint";

component TestDialog inherits Window {
  height: 600px;
  width: 600px;
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
    dialog-title: "Surrealism Dialog Title";
    dialog-title-size: 20px;
    dialog-details-padding-top : 16px;
    // do after confirm btn clicked
    viewport-height : dialog-detail.height;
    // viewport-width : dialog-detail.width;
    confirm=>{
      debug("confirm btn clicked~!")
    }
    dialog-detail:=Rectangle {
      height: 200px;
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
}
```
# `SDrawer` 抽屉
Sometimes, the Dialogue component does not meet our needs
such as your form being too long, or if you need to temporarily display some documents, please use the SDrawer
## properties
- in property <Themes> drawer-theme : drawer theme 
- in property <brush> drawer-background-color : drawer background color
- in property <PaddingType> padding-type: drawer padding type
- in property <Position> position : the position of the drawer
- in property <percent> proportion : the percentage of the drawer
## functions
- function default-height-width()->{height:percent,width:percent} : count drawer height and width ⛔
- function get-position()->{x:length,y:length} : count position ⛔
## example
```
import {SDrawer,SButton, SInput} from "../../index.slint";
import {Themes} from "../../use/index.slint";

component TestDrawer inherits Window {
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
 

  p:=SDrawer {
    proportion:40%;
    drawer-theme: Dark;
    SButton {
      theme: Dark;
    }
    SInput { 
      y: 30px;
     }
  }
}
```

# `SAlert` 通知
SAlert is used to display important prompt information on the page
## properties
- in property <int> font-weight :font weight;
- in property <length> font-size: font size;
- in property <brush> font-color : font color;
- in property <bool> font-italic : font italic;
- in property <string> font-family : font family;
- in property <TextOverflow> overflow : text overflow;
- in property <length> spacing : spacing among icons and text in alert;
- in-out property <string> text : display text in alert;
- in-out property <bool> is-show : is alert show or not;
- in property <length> alert-height : alert height;
- in-out property <ResultType> result-type: the result type of the alert;
- in property <image> close-icon : close icon;
- in property <length> icon-size : icon size;
## functions
- public function open() : open alert
- public function close() : close alert
- public function success(text:string) : open success alert
- public function warning(text:string) : open warning alert
- public function error(text:string) : open error alert
- public function info(text:string) : open info alert
- public function help(text:string) : open help alert
- public function primary(text:string) : open primary alert 
## example
```
import {SButton, SAlert} from "../../index.slint";
import {ResultType,Themes,UseIcons} from "../../use/index.slint";

component TestAlert inherits Window {
  height: 400px;
  width: 600px;
  background: #535353;
 
  SButton {
    text: "show";
    clicked => {
      p.error("success->error!!!");
      
      debug("sds1")
    }
  }
 

  p:=SAlert { 
    result-type:ResultType.Success ;
    text:"this is a success message!";
  }
}
```