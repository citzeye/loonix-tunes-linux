/* --- LOONIX-TUNES qml/ui/Pref.qml --- */

import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Qt.labs.platform
import "pref"

Item {
    id: prefPage
    anchors.fill: parent
    visible: root.prefDialogVisible
    enabled: root.prefDialogVisible

    // ==========================================
    // GLOBAL PROPERTIES FOR APPEARANCE MENU
    // ==========================================
    property bool appearanceMenuVisible: false
    property real appearanceMenuX: 0
    property real appearanceMenuY: 0
    property int appearanceMenuIndex: -1

    function openAppearanceMenu(x, y, index) {
        appearanceMenuX = x;
        appearanceMenuY = y;
        appearanceMenuIndex = index;
        appearanceMenuVisible = true;
    }

    function closeAppearanceMenu() {
        appearanceMenuVisible = false;
    }

    function openRenameDialog() {
        appearanceMenuVisible = false;
        root.customRenameDialogIndex = appearanceMenuIndex;
        root.customRenameDialogVisible = true;
    }

    // ==========================================
    // 1. BACKGROUND BLOCKER (Tembok Luar)
    // ==========================================
    Rectangle {
        anchors.fill: parent
        color: "transparent"

        MouseArea {
            anchors.fill: parent
            acceptedButtons: Qt.AllButtons
            hoverEnabled: true

            onWheel: wheel => {
                wheel.accepted = true;
            }

            onClicked: {
                root.prefDialogVisible = false;
            }
        }
    }

    property int currentTabIndex: 0

    // === MAIN CONTAINER ===
    Rectangle {
        id: popupContainer
        width: parent.width * 0.9
        height: parent.height * 0.8 - 10
        anchors.horizontalCenter: parent.horizontalCenter
        anchors.verticalCenter: parent.verticalCenter
        color: theme.colormap.bgmain
        border.color: theme.colormap["graysolid"]
        border.width: 0.5
        radius: 0

        // ==========================================
        // 2. TAMENG POPUP (Tembok Dalam)
        // ==========================================
        MouseArea {
            anchors.fill: parent
            acceptedButtons: Qt.AllButtons
            hoverEnabled: true
            cursorShape: Qt.ArrowCursor
            onWheel: wheel => {
                wheel.accepted = true;
            }
            // Sengaja GAK ADA onClicked, supaya dia cuma "nelen" klik tanpa melakukan apa-apa
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

                    Text {
                        text: "PREFERENCES"
                        color: theme.colormap.headertext
                        font.family: kodeMono.name
                        font.pixelSize: 12
                        font.capitalization: Font.AllUppercase
                        font.weight: Font.DemiBold
                        Layout.alignment: Qt.AlignLeft | Qt.AlignVCenter
                    }

                    Item {
                        Layout.fillWidth: true
                    }

                    Text {
                        id: closeButton
                        text: "󰅖"
                        property bool isHovered: false
                        color: isHovered ? theme.colormap["headerhover"] : theme.colormap.headertext
                        font.family: kodeMono.name
                        font.pixelSize: 18
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
                            text: "About"
                            icon: "󰋽"
                            isActive: prefPage.currentTabIndex === 0
                            onClicked: prefPage.currentTabIndex = 0
                        }
                        PrefTab {
                            text: "Appearance"
                            icon: "󰸌"
                            isActive: prefPage.currentTabIndex === 1
                            onClicked: prefPage.currentTabIndex = 1
                        }
                        PrefTab {
                            text: "Donate"
                            icon: "󱉛"
                            isActive: prefPage.currentTabIndex === 2
                            onClicked: prefPage.currentTabIndex = 2
                        }
                        PrefTab {
                            text: "Report Bug"
                            icon: "󰈮"
                            isActive: prefPage.currentTabIndex === 3
                            onClicked: prefPage.currentTabIndex = 3
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

                    StackLayout {
                        anchors.fill: parent
                        anchors.leftMargin: 10
                        anchors.topMargin: 10
                        anchors.bottomMargin: 10
                        anchors.rightMargin: 10

                        currentIndex: prefPage.currentTabIndex

                        Item {
                            Layout.fillWidth: true
                            Layout.fillHeight: true
                            PrefAbout {
                                anchors.fill: parent
                            }
                        }
                        Item {
                            Layout.fillWidth: true
                            Layout.fillHeight: true
                            PrefAppearance {
                                anchors.fill: parent
                            }
                        }
                        Item {
                            Layout.fillWidth: true
                            Layout.fillHeight: true
                            PrefDonate {
                                anchors.fill: parent
                            }
                        }
                        Item {
                            Layout.fillWidth: true
                            Layout.fillHeight: true
                            PrefReportBug {
                                anchors.fill: parent
                            }
                        }
                    }
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

        // ==========================================
        // GLOBAL FLOATING MENU (Same level as popupContainer)
        // ==========================================
        Item {
            id: appearanceMenuLayer
            anchors.fill: parent
            visible: prefPage.appearanceMenuVisible
            z: 9999

            MouseArea {
                anchors.fill: parent
                onClicked: prefPage.appearanceMenuVisible = false
            }

            AppearanceContextMenu {
                id: appearanceMenu
                x: prefPage.appearanceMenuX
                y: prefPage.appearanceMenuY
                presetIndex: prefPage.appearanceMenuIndex
            }
        }
    }
}
