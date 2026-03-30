/* --- LOONIX-TUNES qml/ui/Preferences.qml --- */

import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Qt.labs.platform
import "preferences"

Item {
    id: settingsPage
    anchors.fill: parent

    property int currentTabIndex: 0
    readonly property bool isCompact: settingsPage.width < 500

    // === ROW 1: HEADER ===
    Item {
        id: headerRow
        anchors.top: parent.top
        anchors.left: parent.left
        anchors.right: parent.right
        height: 35

        SettingHeader {
            anchors.fill: parent
            title: "PREFERENCES"
        }
    }

    // === ROW 2: MAIN (Sidebar + Pages) ===
    Rectangle {
        id: mainRow
        anchors.top: headerRow.bottom
        anchors.bottom: footerRow.top
        anchors.left: parent.left
        anchors.right: parent.right
        color: "transparent"
        border.color: theme.colormap["graysolid"]
        border.width: 1
        radius: 2
        clip: true

        // --- LEFT SIDEBAR ---
        Rectangle {
            id: sidebar
            anchors.top: parent.top
            anchors.bottom: parent.bottom
            anchors.left: parent.left
            anchors.topMargin: parent.border.width
            anchors.bottomMargin: parent.border.width
            anchors.leftMargin: parent.border.width
            width: isCompact ? 60 : 180
            color: theme.colormap["bgoverlay"]
            radius: parent.radius

            Behavior on width { NumberAnimation { duration: 200 } }

            ColumnLayout {
                anchors.fill: parent
                anchors.leftMargin: isCompact ? 5 : 15
                anchors.rightMargin: isCompact ? 5 : 15
                anchors.topMargin: 15
                anchors.bottomMargin: 15
                spacing: 8

                SettingTab {
                    text: isCompact ? "" : "Hardware"
                    icon: "󰍛"
                    isActive: currentTabIndex === 0
                    onClicked: currentTabIndex = 0
                }
                SettingTab {
                    text: isCompact ? "" : "Audio"
                    icon: "󰗅"
                    isActive: currentTabIndex === 1
                    onClicked: currentTabIndex = 1
                }
                SettingTab {
                    text: isCompact ? "" : "Library"
                    icon: ""
                    isActive: currentTabIndex === 2
                    onClicked: currentTabIndex = 2
                }
                SettingTab {
                    text: isCompact ? "" : "Appearance"
                    icon: "󰸌"
                    isActive: currentTabIndex === 3
                    onClicked: currentTabIndex = 3
                }
                SettingTab {
                    text: isCompact ? "" : "About"
                    icon: "󰋽"
                    isActive: currentTabIndex === 4
                    onClicked: currentTabIndex = 4
                }
                SettingTab {
                    text: isCompact ? "" : "Donate"
                    icon: ""
                    isActive: currentTabIndex === 5
                    onClicked: currentTabIndex = 5
                }

                Item { Layout.fillHeight: true }
            }
        }

        // --- RIGHT CONTENT AREA ---
        Rectangle {
            anchors.top: parent.top
            anchors.bottom: parent.bottom
            anchors.left: sidebar.right
            anchors.right: parent.right
            anchors.topMargin: parent.border.width
            anchors.bottomMargin: parent.border.width
            anchors.rightMargin: parent.border.width
            color: theme.colormap["bgmain"]
            radius: parent.radius

            Rectangle {
                anchors.left: parent.left
                anchors.top: parent.top
                anchors.bottom: parent.bottom
                width: 1
                color: theme.colormap["graysolid"]
                opacity: 0.15
            }

            StackLayout {
                anchors.fill: parent
                anchors.margins: 20
                currentIndex: settingsPage.currentTabIndex

                Hardware {}
                Audio {}
                Library {}
                Appearance {}
                About {}
                Donate {}
            }
        }
    }

    // === ROW 3: FOOTER ===
    Item {
        id: footerRow
        anchors.bottom: parent.bottom
        anchors.left: parent.left
        anchors.right: parent.right
        height: 35

        SettingFooter {
            anchors.fill: parent
            isCompact: settingsPage.isCompact
            onCloseClicked: root.settingsDialogVisible = false
        }
    }
}
