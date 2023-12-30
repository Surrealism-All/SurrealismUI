# `SHeader` 页头
SHeader is a simple header component that is generated based on routing information
## properties inherits SCard
- in property <length> spacing : the spacing of header
- in property <[SOption]> value : route value of header
- in property <image> source : split icon of header
## functions
## callbacks
- callback clicked(int,SOption) : run if you click the header
## example
```slint
import {SHeader} from "../../index.slint";
import {Themes} from "../../use/index.slint";

component TestHeader inherits Window {
  width: 600px;
  height: 400px;
  VerticalLayout {
    padding: 30px;
    spacing: 30px;
    SHeader {
      theme: Dark;
    }
    SHeader {
      theme: Error;
    }
    SHeader {
      theme: Primary;
      font-size: 16px;
      clicked(index,node)=>{
        txt.index = index;
        txt.name = node.label;
      }
    }
    txt:=Text{
      font-size: 18px;
      in-out property <int> index;
      in-out property <string> name;
      text: "route-index:" + index + " route-name:" + name;
    }
  }
}
```
# `SMenu` 菜单
SMenu is a menu bar located on the left side that you can quickly generate through the menu-data property
## properties
- in-out property <length> icon-box-size : menu item size ⛔
- in-out property <length> icon-size : menu item icon size ⛔;
- in property <[MenuData]> menu-data : menu item data (generate menus through it)
- in-out property <int> active : which item is active
- private property <brush> hover-icon-color : menu item icon color changed when hover
## callbacks
- callback change(int,MenuData) : run if you click menu item
- callback clicked-account() : run if you click account icon
- callback clicked-setting() : run if you click setting icon
## example
```slint
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
