/* --- LOONIX-TUNES qml/ui/Pref.qml --- */

import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "pref"

Item {
    id: prefPage
    anchors.fill: parent
    visible: root.prefDialogVisible
    enabled: root.prefDialogVisible

    // Block mouse events to background when popup is open
    Rectangle {
        anchors.fill: parent
        color: "transparent"
        
        MouseArea {
            anchors.fill: parent
            hoverEnabled: true 
            onWheel: (wheel) => { wheel.accepted = true } 
            onClicked: root.prefDialogVisible = false
        }
    }

    property int currentTabIndex: 0

    // === MAIN CONTAINER ===
    Rectangle {
        id: popupContainer
        width: Math.max(Math.min(parent.width * 0.6, 600), 450)
        height: parent.height * 0.8 - 10
        anchors.horizontalCenter: parent.horizontalCenter
        anchors.verticalCenter: parent.verticalCenter
        color: theme.colormap.bgmain
        border.color: "#333333"
        border.width: 0.5
        radius: 0

        // Tameng klik - mencegah click jatuh ke background
        MouseArea {
            anchors.fill: parent
        }

        ColumnLayout {
            anchors.fill: parent
            anchors.leftMargin: 1
            anchors.rightMargin: 1
            anchors.topMargin: 1
            anchors.bottomMargin: 1

            // === 1. TOP HEADER ===
            Rectangle {
                Layout.fillWidth: true
                Layout.preferredHeight: 20
                color: theme.colormap.bgmain

                Rectangle {
                    width: 8
                    anchors.top: parent.top
                    anchors.bottom: parent.bottom
                    color: theme.colormap.bgmain
                }

                Rectangle {
                    width: 8
                    anchors.right: parent.right
                    anchors.top: parent.top
                    anchors.bottom: parent.bottom
                    color: theme.colormap.bgmain
                }

                RowLayout {
                    anchors.fill: parent
                    anchors.leftMargin: 8
                    anchors.rightMargin: 8
                    Layout.alignment: Qt.AlignRight | Qt.AlignVCenter

                    Text {
                        text: "PREFERENCES"
                        color: theme.colormap.headertext
                        font.family: kodeMono.name
                        font.pixelSize: 12
                        font.capitalization: Font.AllUppercase
                        font.weight: Font.DemiBold
                        Layout.alignment: Qt.AlignLeft | Qt.AlignVCenter
                    }

                    Item { Layout.fillWidth: true }

                    Text {
                        id: closeButton
                        text: "X"
                        property bool isHovered: false
                        color: isHovered ? "#FF69B4" : theme.colormap.headertext
                        font.family: kodeMono.name
                        font.pixelSize: 12
                        Layout.alignment: Qt.AlignRight | Qt.AlignVCenter

                        MouseArea {
                            anchors.fill: parent
                            anchors.margins: -10
                            hoverEnabled: true
                            onEntered: closeButton.isHovered = true
                            onExited: closeButton.isHovered = false
                            onClicked: root.prefDialogVisible = false
                        }
                    }
                }
            }

            // === 2. MAIN CONTENT AREA ===
            RowLayout {
                Layout.fillWidth: true
                Layout.fillHeight: true

                Rectangle {
                    Layout.fillHeight: true
                    width: 5
                    color: theme.colormap.bgmain
                }

                // --- PURPLE BAR ---
                Rectangle {
                    Layout.fillHeight: true
                    width: 4
                    color: theme.colormap.playeraccent
                }

                // --- LEFT BOX (SIDEBAR) ---
                Rectangle {
                    Layout.preferredWidth: 100
                    Layout.fillHeight: true
                    color: theme.colormap["bgoverlay"]
                    radius: 0

                    Column {
                        anchors.fill: parent
                        anchors.margins: 10
                        spacing: 8

                        PrefTab {
                            text: "Hardware"
                            isActive: prefPage.currentTabIndex === 0
                            onClicked: prefPage.currentTabIndex = 0
                        }
                        PrefTab {
                            text: "Audio"
                            isActive: prefPage.currentTabIndex === 1
                            onClicked: prefPage.currentTabIndex = 1
                        }
                        PrefTab {
                            text: "Library"
                            isActive: prefPage.currentTabIndex === 2
                            onClicked: prefPage.currentTabIndex = 2
                        }
                        PrefTab {
                            text: "Appearance"
                            isActive: prefPage.currentTabIndex === 3
                            onClicked: prefPage.currentTabIndex = 3
                        }
                        PrefTab {
                            text: "About"
                            isActive: prefPage.currentTabIndex === 4
                            onClicked: prefPage.currentTabIndex = 4
                        }
                        PrefTab {
                            text: "Donate"
                            isActive: prefPage.currentTabIndex === 5
                            onClicked: prefPage.currentTabIndex = 5
                        }
                    }
                }

                Rectangle {
                    Layout.preferredWidth: 1
                    Layout.fillHeight: true
                    color: "transparent"
                }

                // --- RIGHT BOX (PAGES) ---
                Rectangle {
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    color: theme.colormap["bgoverlay"]
                    radius: 0
                }

                Rectangle {
                    Layout.fillHeight: true
                    width: 5
                    color: theme.colormap.bgmain
                }
            }

            Rectangle {
                Layout.fillWidth: true
                Layout.preferredHeight: 5
                color: "transparent"
            }
        }
    }
}