# opfsRoot

OPFS (Origin Private File System) is a secure, sandboxed file system for web applications.

```js
const opfsRoot = await navigator.storage.getDirectory();

// Create a new file in the root
const fileHandle = await opfsRoot.getFileHandle("example.txt", {
  create: true,
});

// Write to the file
const writable = await fileHandle.createWritable();
await writable.write("Hello OPFS!");
await writable.close();
```

While Tauri applications can leverage the OPFS through the webview, it's important to note that Tauri also provides its own file system APIs via the @tauri-apps/plugin-fs plugin. This plugin offers more direct access to the native file system, which might be preferable for certain use cases.

Using Tauri's file system plugin can provide more flexibility and control, especially when dealing with files outside the sandboxed environment of the OPFS.

```js
import { readTextFile, BaseDirectory } from "@tauri-apps/plugin-fs";

const contents = await readTextFile("example.txt", {
  baseDir: BaseDirectory.Home,
});
```
