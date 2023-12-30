## Components

-  `SText` ：It is the simplest and most common component in SurrealismUI
-  `SButton` ：SButton is a button component that you can freely perform regular attribute operations on
-  `SIcon` ：this is a icon container ,  better than Image
-  `SInput` ：This is a basic input box, often used in forms, divided into two types: text and password
-  `SCard` ：A very simple universal card without any layout or restrictions ， you can add anything you want to the card
-  `SStar` ： SStar is a scoring component
-  `SSelect` ：SSelect is a selector that provides three types of optional input parameter values
-  `STag` ：A small tag used to display data
-  `SHeader` ：SHeader is a simple header component that is generated based on routing information
-  `STable` ：This is the outter of the Table, and the column data of the table is separated from the outter . The outter only serves as a standard layout , this is a zero cost construction
-  `STableColumn` ：STableColumn is a component of STable, and each STableColumn forms a complete column of the table . If it's gone, the table will become a card with a horizontal layout
-  `SCollapse` ：SCollapse is a foldable panel. This is the outter of the Collapse, what really works is SCollapseItem. The outter only serves as a standard layout , this is a zero cost construction
-  `SCollapseItem` ：SCollapseItem is a component of SCollapse, without which SCollapse will not work , You can customize the components or use the default text display method in it
-  `SResult` ：SResult helps you easily build a quick prompt , you can build it in popup window
-  `SAvatar` ：SAvatar is a avatar component that defaults to Icons.Avatar when there are no images available
-  `SLink` ：SLink is commonly used to represent text connections or sharing
-  `SDivider` ：A divider groups sections of content to create visual rhythm and hierarchy. Use dividers along with spacing and headers to organize content in your layout. 
-  `SPopup` ：A masked pop-up layer appears in the current window . And users will not be able to use the pop-up layer to cover the components under it. Clicking on the pop-up layer again will close it
-  `SCollection` ：SCollection is an expandable box that can be zoomed in or out by clicking (internal can also be used)
-  `SRadio` ：Radio let people select a single item
-  `SBadge` ：SBadge is a quick way to display user status or events
-  `SPersona` ：This component is used to display simple user introduction information
-  `SProgress` ：SProgress is commonly used to display download progress or event processing progress . And you can fully control it through the progress property
-  `STip` ：A tip provides supplemental, contextual information elevated near its target component
-  `SLoading` ： This is a loading component that you can embed anywhere you want to add a loading animation 
-  `SDialog` ：SDialogs are used to confirm messages or events and display text
-  `SMenu` ：SMenu is a menu bar located on the left side that you can quickly generate through the menu-data property
-  `SSwitch` ：SSwitch is a switch used for simple judgment scenarios
-  `SDrawer` ：Sometimes, the Dialogue component does not meet our needs , such as your form being too long, or if you need to temporarily display some documents, please use the SDrawer
-  `SAlert` ：SAlert is used to display important prompt information on the page
-  `SSwitchGroup` ：SSwitchGroup switch group can contain more switch cases
-  `STree` ：STree can be used to display directory structure, forming a parent-child relationship, and can be easily displayed
-  `SFile` ：SFile can help users present file selectors GUI

## Globals

### ROOT_STYLES

Root Theme Styles

- out property <FontProps> sur-font : SurrealismUI default font styles
- in-out property <length> tag-size : tag size to `STag`
- out property <brush> font-light : font color - light #fff
- out property <brush> font-black : font color - black #000
- in-out property <ThemeColor> sur-theme-colors : SurrealismUI theme colors
- in-out property <brush> radio-active : radio active color
- out property <ThemePadding> sur-padding : theme padding
- out property <ThemeBorder> sur-border : theme border
- out property <{low:{shadow1:percent,shadow2:percent},high:{shadow1:percent,shadow2:percent}}> sur-opacity : theme opacity
- out property <duration> sur-an-duration : theme animation duration
- out property <easing> sur-an-easing : theme animation easing
- in-out property <SizeProps> sur-size : the size of components
- out property <length> scroll-bar-width : scroll bar width
- out property <ThemeShadow> sur-shadow : theme shadow
- out property <ThemeSpace> sur-spacing : theme spacing

### GlobalProps

default global properties

- in-out property <FontProps> font
- in-out property <Themes> theme
- in-out property <TextActionProps>
- in property <TextAlignmentProps> text-alignment
- in-out property <length> line-height
- in-out property <length> standard-height
- in-out property <length> standard-width
- in-out property <length> standard-icon-size
- in-out property <bool> clip
- in-out property <brush> active-color

### DefaultSAlertProps

```
export global DefaultSAlertProps {
  in-out property <int> font-weight : 700;
  in-out property <length> font-size: GlobalProps.font.font-size;
  in-out property <brush> color : GlobalProps.font.color;
  in-out property <bool> font-italic : GlobalProps.font.font-italic;
  in-out property <string> font-family : GlobalProps.font.font-family;
  in-out property <TextOverflow> overflow : TextOverflow.elide;
  in-out property <length> spacing : 16px;
  in-out property <string> text : "this is a alert message!";
  in-out property <bool> is-show : false;
  in-out property <length> alert-height : font-size * 1.5;
  in-out property <ResultType> result-type: ResultType.Success;
  in-out property <image> close-icon : UseIcons.icons.Close-one;
  in-out property <length> icon-size : 16px;
}
```
### DefaultSAvatarProps

```
export global DefaultSAvatarProps {
  in-out property <length> card-height : avatar-size;
  in-out property <length> card-width : avatar-size;
  in-out property <PaddingType> padding-type: PaddingType.Normal;
  in-out property <ShadowType> shadow-type: ShadowType.Low1;
  in-out property <BorderType> border-type : Circle-Normal;
  in-out property <length> avatar-size : ROOT-STYLES.sur-size.normal * 2;
  in-out property <image> avatar;
  in-out property <image> alt : UseIcons.icons.Avatar;
  in-out property <ImageFit> image-fit : ImageFit.cover;
}
```
### DefaultSBadgeProps
```
export global DefaultSBadgeProps {
  //font
  in-out property <int> font-weight : GlobalProps.font.font-weight;
  in-out property <length> font-size: GlobalProps.font.font-size - 2px;
  in-out property <brush> font-color : GlobalProps.font.color;
  in-out property <bool> font-italic : GlobalProps.font.font-italic;
  in-out property <string> font-family : GlobalProps.font.font-family;
  //theme
  in-out property <Themes> theme : GlobalProps.theme;
  //hight-width
  in-out property <length> card-height : font-size;
  in-out property <length> card-width : font-size;
  in-out property <string> text : "";
  in-out property <image> icon : UseIcons.icons.Attention;
  in-out property <Position> position : Position.RightTop;
  in-out property <brush> icon-color : GlobalProps.font.color;
}
```
### DefaultSButtonProps
```
export global DefaultSButtonProps {
  //font
  in-out property <int> font-weight : GlobalProps.font.font-weight;
  in-out property <length> font-size: GlobalProps.font.font-size;
  in-out property <brush> color : GlobalProps.font.color;
  in-out property <bool> font-italic : GlobalProps.font.font-italic;
  in-out property <string> font-family : GlobalProps.font.font-family;
  //theme
  in-out property <Themes> theme : GlobalProps.theme;
  in property <PaddingType> padding-type:PaddingType.Normal;
  in property <ShadowType> shadow-type: ShadowType.Low1;
  in property <BorderType> border-type : BorderType.Normal;
  in property <image> icon;
  in property <bool> show-icon : false;
  in-out property <string> text : "SButton";
  in property <length> letter-spacing : GlobalProps.text-action.letter-spacing;
  in property <bool> clip : GlobalProps.clip;
  in property <bool> round : false;
}
```
### DefaultSCardProps
```
export global DefaultSCardProps {
  //font
  in-out property <int> font-weight : GlobalProps.font.font-weight;
  in-out property <length> font-size: GlobalProps.font.font-size;
  in-out property <brush> font-color : GlobalProps.font.color;
  in-out property <bool> font-italic : GlobalProps.font.font-italic;
  in-out property <string> font-family : GlobalProps.font.font-family;
  //theme
  in-out property <Themes> theme : GlobalProps.theme;
  //hight-width
  in-out property <length> card-height : GlobalProps.standard-height;
  in-out property <length> card-width : GlobalProps.standard-width;
  in-out property <PaddingType> padding-type:PaddingType.Normal;
  in-out property <ShadowType> shadow-type: ShadowType.Low1;
  in-out property <BorderType> border-type : BorderType.Normal;
  in-out property <bool> clip : GlobalProps.clip;
}
```
### DefaultSCollapseProps
```
export global DefaultSCollapseProps {
  //font
  in-out property <int> font-weight : GlobalProps.font.font-weight;
  in-out property <length> font-size: GlobalProps.font.font-size;
  in-out property <brush> font-color : GlobalProps.font.color;
  in-out property <bool> font-italic : GlobalProps.font.font-italic;
  in-out property <string> font-family : GlobalProps.font.font-family;
  //theme
  in-out property <Themes> theme : GlobalProps.theme;
  //header
  in-out property <length> header-height : GlobalProps.font.font-size;
  in-out property <string> header-title : "collapse";
  in-out property <PaddingType> header-padding-type:PaddingType.Normal;
  in-out property <ShadowType> header-shadow-type: ShadowType.Low1;
  in-out property <BorderType> header-border-type : BorderType.Normal;
  //details
  in-out property <length> details-height : 120px;
  in-out property <PaddingType> details-padding-type:PaddingType.Normal;
  in-out property <ShadowType> details-shadow-type: ShadowType.Low1;
  in-out property <BorderType> details-border-type : BorderType.Normal;
  in-out property <bool> is-show : false;
  in-out property <image> collapse-icon : UseIcons.icons.Right-one;
}
```
### DefaultSCollectionProps
```
export global DefaultSCollectionProps {
  in-out property <float> scale : 2;
  in-out property <bool> is-scale : false;
  in-out property <easing> easing : ease-in-out;
  in-out property <duration> duration : 200ms;
}
```
### DefaultSDialogProps
```
export global DefaultSDialogProps {
  //theme
  in-out property <Themes> theme : Dark;
  in-out property <Themes> cancel-btn-theme : Info;
  in-out property <Themes> confirm-btn-theme : Primary;
  in-out property <string> cancel-btn-text : "Cancel";
  in-out property <string> confirm-btn-text : "Confirm";
  in-out property <bool> is-show:false;
  in-out property <percent> mask-opacity : 80%;
  in-out property <length> spacing : 16px;
  //font
  in-out property <int> font-weight : GlobalProps.font.font-weight;
  in-out property <length> font-size: GlobalProps.font.font-size;
  in-out property <brush> font-color : GlobalProps.font.color;
  in-out property <bool> font-italic : GlobalProps.font.font-italic;
  in-out property <string> font-family : GlobalProps.font.font-family;
  //dialog
  in-out property <Themes> dialog-theme : Dark;
  in-out property <string> dialog-title : "Dialog Title";
  in-out property <length> dialog-title-size : 18px;
  in-out property <string> dialog-details : "This is a dialog info";
  in-out property <float> dialog-height : 0.36;
  in-out property <float> dialog-title-height : 0.2;
  in-out property <float> dialog-view-height : 0.6;
  in-out property <float> btn-view-height : 0.2;
  in-out property <float> dialog-width : 0.6;
  in-out property <length> dialog-details-padding-top : 0;
  in-out property <length> dialog-details-padding-bottom : 0;
  in-out property <length> dialog-details-padding-left : 0;
  in-out property <length> dialog-details-padding-right : 0;
  in-out property <LayoutAlignment> dialog-details-alignment : end;
  in-out property <PaddingType> padding-type:PaddingType.Normal;
  in-out property <ShadowType> shadow-type: ShadowType.Low1;
  in-out property <BorderType> border-type : BorderType.Normal;
}
```
### DefaultSDividerProps
```
export global DefaultSDividerProps {
  //theme
  in-out property <Themes> theme : GlobalProps.theme;
  in-out property <length> height : 2px;
  in-out property <length> width : 100%;
  in-out property <PaddingType> padding-type:PaddingType.None;
  in-out property <ShadowType> shadow-type: ShadowType.Low1;
  in-out property <BorderType> border-type : BorderType.None;
}
```
### DefaultSDrawerProps
```
export global DefaultSDrawerProps {
  //theme
  in-out property <Themes> theme : GlobalProps.theme;
  in-out property <bool> is-show : false;
  in-out property <percent> mask-opacity : 80%;
  in-out property <PaddingType> padding-type:PaddingType.Normal;
  in-out property <Themes> drawer-theme : Light;
  in-out property <Position> position : Left;
  in-out property <percent> proportion : 30%;
}
```
### DefaultSFileProps
```
export global DefaultSFileProps {
  //theme
  in-out property <Themes> theme :GlobalProps.theme;
  in-out property <[SOption]> tabs : [];
  in-out property <[length]> column-width : [];
  //tab
  in-out property <string> font-family : GlobalProps.font.font-family;
  in-out property <int> font-weight : 700;
  in-out property <length> font-size: GlobalProps.font.font-size;
  in-out property <brush> font-color : GlobalProps.font.color;
  in-out property <bool> font-italic : GlobalProps.font.font-italic;
  in-out property <PaddingType> padding-type:PaddingType.Normal;
  in-out property <ShadowType> shadow-type: ShadowType.Low1;
  in-out property <BorderType> border-type : BorderType.Normal;
  in-out property <TextHorizontalAlignment> text-alignment: TextHorizontalAlignment.left;
  // item
  in-out property <[FileItem]> files : [];
  in-out property <string> item-font-family : GlobalProps.font.font-family;
  in-out property <int> item-font-weight : GlobalProps.font.font-weight;
  in-out property <length> item-font-size: GlobalProps.font.font-size;
  in-out property <bool> item-font-italic : GlobalProps.font.font-italic;
  in-out property <PaddingType> item-padding-type:PaddingType.Normal;
}
```
### DefaultSHeaderProps
```
export global DefaultSHeaderProps {
  in-out property <length> spacing: 2px;
  in-out property <image> source :UseIcons.icons.Right;
  in-out property <[SOption]> options : [
    {label:"src",value:"src"},
    {label:"header",value:"header"},
    {label:"SHeader",value:"SHeader"}
  ];
  in-out property <int> font-weight : GlobalProps.font.font-weight;
  in-out property <length> font-size: GlobalProps.font.font-size;
  in-out property <brush> font-color : GlobalProps.font.color;
  in-out property <bool> font-italic : GlobalProps.font.font-italic;
  in-out property <string> font-family : GlobalProps.font.font-family;
  //theme
  in-out property <Themes> theme : GlobalProps.theme;
  //hight-width
  in-out property <length> card-height : font-size;
  in-out property <length> card-width : GlobalProps.standard-width;
  in-out property <PaddingType> padding-type:PaddingType.Normal;
  in-out property <ShadowType> shadow-type: ShadowType.Low1;
  in-out property <BorderType> border-type : BorderType.Normal;
}
```
### DefaultSIconProps
```
export global DefaultSIconProps {
  in-out property <MouseCursor> mouse-cursor : MouseCursor.pointer;
  in-out property <Themes> theme : GlobalProps.theme;
  in-out property <length> height : GlobalProps.standard-icon-size;
  in-out property <length> width : GlobalProps.standard-icon-size;
  in-out property <length> padding : 0;
  //image props
  in-out property <image> source;
  in-out property <brush> colorize;
  in property <ImageFit> image-fit : ImageFit.contain;
  in property <ImageRendering> image-rendering : ImageRendering.smooth;
  in-out property <RotationProps> rotation : {
    rotation-angle : 0,
    rotation-origin-x : 0,
    rotation-origin-y: 0,
  };
  in-out property <int> source-clip-x : 0;
  in-out property <int> source-clip-y : 0;
  in-out property <int> source-clip-height : 0;
  in-out property <int> source-clip-width : 0;
}
```
### DefaultSInputProps
```
export global DefaultSInputProps {
  //font
  in-out property <int> font-weight : GlobalProps.font.font-weight;
  in-out property <length> font-size: GlobalProps.font.font-size;
  in-out property <brush> font-color : GlobalProps.font.color;
  in-out property <bool> font-italic : GlobalProps.font.font-italic;
  in-out property <string> font-family : GlobalProps.font.font-family;
  //theme
  in-out property <Themes> theme : GlobalProps.theme;
  //hight-width
  in-out property <length> card-height : font-size;
  in-out property <length> card-width : 18 * font-size;
  in-out property <PaddingType> padding-type:PaddingType.Normal;
  in-out property <ShadowType> shadow-type: ShadowType.Low1;
  in-out property <BorderType> border-type : BorderType.Normal;
 
  in-out property <string> placeholder : "please input";
  in-out property <bool> disabled:false;
  in-out property <bool> clearable:false;
  //use eye-icon
  in-out property <bool> password:false;
  in-out property <bool> has-focus:false;
  in-out property <InputType> type : InputType.text;
  in-out property <brush> icon-color;
  in-out property <string> text :"";
}
```
### DefaultSLinkProps
```
export global DefaultSLinkProps {
  in-out property <image> icon : UseIcons.icons.Share;
  in-out property <bool> funny :  false;
  in-out property <bool> underline : true;
  in-out property <MouseCursor> mouse-cursor : pointer;
  in-out property <Themes> theme : GlobalProps.theme;
  in-out property <length> font-size : GlobalProps.font.font-size;
  in-out property <string> text:"";
  in-out property <int> font-weight : 500;
  in-out property <string> font-family : GlobalProps.font.font-family;
  in-out property <bool> font-italic : GlobalProps.font.font-italic;
}
```
### DefaultSLoadingProps
```
export global DefaultSLoadingProps {
  in-out property <int> font-weight : GlobalProps.font.font-weight;
  in-out property <length> font-size: GlobalProps.font.font-size;
  in-out property <brush> font-color : GlobalProps.font.color;
  in-out property <bool> font-italic : GlobalProps.font.font-italic;
  in-out property <string> font-family : GlobalProps.font.font-family;
  in-out property <float> opacity : 1;
  in-out property <bool> is-show : false;
  in-out property <Themes> theme : GlobalProps.theme;
  in-out property <image> loading-icon : UseIcons.icons.Loading;
  in-out property <duration> duration : 1200ms;
  in-out property <string> text : "Loading ...";
  in-out property <easing> easing : ease-in-out;
  in-out property <int> iteration-count : -1;
}
```
### DefaultSPersonaProps
```
export global DefaultSPersonaProps {
  in-out property <string> btn-text : "Click";
  in-out property <[SButtonProps]> btns : [];
  in-out property <image> avatar : @image-url("");
  in-out property <length> avatar-height:130px;
  in-out property <Themes> avatar-theme : GlobalProps.theme;
  in-out property <length> card-width : 200px;
  in-out property <length> spacing : 8px;
  //name
  in-out property <string> name : "SYF20020816";
  in-out property <length> name-height: GlobalProps.font.font-size * 3;
  in-out property <length> name-font-size: GlobalProps.font.font-size + 2px;
  in-out property <int> name-font-weight : 700;
  in-out property <Themes> name-theme: GlobalProps.theme;
  in-out property <string> name-font-family : GlobalProps.font.font-family;
  in-out property <bool> name-font-italic : GlobalProps.font.font-italic;
  //des
  in-out property <string> des : @tr("A Rust | Vue Developer\nEmail:\nsyf20020816@outlook.com");
  in-out property <length> des-height: des-font-size * 1.5 * 3;
  in-out property <length> des-font-size: GlobalProps.font.font-size - 2px;
  in-out property <int> des-font-weight : GlobalProps.font.font-weight;
  in-out property <Themes> des-theme: GlobalProps.theme;
  in-out property <string> des-font-family : GlobalProps.font.font-family;
  in-out property <bool> des-font-italic : GlobalProps.font.font-italic;
}
```
### DefaultSPopupProps
```
export global DefaultSPopupProps {
  in-out property <bool> is-show : false;
  in-out property <Themes> theme : GlobalProps.theme;
  in-out property <percent> mask-opacity : 80%;
}
```
### DefaultSProgressProps
```
export global DefaultSProgressProps {
  //font
  in-out property <int> font-weight : GlobalProps.font.font-weight;
  in-out property <length> font-size: GlobalProps.font.font-size - 2px;
  in-out property <brush> font-color : GlobalProps.font.color;
  in-out property <bool> font-italic : GlobalProps.font.font-italic;
  in-out property <string> font-family : GlobalProps.font.font-family;
  //theme
  in-out property <Themes> theme : GlobalProps.theme;
  //hight-width
  in-out property <length> height : 8px + font-size * 2;
  in-out property <length> width : 100%;
  in-out property <string> text : @tr("now: {}% used 100%" , progress);
  in-out property <float> progress : 0;
}
```
### DefaultSRadioProps
```
export global DefaultSRadioProps {
  in-out property <int> font-weight : GlobalProps.font.font-weight;
  in-out property <length> font-size: GlobalProps.font.font-size;
  in-out property <brush> color : GlobalProps.font.color;
  in-out property <bool> font-italic : GlobalProps.font.font-italic;
  in-out property <string> font-family : GlobalProps.font.font-family;
  in-out property <length> card-height : GlobalProps.font.font-size;
  in-out property <length> card-width : GlobalProps.font.font-size;
  in-out property <Themes> theme : GlobalProps.theme;
  in-out property <brush> active-color: GlobalProps.active-color;
  in-out property <PaddingType> padding-type: PaddingType.Icon;
  in-out property <ShadowType> shadow-type: ShadowType.Low1;
  in-out property <BorderType> border-type : BorderType.Small;
  in-out property <string> text : "SRadio";
  in-out property <string> value : "radio";
  in-out property <bool> actived : false;
}
```
### DefaultSResultProps
```
export global DefaultSResultProps {
  in-out property <length> card-height : 200px;
  in-out property <length> card-width : 140px;
  in-out property <length> icon-size : 48px;
  in-out property <[SButtonProps]> btns : [];
  in-out property <string> btn-text : "CLICK!";
  in-out property <ResultType> result-type:ResultType.Success;
  in-out property <string> text : "This is a success message!";
  in-out property <PaddingType> padding-type: PaddingType.Normal;
  in-out property <ShadowType> shadow-type: ShadowType.Low1;
  in-out property <BorderType> border-type : BorderType.Normal;
  in-out property <image> icon : UseIcons.icons.Success;
  in-out property <Themes> theme : Success;
  in-out property <int> font-weight : GlobalProps.font.font-weight;
  in-out property <length> font-size: GlobalProps.font.font-size;
  in-out property <brush> color : ROOT-STYLES.sur-theme-colors.success.font;
  in-out property <bool> font-italic : GlobalProps.font.font-italic;
  in-out property <string> font-family : GlobalProps.font.font-family;
}
```
### DefaultSSelectProps
```
export global DefaultSSelectProps {
  //font
  in-out property <int> font-weight : GlobalProps.font.font-weight;
  in-out property <length> font-size: GlobalProps.font.font-size;
  in-out property <bool> font-italic : GlobalProps.font.font-italic;
  in-out property <string> font-family : GlobalProps.font.font-family;
  in-out property <int> item-font-weight : GlobalProps.font.font-weight;
  in-out property <length> item-font-size: GlobalProps.font.font-size;
  in-out property <bool> item-font-italic : GlobalProps.font.font-italic;
  in-out property <string> item-font-family : GlobalProps.font.font-family;
  //theme
  in-out property <Themes> theme : GlobalProps.theme;
  //hight-width
  in-out property <length> card-height : font-size;
  in-out property <length> card-width : 180px;
  in-out property <PaddingType> padding-type:PaddingType.Normal;
  in-out property <ShadowType> shadow-type: ShadowType.Low1;
  in-out property <BorderType> border-type : BorderType.Normal;
  in-out property <[SOption]> options;
  in-out property <string> placeholder : "please select";
  in-out property <bool> is-show : false;
}
```
### DefaultSSwitchProps
```
export global DefaultSSwitchProps {
  //container
  in-out property <length> card-height : 6px;
  in-out property <length> card-width : 24px;
  in-out property <PaddingType> padding-type: PaddingType.Normal;
  in-out property <ShadowType> shadow-type: ShadowType.Low1;
  in-out property <BorderType> border-type : BorderType.Small;
  in-out property <bool> active : false;
  in-out property <Themes> theme:GlobalProps.theme;
  //switch-circle
  in-out property <length> switch-height : 6px + ROOT-STYLES.sur-padding.normal.padding-same;
  in-out property <length> switch-width : 6px + ROOT-STYLES.sur-padding.normal.padding-same;
  in-out property <PaddingType> switch-padding-type: PaddingType.None;
  in-out property <ShadowType> switch-shadow-type: ShadowType.Low1;
  in-out property <BorderType> switch-border-type : Normal;
  in-out property <brush> switch-background-color : ROOT-STYLES.sur-theme-colors.dark.deepest;
  in-out property <brush> switch-border-color : ROOT-STYLES.sur-theme-colors.dark.deepest;
  in-out property <color> switch-drop-shadow-color : ROOT-STYLES.sur-theme-colors.dark.weakest;
}
```
### DefaultSSwitchGroupProps
```
export global DefaultSSwitchGroupProps {
  in-out property <Themes> theme:GlobalProps.theme;
  in-out property <length> card-height : self.font-size / 2;
  in-out property <length> card-width : 140px;
  in-out property <PaddingType> padding-type: PaddingType.Normal;
  in-out property <ShadowType> shadow-type: ShadowType.Low1;
  in-out property <BorderType> border-type : BorderType.None;
  in-out property <string> active ;
  in-out property <[SOption]> switchs : [];
  in-out property <string> font-family : GlobalProps.font.font-family;
  in-out property <int> font-weight : GlobalProps.font.font-weight;
  in-out property <length> font-size: 14px;
  in-out property <bool> font-italic : GlobalProps.font.font-italic;
}
```
### DefaultSTableProps
```
export global DefaultSTableProps {
  //theme
  in-out property <Themes> theme :GlobalProps.theme;
  in-out property <[SOption]> columns : [];
  in-out property <[length]> column-width : [];
  in-out property <[Themes]> column-themes:[];
  in-out property <length> viewport-height;
  //tab
  in-out property <string> font-family : GlobalProps.font.font-family;
  in-out property <int> font-weight : 700;
  in-out property <length> font-size: GlobalProps.font.font-size;
  in-out property <brush> font-color : GlobalProps.font.color;
  in-out property <bool> font-italic : GlobalProps.font.font-italic;
  in-out property <PaddingType> padding-type:PaddingType.Normal;
  in-out property <ShadowType> shadow-type: ShadowType.Low1;
  in-out property <BorderType> border-type : BorderType.None;
  in-out property <TextHorizontalAlignment> alignment: TextHorizontalAlignment.left;
}
```
### DefaultSTableColumnProps
```
export global DefaultSTableColumnProps {
  //theme
  in-out property <Themes> theme :GlobalProps.theme;
  in-out property <int> index;
  in-out property <[string]> datas;
  in-out property <length> height;
  in-out property <length> width;
  //tab
  in-out property <string> font-family : GlobalProps.font.font-family;
  in-out property <int> font-weight : GlobalProps.font.font-weight;
  in-out property <length> font-size: GlobalProps.font.font-size;
  in-out property <brush> font-color : GlobalProps.font.color;
  in-out property <bool> font-italic : GlobalProps.font.font-italic;
  in-out property <PaddingType> padding-type:PaddingType.Normal;
  in-out property <ShadowType> shadow-type: ShadowType.Low1;
  in-out property <BorderType> border-type : BorderType.None;
  in-out property <TextHorizontalAlignment> alignment: TextHorizontalAlignment.left;
}
```
### DefaultSTagProps
```
export global DefaultSTagProps {
  in-out property <string> text : "";
  in property <length> font-size : ROOT-STYLES.tag-size;
  in-out property <int> font-weight : GlobalProps.font.font-weight;
  in-out property <string> font-family : GlobalProps.font.font-family;
  in-out property <bool> font-italic : GlobalProps.font.font-italic;
  in-out property <PaddingType> padding-type : PaddingType.Tag;
  in-out property <BorderType> border-type : BorderType.Normal;
  in-out property <ShadowType> shadow-type : ShadowType.Low1;
  in-out property <Themes> theme : GlobalProps.theme;
}
```
### DefaultSTextProps
```
export global DefaultSTextProps {
  //font
  in-out property <string> font-family : GlobalProps.font.font-family;
  in-out property <int> font-weight : GlobalProps.font.font-weight;
  in-out property <length> font-size: GlobalProps.font.font-size;
  in-out property <brush> color : GlobalProps.font.color;
  in-out property <bool> font-italic : GlobalProps.font.font-italic;
  //theme
  in-out property <Themes> theme : GlobalProps.theme;
  in-out property <TextWrap> wrap :GlobalProps.text-action.wrap;
  in-out property <TextOverflow> overflow : GlobalProps.text-action.overflow;
  in-out property <length> letter-spacing : GlobalProps.text-action.letter-spacing;
  in-out property <TextHorizontalAlignment> horizontal-alignment : GlobalProps.text-alignment.horizontal-alignment;
  in-out property <TextVerticalAlignment> vertical-alignment : GlobalProps.text-alignment.vertical-alignment;
}
```
### DefaultSTipProps
```
export global DefaultSTipProps {
  //font
  in-out property <string> font-family : GlobalProps.font.font-family;
  in-out property <int> font-weight : GlobalProps.font.font-weight;
  in-out property <length> font-size: GlobalProps.font.font-size;
  in-out property <brush> font-color : GlobalProps.font.color;
  in-out property <bool> font-italic : GlobalProps.font.font-italic;
  //theme
  in-out property <Themes> theme : GlobalProps.theme;
  in-out property <TextWrap> wrap :word-wrap;
  in-out property <TextOverflow> overflow : GlobalProps.text-action.overflow;
  in-out property <length> letter-spacing : GlobalProps.text-action.letter-spacing;
  in-out property <TextHorizontalAlignment> horizontal-alignment : GlobalProps.text-alignment.horizontal-alignment;
  in-out property <TextVerticalAlignment> vertical-alignment : GlobalProps.text-alignment.vertical-alignment;
  in-out property <Position> position : Top;
  in-out property <bool> is-show : false;
  in-out property <string> text : "default tips";
  in-out property <length> tip-width : 0;
}
```
### DefaultSTreeProps
```
export global DefaultSTreeProps {
  //font
  in-out property <string> font-family : GlobalProps.font.font-family;
  in-out property <int> font-weight : 700;
  in-out property <length> font-size: GlobalProps.font.font-size;
  in-out property <bool> font-italic : GlobalProps.font.font-italic;
  //font
  in-out property <string> item-font-family : GlobalProps.font.font-family;
  in-out property <int> item-font-weight : GlobalProps.font.font-weight;
  in-out property <length> item-font-size: GlobalProps.font.font-size - 2px;
  in-out property <bool> item-font-italic : GlobalProps.font.font-italic;
  //theme
  in-out property <Themes> theme : GlobalProps.theme;
  //hight-width
  in-out property <length> height : 100%;
  in-out property <length> width : 100%;
  in-out property <PaddingType> padding-type:PaddingType.Normal;
  in-out property <ShadowType> shadow-type: ShadowType.Low1;
  in-out property <BorderType> border-type : BorderType.Normal;
  in-out property <TreeData> tree-data : {
    icon : UseIcons.icons.Folder,
    label: "parent_folder",
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
      }
    ]
  };
}
```
## Structs

### FontProps
- font-family (string) : font family
- font-size (length) : font size (16px👍)
- font-weight (int) : font weight [100,900] 500==Normal 
- font-italic (bool) : font italic
- color (brush) : font color
### ThemeColor
- light (ColorProps) : theme light
- primary (ColorProps) : theme primary
- success (ColorProps) : theme success
- info (ColorProps) : theme info
- warning (ColorProps) : theme warning
- error (ColorProps) : theme error
- dark (ColorProps) : theme dark
#### ColorProps
- name (string) : color name
- weakest (brush) : weakest color
- weaker (brush) : weaker color
- normal (brush) : normal color
- deeper (brush) : deeper color
- deepest (brush) : deepest color
- font (brush) : font color
- opacity (brush) : opacity color
### ThemePadding
- none (PaddingProps) : theme padding none
- tip (PaddingProps) : theme padding tip
- tag (PaddingProps) : theme padding tag
- icon (PaddingProps) : theme padding icon
- small (PaddingProps) : theme padding small
- normal (PaddingProps) : theme padding normal
- large (PaddingProps) : theme padding large
#### PaddingProps 
- padding-top (length) : padding top
- padding-bottom: (length) : padding bottom
- padding-left (length) : padding left
- padding-right (length) : padding right
- padding-same (length) : padding
### ThemeBorder
- none (BorderProps) : no border 
- small (BorderProps) : small border 
- normal (BorderProps) : normal border 
- large (BorderProps) : large border 
- x-large (BorderProps) : x-large border 
- circle:`({
      none:BorderProps,
      small:BorderProps,
      normal:BorderProps,
      large:BorderProps,
      x-large:BorderProps,
    })` : circle border 
#### BorderProps
- border-radius (length) : border radius
- border-width (length) : border width
- border-color (brush) : color of border
### SizeProps
- small (length) : size small
- normal (length) : size normal
- large (length) : size large
- largest (length) : size largest
### ThemeShadow
- low1 (ShadowProps) : lowest shadow
- low2 (ShadowProps) : lower shadow
- low3 (ShadowProps) : low shadow
- high1 (ShadowProps) : high shadow
- high2 (ShadowProps) : higher shadow
- high-empty (ShadowProps) : high blur but no x and y shadow
#### ShadowProps
- x (length) : shadow x
- y (length) : shadow y
- blur (length) : shadow blur
### ThemeSpace
- none (length) : spacing when width == none(0px)
- len20 (length) : spacing when width == 20px
- len40 (length) : spacing when width == 40px
- len60 (length) : spacing when width == 60px
- len80 (length) : spacing when width == 80px
- len120 (length) : spacing when width == 120px
- len160 (length) : spacing when width == 160px
- len200 (length) : spacing when width == 200px
- len240 (length) : spacing when width == 240px
- len280 (length) : spacing when width == 280px
- len320 (length) : spacing when width == 320px
- len360 (length) : spacing when width == 360px
- len400 (length) : spacing when width == 400px
- len440 (length) : spacing when width == 440px
- len480 (length) : spacing when width == 480px
- len520 (length) : spacing when width == 520px
- len560 (length) : spacing when width == 560px

### SAlertProps
```
export struct SAlertProps {
  font-weight : int,
  font-size: length,
  color : brush,
  font-italic : bool,
  font-family : string,
  overflow : TextOverflow,
  spacing : length,
  text : string,
  is-show : bool,
  alert-height : length,
  result-type: ResultType,
  close-icon : image,
  icon-size : length,
}
```
### SAvatarProps
```
export struct SAvatarProps {
  card-height : length,
  card-width : length,
  padding-type: PaddingType,
  shadow-type: ShadowType,
  border-type : BorderType,
  avatar-size : length,
  avatar : image,
  alt : image,
  image-fit : ImageFit,
}
```
### SBadgeProps
```
export struct SBadgeProps {
  //font
  font-weight : int,
  font-size: length,
  font-color : brush,
  font-italic : bool,
  font-family : string,
  //theme
  theme : Themes,
  //hight-width
  card-height : length,
  card-width : length,
  text : string,
  icon : image,
  position :Position,
  icon-color : brush,
}
```
### SButtonProps
```
export struct SButtonProps {
  font-weight : int,
  font-size : length,
  color : brush,
  font-italic : bool,
  font-family : string,
  theme : Themes,
  padding-type : PaddingType,
  shadow-type : ShadowType,
  border-type : BorderType,
  icon : image,
  show-icon : bool,
  text : string,
  letter-spacing : length,
  clip : bool,
  round : bool
}
```
### SCardProps
```
export struct SCardProps {
  //font
  font-weight : int,
  font-size: length,
  font-color : brush,
  font-italic : bool,
  font-family : string,
  //theme
  theme : Themes,
  //hight-width
  card-height : length,
  card-width : length,
  padding-type: PaddingType,
  shadow-type: ShadowType,
  border-type : BorderType,
  clip : bool,
}
```
### SCollapseProps
```
export struct SCollapseProps {
  //font
  font-weight : int,
  font-size: length,
  font-color : brush,
  font-italic : bool,
  font-family : string,
  //theme
  theme : Themes,
  //header
  header-height : length,
  header-title : string,
  header-padding-type: PaddingType,
  header-shadow-type: ShadowType,
  header-border-type : BorderType,
  //details
  details-height : length,
  details-padding-type: PaddingType,
  details-shadow-type: ShadowType,
  details-border-type : BorderType,
  is-show : bool,
  collapse-icon : image,
}
```
### SCollectionProps
```
export struct SCollectionProps {
  scale : float,
  is-scale : bool,
  easing : easing,
  duration : duration,
}
```
### SDialogProps
```
export struct SDialogProps {
  //theme
  theme : Themes,
  cancel-btn-theme : Themes,
  confirm-btn-theme : Themes,
  cancel-btn-text : string,
  confirm-btn-text : string,
  is-show: bool,
  mask-opacity : percent,
  spacing : length,
  //font
  font-weight : int,
  font-size: length,
  font-color : brush,
  font-italic : bool,
  font-family : string,
  //dialog
  dialog-theme : Themes,
  dialog-title : string,
  dialog-title-size : length,
  dialog-details : string,
  dialog-height : float,
  dialog-title-height : float,
  dialog-view-height : float,
  btn-view-height : float,
  dialog-width : float,
  dialog-details-padding-top : length,
  dialog-details-padding-bottom : length,
  dialog-details-padding-left : length,
  dialog-details-padding-right : length,
  dialog-details-alignment : LayoutAlignment,
  padding-type:PaddingType,
  shadow-type: ShadowType,
  border-type : BorderType,
}
```
### SDividerProps
```
export struct SDividerProps {
  //theme
  theme : Themes,
  height : length,
  width : length,
  padding-type: PaddingType,
  shadow-type: ShadowType,
  border-type : BorderType,
}
```
### SDrawerProps
```
export struct SDrawerProps {
  theme : Themes,
  is-show : bool,
  mask-opacity : percent,
  padding-type: PaddingType,
  drawer-theme : Themes,
  position : Position,
  proportion : percent,
}
```
### FileItem
```
export struct FileItem {
  icon:image,
  name:string,
  datetime:string,
  file-type:string,
  size:string,
}
```
### SFileProps
```
export struct SFileProps {
  //theme
  theme : Themes,
  tabs : [SOption],
  column-width : [length],
  //tab
  font-family : string,
  font-weight : int,
  font-size: length,
  font-color : brush,
  font-italic : bool,
  padding-type:PaddingType,
  shadow-type: ShadowType,
  border-type : BorderType,
  text-alignment: TextHorizontalAlignment,
  // item
  files : [FileItem],
  item-font-family : string,
  item-font-weight : int,
  item-font-size: length,
  item-font-italic : bool,
  item-padding-type:PaddingType,
}
```
### SHeaderProps
```
export struct SHeaderProps {
  spacing: length,
  source : image,
  options : [SOption],
  font-weight : int,
  font-size: length,
  font-color : brush,
  font-italic : bool,
  font-family : string,
  //theme
  theme : Themes,
  //hight-width
  card-height : length,
  card-width : length,
  padding-type: PaddingType,
  shadow-type: ShadowType,
  border-type : BorderType,
}
```
### Icons
```
struct Icons {
  Null:image,
  Loading:image,
  Avatar:image,
  Success:image,
  Smiling_face:image,
  Info:image,
  Close_one:image,
  Attention:image,
  Help:image,
  Share:image,
  Up:image,
  Down:image,
  Down_one:image,
  Right:image,
  Right_one:image,
  Link_left:image,
  Preview_close:image,
  Preview_open:image,
  Close_one:image,
  Setting_two:image,
  Folder:image,
  Folder_filled:image,
  FileCode:image
}
```
### IconProps
```
struct IconProps {
  name:string,
  source:image
}
```
### SIconProps
```
struct SIconProps {
  mouse-cursor : MouseCursor,
  theme : Themes,
  height : length,
  width : length,
  padding : length,
  //image props
  source : image,
  colorize : brush,
  image-fit : ImageFit,
  image-rendering : ImageRendering,
  rotation : RotationProps,
  source-clip-x : int,
  source-clip-y : int,
  source-clip-height : int,
  source-clip-width : int
}
```
### SInputProps
```
export struct SInputProps {
  //font
  font-weight : int,
  font-size: length,
  font-color : brush,
  font-italic : bool,
  font-family : string,
  //theme
  theme : Themes,
  //hight-width
  card-height : length,
  card-width : length,
  padding-type:PaddingType,
  shadow-type: ShadowType,
  border-type : BorderType,
  placeholder : string,
  disabled:bool,
  clearable:bool,
  //use eye-icon
  password:bool,
  has-focus:bool,
  type : InputType,
  icon-color : brush,
  text : string,
}
```
### SLinkProps
```
export struct SLinkProps {
  icon : image,
  funny :  bool,
  underline : bool,
  mouse-cursor : MouseCursor,
  theme : Themes,
  font-size : length,
  text:string,
  font-weight : int,
  font-family : string,
  font-italic : bool,
}
```
### SLoadingProps
```
export struct SLoadingProps {
  font-weight : int,
  font-size: length,
  font-color : brush,
  font-italic : bool,
  font-family : string,
  opacity : float,
  is-show : bool,
  theme : Themes,
  loading-icon : image,
  duration : duration,
  text : string,
  easing : easing,
  iteration-count : int,
}
```
### SPersonaProps
```
export struct SPersonaProps {
  btn-text : string,
  btns : [SButtonProps],
  //avatar
  avatar : image,
  avatar-height: length,
  avatar-theme : Themes,
  card-width : length,
  spacing : length,
  //name
  name : string,
  name-height: length,
  name-font-size: length,
  name-font-weight : int,
  name-theme: Themes,
  name-font-family: string,
  name-font-italic: bool,
  //des
  des : string,
  des-height: length,
  des-font-size: length,
  des-font-weight : int,
  des-theme: Themes,
  des-font-family: string,
  des-font-italic: bool,
}
```
### SPopupProps
```
export struct SPopupProps {
  is-show : bool,
  theme : Themes,
  mask-opacity : percent,
}
```
### SProgressProps
```
export struct SProgressProps {
  //font
  font-weight : int,
  font-size: length,
  font-color : brush,
  font-italic : bool,
  font-family : string,
  //theme
  theme : Themes,
  //hight-width
  height : length,
  width : length,
  text : string,
  progress : float,
}
```
### SRadioProps
```
export struct SRadioProps {
  font-weight : int,
  font-size: length,
  color : brush,
  font-italic : bool,
  font-family : string,
  card-height : length,
  card-width : length,
  theme : Themes,
  active-color: brush,
  padding-type: PaddingType,
  shadow-type: ShadowType,
  border-type : BorderType,
  text : string,
  value : string,
  actived : bool,
}
```
### SResultProps
```
export struct SResultProps {
  card-height : length,
  card-width : length,
  icon-size : length,
  btns : [SButtonProps],
  btn-text : string,
  result-type: ResultType,
  text : string,
  padding-type: PaddingType,
  shadow-type: ShadowType,
  border-type : BorderType,
  icon : image,
  theme : Themes,
}
```
### SSelectProps
```
export struct SSelectProps {
  //font
  font-weight : int,
  font-size: length,
  font-italic : bool,
  font-family : string,
  item-font-weight : int,
  item-font-size: length,
  item-font-italic : bool,
  item-font-family : string,
  //theme
  theme : Themes,
  //hight-width
  card-height : length,
  card-width : length,
  padding-type:PaddingType,
  shadow-type: ShadowType,
  border-type : BorderType,
  options : [SOption],
  placeholder : string,
  is-show : bool,
}
```
### SOption
```
export struct SOption {label:string,value:string}
```
### SSwitchProps
```
export struct SSwitchProps {
  //container
  card-height : length,
  card-width : length,
  padding-type: PaddingType,
  shadow-type: ShadowType,
  border-type : BorderType,
  active : bool,
  theme: Themes,
  //switch-circle
  switch-height : length,
  switch-width : length,
  switch-padding-type: PaddingType,
  switch-shadow-type: ShadowType,
  switch-border-type : BorderType,
  switch-background-color : brush,
  switch-border-color : brush,
  switch-drop-shadow-color : color,
}
```
### SSwitchGroupProps
```
export struct SSwitchGroupProps {
  card-height : length,
  card-width : length,
  padding-type: PaddingType,
  shadow-type: ShadowType,
  border-type : BorderType,
  active : string,
  theme: Themes,
  switchs : [SOption],
  font-family : string,
  font-weight : int,
  font-size: length,
  font-italic : bool,
}
```
### STableProps
```
export struct STableProps {
  //theme
  columns : [SOption],
  column-width : [length],
  column-themes:[Themes],
  viewport-height:length,
  //tab
  font-weight : int,
  font-size: length,
  font-color : brush,
  font-italic : bool,
  font-family : string,
  //theme
  theme : Themes,
  padding-type: PaddingType,
  shadow-type: ShadowType,
  border-type : BorderType,
  alignment: TextHorizontalAlignment,
}
```
### STableColumnProps
```
export struct STableColumnProps {
  index : int,
  datas : [string],
  height : length,
  width : length,
  //tab
  font-weight : int,
  font-size: length,
  font-color : brush,
  font-italic : bool,
  font-family : string,
  theme : Themes,
  padding-type: PaddingType,
  shadow-type: ShadowType,
  border-type : BorderType,
  alignment: TextHorizontalAlignment,
}
```
### STagProps
```
export struct STagProps {
  text : string,
  font-size : length,
  font-weight : int,
  font-family : string,
  font-italic : bool,
  theme : Themes,
  padding-type : PaddingType,
  border-type : BorderType,
  shadow-type : ShadowType
}
```
### STextProps
```
struct STextProps {
  font-family : string,
  font-weight : int,
  font-size : length,
  color : brush,
  font-italic : bool,
  theme : Themes,
  wrap :TextWrap,
  overflow : TextOverflow,
  letter-spacing : length,
  horizontal-alignment : TextHorizontalAlignment,
  vertical-alignment : TextVerticalAlignment,
}
```
### STipProps
```
export struct STipProps {
  //font
  font-family : string,
  font-weight : int,
  font-size: length,
  font-color : brush,
  font-italic : bool,
  //theme
  theme : Themes,
  wrap : TextWrap,
  overflow : TextOverflow,
  letter-spacing : length,
  horizontal-alignment : TextHorizontalAlignment,
  vertical-alignment : TextVerticalAlignment,
  position : Position,
  is-show : bool,
  text : string,
  tip-width : length,
}
```
### STreeProps
```
export struct STreeProps {
  //font
  font-family : string,
  font-weight : int,
  font-size: length,
  font-italic : bool,
  //font
  item-font-family : string,
  item-font-weight : int,
  item-font-size: length,
  item-font-italic : bool,
  //theme
  theme : Themes,
  //hight-width
  height : length,
  width : length,
  padding-type: PaddingType,
  shadow-type: ShadowType,
  border-type : BorderType,
  tree-data : TreeData
}
```
## Build-In Function : `UseSurrealismFn`
- pure public function count-height(h:length,padding:length)->length : count component height
- pure public function count-width(w:length,padding:length)->length : count component weight
- pure public function get-padding(size:PaddingType)->PaddingProps : get padding by PaddingType
- pure public function get-shadow(shadow:ShadowType)->ShadowProps : get shadow by ShadowType
- pure public function get-shadow-x(shadow:ShadowType)->length : get shadow x by ShadowType
- pure public function get-shadow-y(shadow:ShadowType)->length : get shadow y by ShadowType
- pure public function get-shadow-blur(shadow:ShadowType)->length : get shadow blur by ShadowType
- pure public function get-border(border:BorderType)->BorderProps : get border by BorderType
- pure public function get-space(w:length) -> length : get spacing by component width
- pure public function deeper(theme:Themes,color:brush)->brush : get deeper theme color
  - pure public function light-deeper(color:brush)->brush : get deeper light theme color
  - pure public function primary-deeper(color:brush)->brush : get deeper primary theme color
  - pure public function success-deeper(color:brush)->brush : get deeper success theme color
  - pure public function info-deeper(color:brush)->brush : get deeper info theme color
  - pure public function warning-deeper(color:brush)->brush : get deeper warning theme color
  - pure public function error-deeper(color:brush)->brush : get deeper error theme color
  - pure public function dark-deeper(color:brush)->brush : get deeper dark theme color
- pure public function get-color(theme:Themes,level:ColorLevel)->brush : get color by theme and ColorLevel
  - pure public function get-color-light(level:ColorLevel)->brush : get light color by ColorLevel
  - pure public function get-color-dark(level:ColorLevel)->brush : get dark color by ColorLevel
  - pure public function get-color-primary(level:ColorLevel)->brush : get primary color by ColorLevel
  - pure public function get-color-info(level:ColorLevel)->brush : get info color by ColorLevel
  - pure public function get-color-warning(level:ColorLevel)->brush : get warning color by ColorLevel
  - pure public function get-color-success(level:ColorLevel)->brush : get success color by ColorLevel
  - pure public function get-color-error(level:ColorLevel)->brush : get error color by ColorLevel