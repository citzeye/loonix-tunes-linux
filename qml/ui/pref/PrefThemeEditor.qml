/* --- loonix-tunes/qml/ui/pref/PrefThemeEditor.qml --- */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtQuick.Dialogs

Item {
    id: prefThemeEditorRoot
    anchors.fill: parent
    z: 20000
    visible: root.prefThemeEditorVisible

    property int refreshTicker: 0
    property int selectedProfileIndex: 0
    

    Connections {
        target: theme
        function onCustom_themes_changed() {
            refreshTicker++;
        }
    }

    MouseArea {
        anchors.fill: parent
        onClicked: {
            root.prefThemeEditorVisible = false;
        }
    }

    function scanCurrentEditorColors() {
        return {
            "bgmain": inBgMain.inputText,
            "bgoverlay": inBgOverlay.inputText,
            "graysolid": inGraySolid.inputText,
            "contextmenubg": inContextMenuBg.inputText,
            "overlay": inOverlay.inputText,
            "headerbg": inHeaderBg.inputText,
            "headericon": inHeaderIcon.inputText,
            "headertext": inHeaderText.inputText,
            "headerhover": inHeaderHover.inputText,
            "playertitle": inPlayerTitle.inputText,
            "playersubtext": inPlayerSubtext.inputText,
            "playeraccent": inPlayerAccent.inputText,
            "playerhover": inPlayerHover.inputText,
            "tabtext": inTabText.inputText,
            "tabborder": inTabBorder.inputText,
            "tabhover": inTabHover.inputText,
            "playlisttext": inPlaylistText.inputText,
            "playlistfolder": inPlaylistFolder.inputText,
            "playlistactive": inPlaylistActive.inputText,
            "playlisticon": inPlaylistIcon.inputText,
            "dspbg": inEqBg.inputText,
            "dspborder": inEqBorder.inputText,
            "dspeqtext": inEqText.inputText,
            "dspeqicon": inEqIcon.inputText,
            "dspeqhover": inEqHover.inputText,
            "dspeqpresetactive": inEqPresetActive.inputText,
            "dspeq10slider": inEq10Slider.inputText,
            "dspeq10handle": inEq10Handle.inputText,
            "dspeqfaderslider": inEqFaderSlider.inputText,
            "dspfxbg": inFxBg.inputText,
            "dspfxtext": inFxText.inputText,
            "dspfxicon": inFxIcon.inputText,
            "dspfxhover": inFxHover.inputText,
            "dspfxactive": inFxActive.inputText,
            "dspfxslider": inFxSlider.inputText,
            "dspfxsliderbg": inFxSliderBg.inputText,
            "dspfxhandle": inFxHandle.inputText,
        };
    }

    onVisibleChanged: {
        if (visible) {
            prefThemeEditorRoot.selectedProfileIndex = root.prefThemeEditorProfileTarget >= 0 ? root.prefThemeEditorProfileTarget : 0;

            if (root.prefThemeEditorProfileTarget >= 0) {
                var savedColors = theme.get_custom_theme_colors(root.prefThemeEditorProfileTarget);
                inBgMain.inputText = savedColors.bgmain;
                inBgOverlay.inputText = savedColors.bgoverlay;
                inGraySolid.inputText = savedColors.graysolid;
                inContextMenuBg.inputText = savedColors.contextmenubg;
                inOverlay.inputText = savedColors.overlay;
                inHeaderBg.inputText = savedColors.headerbg;
                inHeaderIcon.inputText = savedColors.headericon;
                inHeaderText.inputText = savedColors.headertext;
                inHeaderHover.inputText = savedColors.headerhover;
                inPlayerTitle.inputText = savedColors.playertitle;
                inPlayerSubtext.inputText = savedColors.playersubtext;
                inPlayerAccent.inputText = savedColors.playeraccent;
                inPlayerHover.inputText = savedColors.playerhover;
                inTabText.inputText = savedColors.tabtext;
                inTabBorder.inputText = savedColors.tabborder;
                inTabHover.inputText = savedColors.tabhover;
                inPlaylistText.inputText = savedColors.playlisttext;
                inPlaylistFolder.inputText = savedColors.playlistfolder;
                inPlaylistActive.inputText = savedColors.playlistactive;
                inPlaylistIcon.inputText = savedColors.playlisticon;
                inEqBg.inputText = savedColors.dspbg;
                inEqBorder.inputText = savedColors.dspborder;
                inEqText.inputText = savedColors.dspeqtext;
                inEqIcon.inputText = savedColors.dspeqicon;
                inEqHover.inputText = savedColors.dspeqhover;
                inEqPresetActive.inputText = savedColors.dspeqpresetactive;
                inEq10Slider.inputText = savedColors.dspeq10slider;
                inEq10Handle.inputText = savedColors.dspeq10handle;
                inEqFaderSlider.inputText = savedColors.dspeqfaderslider;
                inFxBg.inputText = savedColors.dspfxbg;
                inFxText.inputText = savedColors.dspfxtext;
                inFxIcon.inputText = savedColors.dspfxicon;
                inFxHover.inputText = savedColors.dspfxhover;
                inFxActive.inputText = savedColors.dspfxactive;
                inFxSlider.inputText = savedColors.dspfxslider;
                inFxSliderBg.inputText = savedColors.dspfxsliderbg;
                inFxHandle.inputText = savedColors.dsphandle;
            } else {
                inBgMain.inputText = theme.colormap.bgmain;
                inBgOverlay.inputText = theme.colormap.bgoverlay;
                inGraySolid.inputText = theme.colormap.graysolid;
                inContextMenuBg.inputText = theme.colormap.contextmenubg;
                inOverlay.inputText = theme.colormap.overlay;
                inHeaderBg.inputText = theme.colormap.headerbg;
                inHeaderIcon.inputText = theme.colormap.headericon;
                inHeaderText.inputText = theme.colormap.headertext;
                inHeaderHover.inputText = theme.colormap.headerhover;
                inPlayerTitle.inputText = theme.colormap.playertitle;
                inPlayerSubtext.inputText = theme.colormap.playersubtext;
                inPlayerAccent.inputText = theme.colormap.playeraccent;
                inPlayerHover.inputText = theme.colormap.playerhover;
                inTabText.inputText = theme.colormap.tabtext;
                inTabBorder.inputText = theme.colormap.tabborder;
                inTabHover.inputText = theme.colormap.tabhover;
                inPlaylistText.inputText = theme.colormap.playlisttext;
                inPlaylistFolder.inputText = theme.colormap.playlistfolder;
                inPlaylistActive.inputText = theme.colormap.playlistactive;
                inPlaylistIcon.inputText = theme.colormap.playlisticon;
                inEqBg.inputText = theme.colormap.dspeqbg;
                inEqBorder.inputText = theme.colormap.dspborder;
                inEqText.inputText = theme.colormap.dspeqtext;
                inEqIcon.inputText = theme.colormap.dspeqicon;
                inEqHover.inputText = theme.colormap.dspeqhover;
                inEqPresetActive.inputText = theme.colormap.dspeqpresetactive;
                inEq10Slider.inputText = theme.colormap.dspeq10slider;
                inEq10Handle.inputText = theme.colormap.dspeq10handle;
                inEqFaderSlider.inputText = theme.colormap.dspeqfaderslider;
                inFxBg.inputText = theme.colormap.dspfxbg;
                inFxText.inputText = theme.colormap.dspfxtext;
                inFxIcon.inputText = theme.colormap.dspfxicon;
                inFxHover.inputText = theme.colormap.dspfxhover;
                inFxActive.inputText = theme.colormap.dspfxactive;
                inFxSlider.inputText = theme.colormap.dspfxslider;
                inFxSliderBg.inputText = theme.colormap.dspfxsliderbg;
                inFxHandle.inputText = theme.colormap.dspfxhandle;
            }
        } else {
            root.prefThemeEditorProfileTarget = -1;
        }
    }

    Rectangle {
        width: 420
        height: 520
        anchors.centerIn: parent
        color: theme.colormap.bgmain
        border.color: theme.colormap.tabborder
        border.width: 1
        radius: 4

        MouseArea {
            anchors.fill: parent
            acceptedButtons: Qt.AllButtons
            propagateComposedEvents: true
            onClicked: mouse.accepted = false
        }

        ColumnLayout {
            anchors.fill: parent
            anchors.margins: 12
            spacing: 8

            // --- HEADER ---
            RowLayout {
                Layout.fillWidth: true
                Text {
                    text: "THEME EDITOR"
                    color: theme.colormap.playeraccent
                    font.family: kodeMono.name
                    font.pixelSize: 14
                    font.bold: true
                    Layout.fillWidth: true
                }
                Text {
                    id: closeBtn
                    text: "󰅖"
                    font.family: symbols.name
                    font.pixelSize: 16
                    color: closeMA.containsMouse ? theme.colormap.playerhover : theme.colormap.tabtext
                    MouseArea {
                        id: closeMA
                        anchors.fill: parent
                        anchors.margins: -10
                        hoverEnabled: true
                        onClicked: root.prefThemeEditorVisible = false
                    }
                }
            }

            // --- THEME NAME INPUT ---
            RowLayout {
                Layout.fillWidth: true
                spacing: 8
                Text {
                    text: "NAME"
                    color: theme.colormap.tabtext
                    font.family: kodeMono.name
                    font.pixelSize: 11
                }
                TextField {
                    id: themeNameInput
                    Layout.fillWidth: true
                    Layout.preferredHeight: 28
                    text: root.prefThemeEditorProfileTarget >= 0 ? theme.get_custom_theme_name(root.prefThemeEditorProfileTarget) : (prefThemeEditorRoot.selectedProfileIndex >= 0 ? theme.get_custom_theme_name(prefThemeEditorRoot.selectedProfileIndex) : "New Theme")
                    color: theme.colormap.playeraccent
                    font.family: kodeMono.name
                    font.pixelSize: 12
                    background: Rectangle {
                        color: theme.colormap.bgoverlay
                        radius: 4
                        border.color: theme.colormap.tabborder
                        border.width: 1
                    }
                }
            }

            Rectangle {
                Layout.fillWidth: true
                Layout.preferredHeight: 1
                color: theme.colormap.tabborder
                Layout.topMargin: 4
                Layout.bottomMargin: 4
            }

            // --- SCROLLABLE AREA ---
            ScrollView {
                Layout.fillWidth: true
                Layout.fillHeight: true
                clip: true
                ScrollBar.horizontal.policy: ScrollBar.AlwaysOff
                ScrollBar.vertical: ScrollBar {
                    width: 6
                    policy: ScrollBar.AsNeeded
                    contentItem: Rectangle {
                        radius: 3
                        color: theme.colormap.tabborder
                        opacity: 0.5
                    }
                }

                ColumnLayout {
                    width: parent.width - 58
                    spacing: 6

                    // --- RADIO BUTTONS ---
                    ColumnLayout {
                        Layout.fillWidth: true
                        Layout.bottomMargin: 8
                        spacing: 2
                        Text {
                            text: "Choose which theme to replace:"
                            color: theme.colormap.tabtext
                            font.family: kodeMono.name
                            font.pixelSize: 10
                        }
                        Repeater {
                            model: theme.get_custom_theme_count()
                            delegate: RadioButton {
                                text: theme.get_custom_theme_name(index)
                                checked: prefThemeEditorRoot.selectedProfileIndex === index
                                onClicked: {
                                    prefThemeEditorRoot.selectedProfileIndex = index;
                                    root.prefThemeEditorProfileTarget = index;
                                }
                                contentItem: Text {
                                    text: parent.text
                                    color: theme.colormap.tabtext
                                    font.family: kodeMono.name
                                    font.pixelSize: 11
                                    leftPadding: 24
                                    verticalAlignment: Text.AlignVCenter
                                }
                            }
                        }
                    }

                    Rectangle {
                        Layout.fillWidth: true
                        Layout.preferredHeight: 1
                        color: theme.colormap.tabborder
                        Layout.bottomMargin: 8
                    }

                    SectionHeader {
                        sectionTitle: "BACKGROUNDS"
                    }
                    ColorInputRow {
                        id: inBgMain
                        labelText: "bgmain"
                        hexValue: theme.colormap.bgmain
                    }
                    ColorInputRow {
                        id: inBgOverlay
                        labelText: "bgoverlay"
                        hexValue: theme.colormap.bgoverlay
                    }
                    ColorInputRow {
                        id: inGraySolid
                        labelText: "graysolid"
                        hexValue: theme.colormap.graysolid
                    }
                    ColorInputRow {
                        id: inContextMenuBg
                        labelText: "contextmenubg"
                        hexValue: theme.colormap.contextmenubg
                    }
                    ColorInputRow {
                        id: inOverlay
                        labelText: "overlay"
                        hexValue: theme.colormap.overlay
                    }

                    SectionHeader {
                        sectionTitle: "HEADER"
                    }
                    ColorInputRow {
                        id: inHeaderBg
                        labelText: "headerbg"
                        hexValue: theme.colormap.headerbg
                    }
                    ColorInputRow {
                        id: inHeaderIcon
                        labelText: "headericon"
                        hexValue: theme.colormap.headericon
                    }
                    ColorInputRow {
                        id: inHeaderText
                        labelText: "headertext"
                        hexValue: theme.colormap.headertext
                    }
                    ColorInputRow {
                        id: inHeaderHover
                        labelText: "headerhover"
                        hexValue: theme.colormap.headerhover
                    }

                    SectionHeader {
                        sectionTitle: "PLAYER"
                    }
                    ColorInputRow {
                        id: inPlayerTitle
                        labelText: "playertitle"
                        hexValue: theme.colormap.playertitle
                    }
                    ColorInputRow {
                        id: inPlayerSubtext
                        labelText: "playersubtext"
                        hexValue: theme.colormap.playersubtext
                    }
                    ColorInputRow {
                        id: inPlayerAccent
                        labelText: "playeraccent"
                        hexValue: theme.colormap.playeraccent
                    }
                    ColorInputRow {
                        id: inPlayerHover
                        labelText: "playerhover"
                        hexValue: theme.colormap.playerhover
                    }

                    SectionHeader {
                        sectionTitle: "TABS"
                    }
                    ColorInputRow {
                        id: inTabText
                        labelText: "tabtext"
                        hexValue: theme.colormap.tabtext
                    }
                    ColorInputRow {
                        id: inTabBorder
                        labelText: "tabborder"
                        hexValue: theme.colormap.tabborder
                    }
                    ColorInputRow {
                        id: inTabHover
                        labelText: "tabhover"
                        hexValue: theme.colormap.tabhover
                    }

                    SectionHeader {
                        sectionTitle: "PLAYLIST"
                    }
                    ColorInputRow {
                        id: inPlaylistText
                        labelText: "playlisttext"
                        hexValue: theme.colormap.playlisttext
                    }
                    ColorInputRow {
                        id: inPlaylistFolder
                        labelText: "playlistfolder"
                        hexValue: theme.colormap.playlistfolder
                    }
                    ColorInputRow {
                        id: inPlaylistActive
                        labelText: "playlistactive"
                        hexValue: theme.colormap.playlistactive
                    }
                    ColorInputRow {
                        id: inPlaylistIcon
                        labelText: "playlisticon"
                        hexValue: theme.colormap.playlisticon
                    }

                    SectionHeader {
                        sectionTitle: "EQ"
                    }
                    ColorInputRow {
                        id: inEqBg
                        labelText: "dspeqbg"
                        hexValue: theme.colormap.dspeqbg
                    }
                    ColorInputRow {
                        id: inEqText
                        labelText: "dspeqtext"
                        hexValue: theme.colormap.dspeqtext
                    }
                    ColorInputRow {
                        id: inEqIcon
                        labelText: "dspeqicon"
                        hexValue: theme.colormap.dspeqicon
                    }
                    ColorInputRow {
                        id: inEqHover
                        labelText: "dspeqhover"
                        hexValue: theme.colormap.dspeqhover
                    }
                    ColorInputRow {
                        id: inEqPresetActive
                        labelText: "dspeqpresetactive"
                        hexValue: theme.colormap.dspeqpresetactive
                    }
                    ColorInputRow {
                        id: inEq10Slider
                        labelText: "dspeq10slider"
                        hexValue: theme.colormap.dspeq10slider
                    }
                    ColorInputRow {
                        id: inEq10Handle
                        labelText: "dspeq10handle"
                        hexValue: theme.colormap.dspeq10handle
                    }
                    ColorInputRow {
                        id: inEqFaderSlider
                        labelText: "dspeqfaderslider"
                        hexValue: theme.colormap.dspeqfaderslider
                    }
                    SectionHeader {
                        sectionTitle: "FX"
                    }
                    ColorInputRow {
                        id: inFxBg
                        labelText: "dspfxbg"
                        hexValue: theme.colormap.dspfxbg
                    }
                    ColorInputRow {
                        id: inFxText
                        labelText: "dspfxtext"
                        hexValue: theme.colormap.dspfxtext
                    }
                    ColorInputRow {
                        id: inFxIcon
                        labelText: "dspfxicon"
                        hexValue: theme.colormap.dspfxicon
                    }
                    ColorInputRow {
                        id: inFxHover
                        labelText: "dspfxhover"
                        hexValue: theme.colormap.dspfxhover
                    }
                    ColorInputRow {
                        id: inFxActive
                        labelText: "dspfxactive"
                        hexValue: theme.colormap.dspfxactive
                    }
                    ColorInputRow {
                        id: inFxSlider
                        labelText: "dspfxslider"
                        hexValue: theme.colormap.dspfxslider
                    }
                    ColorInputRow {
                        id: inFxSliderBg
                        labelText: "dspfxsliderbg"
                        hexValue: theme.colormap.dspfxsliderbg
                    }
                    ColorInputRow {
                        id: inFxHandle
                        labelText: "dspfxhandle"
                        hexValue: theme.colormap.dspfxhandle
                    }
                }
            }

            // --- BOTTOM BUTTONS ---
            RowLayout {
                Layout.fillWidth: true
                Layout.topMargin: 4
                spacing: 8
                Rectangle {
                    Layout.fillWidth: true
                    Layout.preferredHeight: 28
                    radius: 4
                    color: theme.colormap.bgoverlay
                    border.color: theme.colormap.tabborder
                    border.width: 1
                    Text {
                        anchors.centerIn: parent
                        text: "CANCEL"
                        color: theme.colormap.tabtext
                        font.family: kodeMono.name
                        font.pixelSize: 11
                        font.bold: true
                    }
                    MouseArea {
                        anchors.fill: parent
                        onClicked: root.prefThemeEditorVisible = false
                    }
                }
                Rectangle {
                    Layout.fillWidth: true
                    Layout.preferredHeight: 28
                    radius: 4
                    color: theme.colormap.bgoverlay
                    border.color: theme.colormap.tabborder
                    border.width: 1
                    Text {
                        anchors.centerIn: parent
                        text: "RESET"
                        color: theme.colormap.tabtext
                        font.family: kodeMono.name
                        font.pixelSize: 11
                        font.bold: true
                    }
                    MouseArea {
                        anchors.fill: parent
                        onClicked: {
                            var defaults = theme.get_default_colors();
                            inBgMain.inputText = defaults.bgmain; // ... rest of reset logic
                        }
                    }
                }
                Rectangle {
                    Layout.fillWidth: true
                    Layout.preferredHeight: 28
                    radius: 4
                    color: theme.colormap.playeraccent
                    border.width: 1
                    Text {
                        anchors.centerIn: parent
                        text: "SAVE"
                        color: theme.colormap.bgmain
                        font.family: kodeMono.name
                        font.pixelSize: 11
                        font.bold: true
                    }
                    MouseArea {
                        anchors.fill: parent
                        onClicked: {
                            var newName = themeNameInput.text;
                            theme.set_custom_theme_name(prefThemeEditorRoot.selectedProfileIndex, newName);
                            theme.set_custom_theme_colors(prefThemeEditorRoot.selectedProfileIndex, scanCurrentEditorColors());
                            if (root.prefThemeEditorProfileTarget === -1 || (theme.get_custom_theme_name(prefThemeEditorRoot.selectedProfileIndex) === theme.current_theme)) {
                                theme.set_theme(newName);
                            }
                            root.prefThemeEditorVisible = false;
                        }
                    }
                }
            }
        }
    }

    component ColorInputRow: RowLayout {
        property string labelText: "Color"
        property string hexValue: "#000000"
        property alias inputText: hexField.text
        ColorDialog {
            id: colorPicker
            title: "Select " + labelText
            selectedColor: hexField.text
            onAccepted: hexField.text = colorPicker.selectedColor.toString()
        }
        Label {
            text: labelText
            color: theme.colormap.tabtext
            font.family: kodeMono.name
            font.pixelSize: 10
            Layout.preferredWidth: 100
        }
        Rectangle {
            width: 20
            height: 20
            radius: 3
            color: hexField.text
            border.color: theme.colormap.tabborder
            border.width: 1
            MouseArea {
                anchors.fill: parent
                onClicked: colorPicker.open()
            }
        }
        TextField {
            id: hexField
            text: hexValue
            Layout.preferredWidth: 80
            color: theme.colormap.playeraccent
            font.family: kodeMono.name
            font.pixelSize: 11
            background: Rectangle {
                color: theme.colormap.bgoverlay
                radius: 3
                border.color: theme.colormap.tabborder
                border.width: 1
            }
        }
    }

    component SectionHeader: Text {
        property string sectionTitle: ""
        text: sectionTitle
        color: theme.colormap.playeraccent
        font.family: kodeMono.name
        font.pixelSize: 12
        font.bold: true
        Layout.fillWidth: true
        Layout.topMargin: 8
        Layout.bottomMargin: 4
    }
}
