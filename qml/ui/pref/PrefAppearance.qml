import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtQuick.Dialogs
import "../contextmenu"

Item {
    id: prefAppearanceRoot
    property int refreshTicker: 0
    property string wallSyncError: ""
    property string currentWallpaper: ""
    property bool wallSyncEnabled: false
    property bool wallSyncSyncing: false
    property bool playlistContextMenuVisible: false

    function openThemeEditorWithTarget(targetIndex) {
        root.prefThemeEditorProfileTarget = targetIndex
        root.prefThemeEditorVisible = true
    }

    Connections {
        target: theme
        function onColormapChanged() {
            refreshTicker++
        }
        function onWallpaper_sync_status(success, message) {
            wallSyncSyncing = false
            if (!success) {
                wallSyncError = message
                wallSyncEnabled = false
            } else {
                wallSyncError = ""
                wallSyncEnabled = true
            }
        }
    }

    Flickable {
        id: appFlick
        anchors.fill: parent
        contentWidth: width
        contentHeight: appColumn.implicitHeight + 40
        clip: true
        interactive: true
        boundsBehavior: Flickable.StopAtBounds

        ScrollBar.vertical: ScrollBar {
            id: vBar
            width: 6
            policy: ScrollBar.AsNeeded
            background: Rectangle { color: "transparent" }
            contentItem: Rectangle {
                implicitWidth: 6
                implicitHeight: Math.max(30, appFlick.height * appFlick.visibleArea.heightRatio)
                radius: 3
                color: vBar.pressed ? theme.colormap.playeraccent : 
                       vBar.hovered ? theme.colormap.headerhover : 
                       theme.colormap.playeraccent
                opacity: vBar.active ? 1.0 : 0.5
                Behavior on color { ColorAnimation { duration: 150 } }
                Behavior on opacity { NumberAnimation { duration: 150 } }
            }
        }

        ColumnLayout {
            id: appColumn
            y: 10
            anchors.leftMargin: 10
            anchors.rightMargin: 10
            anchors.topMargin: 10
            anchors.bottomMargin: 10
            width: appFlick.width - 20
            spacing: 12

            // --- 1. DEFAULT THEMES ---
            ColumnLayout {
                Layout.alignment: Qt.AlignHCenter 
                spacing: 4

                Repeater {
                    model: ["Loonix", "Blue", "Green", "Monochrome", "Orange", "Pink", "Red", "Yellow"]

                    delegate: Rectangle {
                        Layout.preferredWidth: 200
                        Layout.preferredHeight: 32
                        Layout.alignment: Qt.AlignHCenter 
                        radius: 4
                        color: modelData === theme.current_theme ? theme.colormap["playeraccent"] : theme.colormap["bgoverlay"]
                        border.color: themeItemArea.containsMouse ? theme.colormap["playeraccent"] : theme.colormap["graysolid"]
                        border.width: 1

                        Behavior on color { ColorAnimation { duration: 150 } }
                        Behavior on border.color { ColorAnimation { duration: 150 } }

                        Text {
                            anchors.centerIn: parent
                            text: modelData
                            font.family: kodeMono.name
                            font.pixelSize: 12
                            color: modelData === theme.current_theme ? theme.colormap["bgmain"] : theme.colormap["playlisttext"]
                            font.bold: modelData === theme.current_theme
                        }

                        MouseArea {
                            id: themeItemArea
                            anchors.fill: parent
                            cursorShape: Qt.PointingHandCursor
                            hoverEnabled: true
                            onClicked: theme.set_theme(modelData)
                        }
                    }
                }
            }

            Item { Layout.preferredHeight: 32 }

            // --- 2. CUSTOM THEMES (PRESETS) ---
            ColumnLayout {
                Layout.alignment: Qt.AlignHCenter 
                spacing: 4

                Repeater {
                    id: customThemeRepeater
                    model: theme.get_custom_themes()

                    delegate: Rectangle {
                        property int presetIndex: index
                        property string presetName: modelData.name

                        Layout.preferredWidth: 200
                        Layout.preferredHeight: 32
                        Layout.alignment: Qt.AlignHCenter 
                        radius: 4
                        color: presetName === theme.current_theme ? theme.colormap["playeraccent"] : theme.colormap["bgoverlay"]
                        border.color: {
                             if (prefPage.appearanceMenuVisible && prefPage.appearanceMenuIndex === presetIndex) {
                                return theme.colormap["playeraccent"]
                            }
                            if (customItemArea.containsMouse) {
                                return theme.colormap["playeraccent"]
                            }
                            return theme.colormap["graysolid"]
                        }
                        border.width: (prefPage.appearanceMenuVisible && prefPage.appearanceMenuIndex === presetIndex) ? 2 : 1

                        Behavior on color { ColorAnimation { duration: 150 } }
                        Behavior on border.color { ColorAnimation { duration: 150 } }

                        Text {
                            anchors.centerIn: parent
                            text: presetName
                            font.family: kodeMono.name
                            font.pixelSize: 12
                            color: presetName === theme.current_theme ? theme.colormap["bgmain"] : theme.colormap["playlisttext"]
                            font.bold: presetName === theme.current_theme
                        }

                        MouseArea {
                            id: customItemArea
                            anchors.fill: parent
                            cursorShape: Qt.PointingHandCursor
                            hoverEnabled: true
                            acceptedButtons: Qt.LeftButton | Qt.RightButton
                            onClicked: (mouse) => {
                                if (mouse.button === Qt.RightButton) {
                                    var p = customItemArea.mapToItem(
                                        prefPage,
                                        0,
                                        customItemArea.height
                                    )
                                    prefPage.openAppearanceMenu(p.x, p.y, presetIndex)
                                } else if (mouse.button === Qt.LeftButton) {
                                    prefPage.closeAppearanceMenu()
                                    theme.set_theme(presetName)
                                }
                            }
                        }
                    }
                }
            }

            Item { Layout.preferredHeight: 32 }

            // --- 3. CREATE THEME BUTTON ---
            ColumnLayout {
                Layout.alignment: Qt.AlignHCenter 

                Rectangle {
                    Layout.preferredWidth: 200
                    Layout.preferredHeight: 32
                    radius: 4
                    color: createThemeArea.containsMouse ? theme.colormap.playeraccent : theme.colormap.bgoverlay
                    border.color: theme.colormap.playeraccent
                    Behavior on color { ColorAnimation { duration: 150 } }

                    Text {
                        anchors.centerIn: parent
                        text: 'CREATE THEME'
                        font.family: kodeMono.name
                        font.pixelSize: 12
                        font.bold: true
                        color: createThemeArea.containsMouse ? theme.colormap.bgmain : theme.colormap.playeraccent
                    }

                    MouseArea {
                        id: createThemeArea
                        anchors.fill: parent
                        cursorShape: Qt.PointingHandCursor
                        hoverEnabled: true
                        onClicked: {
                            root.prefThemeEditorProfileTarget = -1
                            root.prefThemeEditorVisible = true
                        }
                    }
                }
            }

            Item { Layout.preferredHeight: 32 }

            // --- GARIS PEMISAH TEBAL & JELAS ---
            Rectangle {
                Layout.fillWidth: true
                height: 4
                color: theme.colormap.bgmain
                radius: 2 
                Layout.topMargin: 20 
                Layout.bottomMargin: 20 
            }

            // --- 2. THEME ENGINE SECTION ---
            ColumnLayout {
                Layout.fillWidth: true
                spacing: 20

                Text {
                    Layout.alignment: Qt.AlignHCenter
                    text: "THEME ENGINE"
                    font.family: kodeMono.name; font.pixelSize: 16; font.bold: true
                    color: theme.colormap.playeraccent
                }

                // --- OPSI A: LOONIX MANUAL ---
                RowLayout {
                    Layout.fillWidth: true
                    spacing: 12
                    opacity: !theme.use_wallpaper_theme ? 1.0 : 0.5

                    // Toggle Switch Manual
                    Rectangle {
                        width: 34; height: 18; radius: 9
                        color: !theme.use_wallpaper_theme ? theme.colormap.playeraccent : theme.colormap.graysolid
                        Rectangle {
                            x: !theme.use_wallpaper_theme ? 18 : 2; y: 2; width: 14; height: 14; radius: 7
                            color: theme.colormap.bgmain
                            Behavior on x { NumberAnimation { duration: 150 } }
                        }
                        MouseArea {
                            anchors.fill: parent
                            cursorShape: Qt.PointingHandCursor
                            onClicked: theme.set_loonix_manual()
                        }
                    }

                    ColumnLayout {
                        spacing: 1
                        Text { text: "Manual Theme Mode"; font.family: kodeMono.name; font.pixelSize: 11; font.bold: true; color: theme.colormap.tabtext }
                        Text { text: "Use presets or custom editor colors."; font.family: kodeMono.name; font.pixelSize: 9; color: theme.colormap.playersubtext }
                    }
                }

                // --- OPSI B: WALLPAPER SYNC (MATUGEN) ---
                ColumnLayout {
                    Layout.fillWidth: true
                    spacing: 10

                    RowLayout {
                        Layout.fillWidth: true
                        spacing: 12

                        // Toggle Status (Hanya indikator)
                        Rectangle {
                            width: 34; height: 18; radius: 9
                            color: theme.use_wallpaper_theme ? theme.colormap.playeraccent : theme.colormap.graysolid
                            Rectangle {
                                x: theme.use_wallpaper_theme ? 18 : 2; y: 2; width: 14; height: 14; radius: 7
                                color: theme.colormap.bgmain
                                Behavior on x { NumberAnimation { duration: 150 } }
                            }
                        }

                        ColumnLayout {
                            Layout.fillWidth: true; spacing: 1
                            Text { text: "Wallpaper Sync Mode"; font.family: kodeMono.name; font.pixelSize: 11; font.bold: true; color: theme.colormap.tabtext }
                            Text { text: "Extract colors from your current wallpaper."; font.family: kodeMono.name; font.pixelSize: 9; color: theme.colormap.playersubtext }
                        }
                    }

                    // --- ACTION BUTTONS: [SYNC] [SET] ---
                    RowLayout {
                        Layout.fillWidth: true
                        Layout.leftMargin: 46
                        spacing: 10

                        // Button SYNC
                        Rectangle {
                            id: syncBtn
                            property bool isSyncing: false
                            width: 70; height: 26; radius: 4
                            color: isSyncing ? theme.colormap.bgmain : theme.colormap.bgoverlay
                            border.color: theme.colormap.playeraccent
                            border.width: 1

                            Text {
                                anchors.centerIn: parent
                                text: syncBtn.isSyncing ? "..." : "SYNC"
                                font.family: kodeMono.name; font.pixelSize: 10; font.bold: true
                                color: theme.colormap.playeraccent
                            }

                            MouseArea {
                                anchors.fill: parent
                                cursorShape: syncBtn.isSyncing ? Qt.WaitCursor : Qt.PointingHandCursor
                                onClicked: {
                                    syncBtn.isSyncing = true
                                    theme.sync_with_wallpaper()
                                }
                            }
                        }

                        // Button SET (Enabled only if sync successful)
                        Rectangle {
                            id: setBtn
                            width: 70; height: 26; radius: 4
                            // Nyala cuma kalau theme.is_sync_ready (dari Rust)
                            color: theme.is_sync_ready ? theme.colormap.playeraccent : theme.colormap.bgmain
                            opacity: theme.is_sync_ready ? 1.0 : 0.3
                            border.color: theme.is_sync_ready ? "transparent" : theme.colormap.graysolid

                            Text {
                                anchors.centerIn: parent
                                text: "SET"
                                font.family: kodeMono.name; font.pixelSize: 10; font.bold: true
                                color: theme.is_sync_ready ? theme.colormap.bgmain : theme.colormap.playersubtext
                            }

                            MouseArea {
                                anchors.fill: parent
                                enabled: wallSyncEnabled
                                cursorShape: wallSyncEnabled ? Qt.PointingHandCursor : Qt.ForbiddenCursor
                                onClicked: {
                                    if (wallSyncEnabled) theme.set_loonix_manual()
                                }
                            }
                        }
                    }
                }

                // --- Error Display ---
                Rectangle {
                    Layout.fillWidth: true
                    Layout.preferredHeight: errTxt.implicitHeight + 10
                    color: "#22ff5555"; radius: 4; visible: wallSyncError !== ""
                    Text {
                        id: errTxt; anchors.centerIn: parent; width: parent.width - 20
                        text: "⚠ " + wallSyncError; color: "#ff8888"
                        font.family: kodeMono.name; font.pixelSize: 10; wrapMode: Text.Wrap
                    }
                }

                // --- SYSTEM DIAGNOSTICS (Box Kecil) ---
                Rectangle {
                    Layout.fillWidth: true
                    height: diagCol.implicitHeight + 16
                    radius: 4; color: theme.colormap.bgoverlay
                    border.color: theme.colormap.graysolid

                    ColumnLayout {
                        id: diagCol; anchors.fill: parent; anchors.margins: 8; spacing: 4
                        Text { text: "SYSTEM DIAGNOSTICS"; font.family: kodeMono.name; font.pixelSize: 9; font.bold: true; color: theme.colormap.playeraccent }
                        
                        RowLayout {
                            spacing: 15
                            Text { text: "DE: " + theme.get_system_report().de; font.family: kodeMono.name; font.pixelSize: 10; color: theme.colormap.playersubtext }
                            Text { text: "Matugen: " + (theme.get_system_report().has_matugen === "true" ? "OK" : "MISSING"); font.family: kodeMono.name; font.pixelSize: 10; color: theme.colormap.playersubtext }
                            Text { text: "Wallpaper: " + (theme.get_system_report().has_wallpaper === "true" ? "DETECTED" : "NOT FOUND"); font.family: kodeMono.name; font.pixelSize: 10; color: theme.colormap.playersubtext }
                        }
                    }
                }
            }

            Item { Layout.preferredHeight: 40 }
        }
    }
}