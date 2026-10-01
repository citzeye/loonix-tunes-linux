# Tab UI Components

## 1. TabMusic.qml - Default Music Tab

- **Purpose**: Fixed tab for ~/Music folder (cannot be deleted/renamed)
- **Width**: Fixed at 60px
- **Text**: Always "MUSIC"
- **Left-click**: Calls `musicModel.scan_music()` (loads default Music folder)
- **Right-click**: Does nothing (prevents deletion)
- **Active state**: When `current_folder_qml` is empty or "MUSIC"

## 2. TabCustom.qml - Custom Folder Template

- **Purpose**: Reusable template for user-added folders
- **Width**: Dynamic (based on folder name width, max 100px)
- **Text**: Shows custom folder name
- **Left-click**: Switches to that folder via `musicModel.switch_to_folder()`
- **Right-click**: Shows context menu (change folder/lock/remove/close)
- **Active state**: When current folder matches this tab's name

## 3. Tab.qml - Main Tab Bar Container

- **Structure**:
  1. `TabMusic` component (leftmost, static)
  2. `Row` with `Repeater` that creates `TabCustom` for each custom folder
  3. Add button (`+`) on the right

## How It Works Together

```
[ QUEUE ] [ FAV ] [ MUSIC ] [ rock ] [ Jazz ] [ classical ] [ + ]
    ↑        ↑        ↑        ↑        ↑          ↑
  Queue  Favorite  TabMusic TabCustom TabCustom TabCustom  Add Button
(static) (static)  (static)  (dynamic) (dynamic) (dynamic)
```

- **TabMusic**: Always visible, cannot be removed
- **TabCustom**: Created dynamically based on `musicModel.custom_folder_count`
- **Add button**: Opens folder picker to add new `TabCustom`

## Architecture

The architecture separates concerns:

- `TabMusic` = default folder behavior
- `TabCustom` = reusable custom folder template
- `Tab.qml` = container that combines both

This makes it easier to maintain and modify tab behavior without touching the main Tab.qml file.

## Resource Registration

Both `TabMusic.qml` and `TabCustom.qml` must be registered in two places:

### 1. `src/main.rs` - qrc! macro (for Rust binary)

```rust
qmetaobject::qrc!(init_resources_v4,
    "/" {
        "qml/Ui.qml",
        "qml/ui/Tab.qml",
        "qml/ui/TabMusic.qml",
        "qml/ui/TabCustom.qml",
        // ... other files
    }
);
```

### 2. `qml/ui/qmldir` - for QML module

```
module ui
Tab 1.0 Tab.qml
TabMusic 1.0 TabMusic.qml
TabCustom 1.0 TabCustom.qml
// ... other components
```

## File Structure

```
qml/
├── Ui.qml
└── ui/
    ├── Tab.qml          # Main tab bar container
    ├── TabMusic.qml     # Default MUSIC tab component
    ├── TabCustom.qml    # Custom folder tab template
    ├── qmldir           # QML module definition
    └── ...              # Other UI components
```
