# `STag` 标签 

A small tag used to display data

## properties inherits SCard

- in property <string> text : text in tag

## functions

- pure public function get() -> string : get tag text
- public function set(value:string) : set tag text

## callbacks
- callback clicked(string) : run if you click the tag

## example

```slint
import {STag} from "../../index.slint";
import {Themes} from "../../use/index.slint";

component TestWindow inherits Window {
  height: 400px;
  width: 400px;
  VerticalLayout {
    spacing: 20px;
    padding: 20px;
    STag {
      text:"default";
      clicked(text) => {
        debug(self.get());
        self.set(@tr("clicked -> {}",text));
      }
    }
    STag {
      text:"error!";
      theme:Themes.Error;
    }
    STag {
      text:"callback";
      theme:Themes.Dark;
      clicked(text)=>{
        self.font-color= #ddff00;
      }
    }
    STag {
      text:"success";
      theme:Themes.Success;
    }
  }
}
```
# `STable` 表格
In fact, it is just the header of a table and needs to be used in conjunction with STableColumn
## properties inherits SCard
- in property <[Themes]> column-themes: table header columns' theme;
- in property <length> viewport-height: table body viewport height
- in property <TextHorizontalAlignment> alignment : table header horizontal alignment
- in property <[SOption]> columns : table columns
- in-out property <[length]> column-width : table column width
## functions
- pure public function get-column-width(w:length,index:int)->length : get each column width depand on the index
## callbacks
- callback clicked(int,SOption) : run if click the Table Header

***
## STableColumn
STableColumn is table body , it covers the data of the table
### properties inherits SCard
- in-out property <int> index : column index
- in property <[string]> datas : column datas
- in property <TextHorizontalAlignment> alignment : row text horizontal alignment
### functions
- pure public function count-column-height()->length : count column height
### callbacks
- callback clicked(int,int,string) : run if click the row item

## example
```slint
import {STable,STableColumn, SCard} from "../../index.slint";
import {Themes,PaddingType,ShadowType,BorderType,PaddingProps,BorderProps,ShadowProps,UseSurrealismFn} from "../../use/index.slint";
import { ROOT-STYLES,DefaultSCardProps } from "../../themes/index.slint";
import { ScrollView } from "std-widgets.slint";

export component TestTable inherits Window {
  height: 500px;
  width: 500px;
  STable{
    theme: Dark;
    width: 86%;
    height: 36%;
    column-themes:[Themes.Primary,Themes.Dark,Themes.Error];
    viewport-height:col1.height;
    alignment: center;
    columns: [
      {label:"序号",value:"$index"},
      {label:"username",value:"name"},
      {label:"age",value:"age"},
    ];
    clicked(index,item)=>{
      debug(index);
      debug(item);
    }
    
    col1:=STableColumn {
      alignment: left;
      datas:[
        "1",
        "2",
        "3",
        "3",
        "3",
      ];
      width: parent.get-column-width(parent.width , 0);
    }
    STableColumn {
      index:1;
      width: parent.get-column-width(parent.width , 1);
      datas:[
        "Matt",
        "John",
        "Gary",
        "Harry",
        "Mary",
      ];
      clicked(col-index,index,value)=>{
        debug(col-index);
        debug(index);
        debug(value);
      }
    }
    STableColumn {
      theme: Light;
      width: parent.get-column-width(parent.width , 2);
      datas:[
        "16",
        "12",
        "19",
        "21",
        "11",
      ];
    }
  }
}
```

# `SCollapse` 折叠面板

SCollapse is a foldable panel

This is the outter of the Collapse, what really works is SCollapseItem

The outter only serves as a standard layout , this is a zero cost construction

## properties inherits Rectangle

***
## SCollapseItem
SCollapseItem is a component of SCollapse, without which SCollapse will not work

You can customize the components or use the default text display method in it
### properties
- in-out property <int> font-weight : font weight
- in-out property <length> font-size: font size
- in-out property <brush> font-color : font color
- in-out property <bool> font-italic : font italic
- in-out property <string> font-family : font family
- in-out property <Themes> theme : SurrealismUI theme
- in-out property <length> header-height : collapse header height
- in-out property <string> header-title : collapse header title
- in-out property <PaddingType> header-padding-type: collapse header padding type
- in-out property <ShadowType> header-shadow-type: collapse header shadow type
- in-out property <BorderType> header-border-type : collapse header border type
- in-out property <length> details-height : collapse detail height
- in-out property <PaddingType> details-padding-type: collapse detail padding type
- in-out property <ShadowType> details-shadow-type: collapse detail shadow type
- in-out property <BorderType> details-border-type : collapse detail border type
- in-out property <bool> is-show : the collapse detail is show or not;
- in-out property <image> collapse-icon : collapse header expand icon
### functions
- pure public function get-height()->length : get collapse header height
### callbacks
- callback clicked() : run if you show collapse detail

## example

```slint
import {SCollapse,SCollapseItem,SButton,STable,STableColumn} from "../../index.slint";
import { UseIcons,Themes } from "../../use/index.slint";
import { SText } from "../../src/text/index.slint";

component TestWindow inherits Window {
  height: 500px;
  width: 400px;
  SCollapse {
    y: 10px;
    // you can set 0 , it has no impact
    // recommend use the following way
    height: item1.get-height() * 3;
    width: 360px;
    item1:=SCollapseItem {
      header-title:"Feedback";
      SText {
        wrap: word-wrap;
        height: item1.details-height;
        width: item1.width;
        text:"Operation feedback: enable the users to clearly perceive their operations by style updates and interactive effects";
      }
    }
    SCollapseItem {
      theme: Themes.Error;
      
      SButton { 

      }
    }
    SCollapseItem {
      header-title:"table";
      theme: Themes.Dark;
      details-height:280px;
      STable{
        theme: Dark;
        width: 86%;
        height: 80%;
        column-themes:[Themes.Dark,Themes.Dark,Themes.Dark];
        viewport-height: col1.height;
        alignment: center;
        columns: [
          {label:"序号",value:"$index"},
          {label:"username",value:"name"},
          {label:"age",value:"age"},
        ];
        clicked(index,item)=>{
          debug(index);
          debug(item);
        }
        
        col1:=STableColumn {
          alignment: left;
          datas:[
            "1",
            "2",
            "3",
            "3",
            "3",
          ];
          width: parent.get-column-width(parent.width , 0);
        }
        STableColumn {
          index:1;
          width: parent.get-column-width(parent.width , 1);
          datas:[
            "Matt",
            "John",
            "Gary",
            "Harry",
            "Mary",
          ];
          clicked(col-index,index,value)=>{
            debug(col-index);
            debug(index);
            debug(value);
          }
        }
        STableColumn {
          theme: Light;
          width: parent.get-column-width(parent.width , 2);
          datas:[
            "16",
            "12",
            "19",
            "21",
            "11",
          ];
        }
      }
    }
  }
}
```
# `SAvatar` 头像
SAvatar is a avatar component that defaults to Icons.Avatar when there are no images available
## properties inherits SCard
- in property <length> avatar-size : avatar image size;
- in property <image> avatar : avatar image;
- in-out property <image> alt : if the image can be displayed the default alt will instead;
- in property <ImageFit> image-fit : image fit;
## examples
```slint
import {SAvatar} from "../../index.slint";
import {ROOT-STYLES} from "../../themes/index.slint";

component TestWindow inherits Window {
  height: 400px;
  width: 400px;
  background: #F5F5F5;
  VerticalLayout {
    padding: 20px;
    spacing: 20px;
    SAvatar {
    }
    SAvatar {
      avatar-size : ROOT-STYLES.sur-size.small * 2;
      padding-type : Small;
      theme: Primary;
    }
    SAvatar {
      theme: Warning;
    }
    SAvatar {
      avatar-size : ROOT-STYLES.sur-size.small * 2;
      padding-type : Small;
      theme: Error;
    }
    SAvatar {
      avatar-size : ROOT-STYLES.sur-size.large * 2;
      padding-type : Large;
      theme: Dark;
      avatar:@image-url("../../README/imgs/logo.png");
    }
  }
}
```
# `SCollection` 收缩盒
SCollection is an expandable box that can be zoomed in or out by clicking (internal can also be used)
## properties (card)
- in property <float> scale : collection scale size;
- in-out property <bool> is-scale : collection is scale or not;
- in property <easing> easing : animation easing type;
- in property <duration> duration : animation duration;
## functions
- pure public function toggle-default(target:length)->length
- pure public function toggle(target:length,scale-size:float)->length
## callbacks
- clicked() : run if you click item in SCollection
## examples
```slint
import {SButton,SCollection, SText} from "../../index.slint";
import {Themes} from "../../use/index.slint";

component TestCollection inherits Window {
  height: 600px;
  width: 600px;
  
  c:=SCollection{
    height: 180px;
    width: 180px;
    scale : 3;
    clicked => {
      txt.font-size = self.toggle(txt.font-size,1.5);
      btn.width = self.toggle(btn.width , 1.6);
      btn.theme = c.is-scale ? Themes.Dark : Themes.Error;
    }
    VerticalLayout {
      padding: 20px;
      spacing: 20px;
      alignment: center;
      Rectangle {
        txt:=SText {
          text:"Surrealism";
        }
      }
      Rectangle {
        btn:=SButton{
        }
      }
    }
  }
}
```

# `SPersona` 个人信息
This component is used to display simple user introduction information
## properties inherits SCard
- in-out property <string> btn-text : button text
- in property <length> spacing : spacing of persona 
- in property <[SButtonProps]> btns : buttons slot
- in property <image> avatar : persona avatar image
- in property <length> avatar-height: persona avatar height
- in property <Themes> avatar-theme : persona avatar theme
- in-out property <string> name : persona name
- in-out property <length> name-height: persona name height
- in-out property <length> name-font-size: persona name font size
- in-out property <int> name-font-weight : persona name font weight
- in-out property <Themes> name-theme: persona name theme
- in-out property <string> name-font-family : persona name font family
- in-out property <bool> name-font-italic : persona name font italic
- in-out property <string> des : persona description text
- in-out property <length> des-height: persona description height
- in-out property <length> des-font-size: persona description font size
- in-out property <int> des-font-weight : persona description font weight
- in-out property <Themes> des-theme: persona description theme
- in-out property <string> des-font-family : persona description font family
- in-out property <bool> des-font-italic : persona description font italic
## functions
## callbacks
- callback clicked(SButtonProps): run if you click the buttons
## example
```slint
import {SPersona} from "../../index.slint";
import {Themes,UseIcons} from "../../use/index.slint";
import { ComponentSchema,DefaultSButtonProps } from "../../themes/index.slint";

component TestPersona inherits Window {
  height: 500px;
  width: 600px;
  SPersona {
    x: 20px;
    avatar: @image-url("../../README/imgs/logo.png");
    avatar-height: 180px;
    name: "SurrealismUI";
    name-font-italic: true;
    name-font-weight: 900;
    des: @tr("A third-party UI library using Slint, I think it will give you an extraordinary experience");
    des-font-size: 14px;
    des-theme: Primary;
    btns: [
      ComponentSchema.button,
      {
        font-weight :700,
        font-size : DefaultSButtonProps.font-size,
        color : DefaultSButtonProps.color,
        font-italic : DefaultSButtonProps.font-italic,
        font-family : DefaultSButtonProps.font-family,
        theme : Themes.Primary,
        padding-type : DefaultSButtonProps.padding-type,
        shadow-type : DefaultSButtonProps.shadow-type,
        border-type : DefaultSButtonProps.border-type,
        icon :  UseIcons.icons.Help,
        show-icon : true,
        text : "Addition",
        letter-spacing : DefaultSButtonProps.letter-spacing,
        clip :DefaultSButtonProps.clip,
        round : true
      }
    ];
    clicked(e) => {
      debug(e);
    }
  }
  SPersona {
    x: 300px;
    btn-text: "GitHub GO!";
  }
}
```
# `SBadge` 勋章
SBadge is a quick way to display user status or events
## properties inherits SCard
-  in property <Position> position : badge position of the main component
-  in-out property <image> icon : badge icon;
-  in property <brush> icon-color : badge icon color;
-  in-out property <string> text : text of the badge;
## functions
- pure public function get-x(p_right:length)->length 👍
- pure public function get-y(p_bottom:length)->length 👍
## callbacks
## example
```slint
import {SBadge,SAvatar} from "../../index.slint";
import {Themes} from "../../use/index.slint";

component TestCollection inherits Window {
  height: 460px;
  width: 400px;
  
  b1:=Rectangle {
    y: 30px;
    height: avatar.height;
    width: avatar.width;
    avatar:=SAvatar {
    } 
    SBadge {
      text : "this is a badge";
      x: self.get-x(avatar.width);
      y: self.get-y(avatar.height);
    }
  }
  b2:=Rectangle {
    y: 120px;
    height: avatar2.height;
    width: avatar2.width;
    avatar2:=SAvatar {
    } 
    SBadge {
      theme: Primary;
      text:"theme primary";
      x: self.get-x(avatar2.width);
      y: self.get-y(avatar2.height);
      position: LeftBottom;
    }
  }
  b3:=Rectangle {
    y: 210px;
    height: avatar3.height;
    width: avatar3.width;
    avatar3:=SAvatar {
    } 
    SBadge {
      theme: Light;
      text:"theme light";
      x: self.get-x(avatar3.width);
      y: self.get-y(avatar3.height);
      position: LeftTop;
      icon-color:#ff0000;
      font-color:#ff0000;
    }
  }
  b4:=Rectangle {
    y: 300px;
    height: avatar4.height;
    width: avatar4.width;
    avatar4:=SAvatar {
    } 
    SBadge {
      x: self.get-x(avatar4.width);
      y: self.get-y(avatar4.height);
      position: RightTop;
    }
  }
}
```
# SProgress
SProgress is commonly used to display download progress or event processing progress

And you can fully control it through the progress property
## properties inherits Rectangle
- in property <Themes> theme : progress theme
- in property <string> text : display text
- in-out property <float> progress : progress vaslue
- in-out property <int> font-weight : display text font weight
- in-out property <length> font-size: display text font size
- in-out property <brush> font-color : display text font color
- in-out property <bool> font-italic : display text font italic
- in-out property <string> font-family : display text font family
- private property <length> unit : unit of the progress
## functions
- pure public function get-progress() : get timely progress
- public function full() : make progress 100%
- public function clear() : : make progress 0%
- public function add(num:float) : increase progress
## callbacks
## example
```slint
import {SProgress,SButton} from "../../index.slint";
import {Themes} from "../../use/index.slint";

component TestDivider inherits Window {
  height: 400px;
  width: 400px;
  background: #1e1f2a;
  SProgress {
    y: 100px;
    progress: 10;
  }
  a:=SProgress {
    y: 200px;
    progress: 32;
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
# `STree` 树型组件
STree can be used to display directory structure, forming a parent-child relationship, and can be easily displayed
## properties inherits SCard
- in property <string> item-font-family : tree item font family
- in property <int> item-font-weight : tree item font weight
- in property <length> item-font-size: tree item font size
- in property <bool> item-font-italic : tree item font italic
- in-out property <TreeData> tree-data : tree data
## callbacks
- callback clicked(int,string,string) : run if you click an item
## example
```slint
import {STree } from "../../index.slint";
import {UseIcons} from "../../use/index.slint";

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
# `SFile` 文件
SFile can help users present file selectors GUI
## properties
- in property <TextHorizontalAlignment> text-alignment : file item horizontal alignment
- in property <[SOption]> tabs : file tabs
- in-out property <[length]> column-width : file item column width
- in-out property <[FileItem]> files : file item font files
- in-out property <string> item-font-family : file item font family
- in-out property <int> item-font-weight : file item font weight
- in-out property <length> item-font-size: file item font size
- in-out property <bool> item-font-italic : file item font italic
- in-out property <PaddingType> item-padding-type: file item padding type
## functions
- pure function get-column-width(w:length,index:int)->length : get file item column width
## callbacks
- callback tab-clicked(int,SOption) : run if you click the tab
- callback item-clicked(int,int,FileItem) : run if you click a file item
## example
```slint
import {SFile} from "../../index.slint";
import { Themes,PaddingType,UseIcons,FileItem} from "../../use/index.slint";

export component TestFile inherits Window {
  height: 400px;
  width: 800px;
  SFile{
    theme: Dark;
    width: 86%;
    height: 40%;
    item-font-size: 14px;
    tabs: [
      {label:"名称",value:"name"},
      {label:"时间",value:"dateTime"},
      {label:"文件类型",value:"file-type"},
      {label:"大小",value:"size"}
    ];
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
    item-clicked(index,cindex,item)=>{
      debug(index);
      debug(cindex);
      debug(item);
    }
  }
}
```