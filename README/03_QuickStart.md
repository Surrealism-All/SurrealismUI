## Config SurrealismUI as Library (optional)

1. Open VsCode and choose Settings , then search `Slint:Library Paths`  
2. Choose edit in settings.json
3. Find `slint.libraryPaths` and add `"SurrealismUI":"parent_file_path\\surrealism-ui\\index.slint"`❗

```json
  "slint.libraryPaths": {
    "SurrealismUI":"E:\\test_try\\test-surrealism\\ui\\modules\\surrealism-ui\\index.slint"
  },
```
![image-20231225233105029](https://github.com/Surrealism-All/SurrealismUI/assets/92167095/93f00002-e303-4798-a152-d372008e8587)

## Import and Use

```slint
import { STag,SCard,SInput,SText,SButton  } from "./modules/surrealism-ui/index.slint";

export component App inherits Window {
  height: 600px;
  width: 800px;
  title: "Surrealism";
  Rectangle {
    VerticalLayout {
      padding: 20px;
      spacing: 60px;
      SText {
        height: 3rem;
        text: "SurrealismUI";
      }
      SButton {
        text: "Try SurrealismUI";
      }
      SCard {
        STag{
          theme: Warning;
          text:"test tag";
        }
      }
      Rectangle {
        SInput{
          theme: Primary;
          placeholder :"please enter your username";
          card-width:300px;
          clearable: true;
          text:"SurrealismUI - input";
          accepted(res)=>{
            debug("content in input:" + res);
          }
          changed(change-res)=>{
            debug(change-res);
          }
        }
      }
    }
  }
}
```
![image-20231226000051318](https://github.com/Surrealism-All/SurrealismUI/assets/92167095/4fe5ba59-b8a8-4239-840b-d272523ffd48)
