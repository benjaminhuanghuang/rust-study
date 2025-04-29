# FS permission

Add FS Permissions to tauri.conf.json

```json
{
  "plugins": {
    "fs": {
      "permissions": {
        "read": ["$APP/config"], // Allow reading from your config folder
        "readDir": ["$APP/config"] // Allow reading directories
      }
    }
  }
}
```

## Special Path Variables

$APP: Points to the app base directory.

$RESOURCE: The resource directory bundled with the app.

$HOME: The user's home directory.

$CONFIG: Tauri’s config directory.

$DOCUMENT, $DOWNLOAD, $DATA, etc.
