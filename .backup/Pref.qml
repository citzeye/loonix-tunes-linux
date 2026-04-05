/* --- LOONIX-TUNES qml/ui/Pref.qml --- */

import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Qt.labs.platform
import "pref"

Item {
    id: settingsPage
    anchors.fill: parent

    // FIX: Pindahin visible ke root Item biar panel gak jadi "hantu"
    visible: root.settingsDialogVisible

    property int currentTabIndex: 0
    readonly property bool isCompact: settingsPage.width < 500

    // Overlay background - click to close
    Rectangle {
        anchors.fill: parent
        color: "#40000000"
        MouseArea {
            anchors.fill: parent
            onClicked: root.settingsDialogVisible = false
        }
    }

    // Main container - CENTER ALIGNED, BOXY & SHARP (Sesuai Foto Asli)
    Rectangle {
        id: popupContainer
        // Lebar dinamis: Minimal 450px, Maksimal 600px, atau 60% dari window
        width: Math.max(Math.min(parent.width * 0.6, 600), 450)
        height: parent.height * 0.8
        anchors.centerIn: parent
        anchors.leftMargin: 8
        anchors.rightMargin: 8
        anchors.topMargin: 8
        anchors.bottomMargin: 8

        // Balikin border tegas dan buang radius lengkung-lengkung
        color: theme.colormap.bgmain
        border.color: theme.colormap.playeraccent
        border.width: 0.5
        radius: 3

        // === MAIN CONTENT (Sidebar + Pages) ===
        // Langsung fill ke parent karena Header & Footer udah dibabat habis
        Rectangle {
            id: mainRow
            anchors.fill: parent
            color: "transparent"

            // --- LEFT SIDEBAR ---
            Rectangle {
                id: sidebar
                anchors.top: parent.top
                anchors.bottom: parent.bottom
                anchors.left: parent.left
                width: isCompact ? 60 : 130
                color: theme.colormap["bgoverlay"]
                border.width: 0

                Behavior on width { NumberAnimation { duration: 200 } }

                ColumnLayout {
                    anchors.fill: parent
                    anchors.leftMargin: isCompact ? 5 : 15
                    anchors.rightMargin: isCompact ? 5 : 15
                    anchors.topMargin: 15
                    anchors.bottomMargin: 15
                    spacing: 8

                    PrefTab {
                        text: isCompact ? "" : "Hardware"
                        icon: "󰍛"
                        isActive: currentTabIndex === 0
                        onClicked: currentTabIndex = 0
                    }
                    PrefTab {
                        text: isCompact ? "" : "Audio"
                        icon: "󰗅"
                        isActive: currentTabIndex === 1
                        onClicked: currentTabIndex = 1
                    }
                    PrefTab {
                        text: isCompact ? "" : "Library"
                        icon: ""
                        isActive: currentTabIndex === 2
                        onClicked: currentTabIndex = 2
                    }
                    PrefTab {
                        text: isCompact ? "" : "Appearance"
                        icon: "󰸌"
                        isActive: currentTabIndex === 3
                        onClicked: currentTabIndex = 3
                    }
                    PrefTab {
                        text: isCompact ? "" : "About"
                        icon: "󰋽"
                        isActive: currentTabIndex === 4
                        onClicked: currentTabIndex = 4
                    }
                    PrefTab {
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
                color: "transparent"

                // Garis pemisah vertikal antara sidebar dan konten
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

                    PrefHardware {}
                    PrefAudio {}
                    PrefLibrary {}
                    PrefAppearance {}
                    PrefAbout {}
                    PrefDonate {}
                }
            }
        }
    }
}