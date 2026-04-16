/* --- LOONIX-TUNES qml/ui/ThemeSlider.qml --- */
import QtQuick
import QtQuick.Controls

Slider {
    id: root

    background: Rectangle {
        x: root.leftPadding
        y: root.topPadding + root.availableHeight / 2 - height / 2
        width: root.availableWidth
        height: 3
        radius: 1.5
        color: theme.colormap["dspsliderbg"]

        Rectangle {
            width: root.visualPosition * parent.width
            height: parent.height
            color: theme.colormap.playeraccent
            radius: 1.5
        }
    }

    handle: Rectangle {
        x: root.leftPadding + root.visualPosition * (root.availableWidth - width)
        y: root.topPadding + root.availableHeight / 2 - height / 2
        width: 10
        height: 10
        radius: 5
        color: root.pressed ? theme.colormap.playerhover : theme.colormap.playeraccent
        border.color: theme.colormap.tabborder
        border.width: 1
    }
}
