# BaseDirectory

| BaseDirectory            | Description                                          |
| ------------------------ | ---------------------------------------------------- |
| BaseDirectory.App        | App-specific data directory (recommended for config) |
| BaseDirectory.Resource   | Folder where app static files are bundled            |
| BaseDirectory.Home       | User's home directory                                |
| BaseDirectory.Config     | OS config folder (AppData, .config, etc.)            |
| BaseDirectory.Executable | directory containing the app binary                  |

In Tauri, BaseDirectory.Home is an enumerated constant that represents the user's home directory on their operating system

On Windows → C:\Users\YourUsername\

On macOS/Linux → /Users/YourUsername/ or /home/YourUsername/

```js
import { readTextFile, BaseDirectory } from "@tauri-apps/plugin-fs";

const contents = await readTextFile("myfile.txt", {
  baseDir: BaseDirectory.Home,
});

// app's base directory
const configRaw = await readTextFile("config/config.json", {
  baseDir: BaseDirectory.App,
});
const config = JSON.parse(configRaw);
console.log(config);
```
