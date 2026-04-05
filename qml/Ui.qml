/* --- LOONIX-TUNES qml/Ui.qml --- */

import QtQuick
import QtQuick.Window
import QtQuick.Controls 
import QtQuick.Layouts
import Qt.labs.platform
import Loonix 1.0
import 'ui'

Window {
  id: root
  visible: true
  width: Screen.desktopAvailableWidth * 0.25
  height: 700
  x: (Screen.desktopAvailableWidth - width) / 2
  y: (Screen.desktopAvailableHeight - height) / 2
  color: 'transparent'
  title: 'Loonix-tunes'

  onClosing: {
    musicModel.save_state()
    Qt.quit()
  }

  // --- OVERLAY BLOKIR LAYAR ---
  readonly property real criticalWidth: 500

  Rectangle {
    id: screenWarning
    width: root.width
    height: root.height
    z: 9999
    color: "#E6000000"
    visible: root.width < 500

    MouseArea {
      anchors.fill: parent
      enabled: parent.visible
    }

    ColumnLayout {
      anchors.centerIn: parent
      spacing: 15

      Text {
        text: "\u26a0\ufe0f"
        font.family: emoji.name
        font.pixelSize: 40
        Layout.alignment: Qt.AlignHCenter
      }

      Text {
        text: "PLEASE MAKE YOUR WINDOW MINIMUM 500PX"
        color: "white"
        font.family: kodeMono.name
        font.pixelSize: 14
        font.bold: true
        horizontalAlignment: Text.AlignHCenter
        Layout.preferredWidth: parent.width * 0.8
        wrapMode: Text.WordWrap
      }

      Text {
        text: "Current size: " + Math.round(root.width) + "px"
        color: theme.colormap["playeraccent"]
        font.family: kodeMono.name
        font.pixelSize: 11
        Layout.alignment: Qt.AlignHCenter
      }
    }
  }

  // Create popup menu instance
  PopupMenu {
    id: popupMenu
  }

  property real popupX: 0
  property real popupY: 0
  property int tabContextMenuIndex: -1
  property bool tabContextMenuVisible: false
  property string tabContextMenuType: 'custom'
  property bool popupMenuVisible: false
  property bool externalFilesContextMenuVisible: false
  property int rightClickedIndex: -1
  property bool playlistContextMenuVisible: false
  property int playlistContextMenuX: 0
  property int playlistContextMenuY: 0
  property int playlistContextItemIndex: -1
  property string playlistContextItemName: ''
  property string playlistContextItemPath: ''
  property bool playlistContextIsFolder: false
  property bool renameDialogVisible: false
  property int renameDialogIndex: -1
  property bool prefDialogVisible: false

  // MINIMAL: 30% dari lebar/tinggi monitor user
  minimumWidth: Screen.width * 0.3
  minimumHeight: Screen.height * 0.3

  // MAKSIMAL: 100% dari area kerja monitor (biar gak bablas ke taskbar)
  maximumWidth: Screen.desktopAvailableWidth
  maximumHeight: Screen.desktopAvailableHeight

  Component.onCompleted: {
    root.flags = Qt.Window | Qt.FramelessWindowHint | Qt.CustomizeWindowHint
    
    // Restore window position
    var cfg = musicModel.get_window_config()
    var hasSavedPosition = (cfg && cfg.window_x >= 0 && cfg.window_y >= 0)
    
    if (hasSavedPosition) {
      root.x = cfg.window_x
      root.y = cfg.window_y
      root.width = cfg.window_width > 0 ? cfg.window_width : 350
      root.height = cfg.window_height > 0 ? cfg.window_height : 700
    } else {
      // Center on screen if no saved position
      root.x = Math.round((Screen.desktopAvailableWidth - root.width) / 2)
      root.y = Math.round((Screen.desktopAvailableHeight - root.height) / 2)
    }
    
    // Mark as initialized after position is set
    root.isInitialized = true
    
    musicModel.scan_music()
    musicModel.start_update_loop()
  }

  property bool isInitialized: false

  onXChanged: if (isInitialized) {
    musicModel.save_window_position(root.x, root.y, root.width, root.height)
  }
  onYChanged: if (isInitialized) {
    musicModel.save_window_position(root.x, root.y, root.width, root.height)
  }
  onWidthChanged: {
    musicModel.save_window_position(root.x, root.y, root.width, root.height)
  }
  onHeightChanged: {
    musicModel.save_window_position(root.x, root.y, root.width, root.height)
  }

  Timer {
    id: positionUpdateTimer
    interval: 33  // ~30fps, cukup smooth tanpa beban CPU
    running: true
    repeat: true
    triggeredOnStart: false
    onTriggered: {
        musicModel.update_tick()  // Rust engine update master clock
    }
}

  FontLoader {
    id: kodeMono
    source: 'qrc:/assets/fonts/KodeMono-VariableFont_wght.ttf'
  }
  FontLoader {
    id: symbols
    source: 'qrc:/assets/fonts/SymbolsNerdFont-Regular.ttf'
  }
  FontLoader {
    id: sansSerif
    source: 'qrc:/assets/fonts/Oswald-Regular.ttf'
  }
  FontLoader {
    id: emoji
    source: 'qrc:/assets/fonts/twemoji.ttf'
  }
  
  // Rename Dialog
  Item {
    id: renameDialogContainer
    visible: root.renameDialogVisible
    anchors.fill: parent
    z: 9999

    Rectangle {
      anchors.fill: parent
      color: '#40000000'

      MouseArea {
        anchors.fill: parent
        onClicked: {
          root.renameDialogVisible = false
        }
      }
    }

    Rectangle {
      id: renameDialogBox
      anchors.centerIn: parent
      width: 240
      height: 80
      color: theme.colormap.bgmain
      border.color: theme.colormap.playeraccent
      radius: 4
      antialiasing: false

      ColumnLayout {
        anchors.fill: parent
        anchors.margins: 10
        spacing: 8

        TextInput {
          id: renameInput
          Layout.fillWidth: true
          Layout.preferredHeight: 28
          text: musicModel.get_current_rename_name(root.renameDialogIndex)
          font.family: kodeMono.name
          font.pixelSize: 12
          color: theme.colormap.playeraccent
          verticalAlignment: Text.AlignVCenter
          maximumLength: 10
          activeFocusOnPress: true
          selectByMouse: true

          onAccepted: {
            if (text.trim().length > 0) {
              musicModel.rename_folder(root.renameDialogIndex, text.trim())
            }
            root.renameDialogVisible = false
          }

          Component.onCompleted: {
            forceActiveFocus()
            selectAll()
          }
        }

        RowLayout {
          Layout.fillWidth: true
          spacing: 16

          Text {
            text: 'CANCEL'
            font.family: kodeMono.name
            font.pixelSize: 10
            color: renameCancelMA.containsMouse ? theme.colormap.playlisticon : theme.colormap.tabtext
            MouseArea {
              id: renameCancelMA
              anchors.fill: parent
              hoverEnabled: true
              onClicked: {
                root.renameDialogVisible = false
              }
            }
          }

          Item { Layout.fillWidth: true }

          Text {
            text: 'SAVE'
            font.family: kodeMono.name
            font.pixelSize: 10
            color: renameSaveMA.containsMouse ? theme.colormap.playlisticon : theme.colormap.tabtext
            MouseArea {
              id: renameSaveMA
              anchors.fill: parent
              hoverEnabled: true
              onClicked: {
                if (renameInput.text.trim().length > 0) {
                  musicModel.rename_folder(root.renameDialogIndex, renameInput.text.trim())
                }
                root.renameDialogVisible = false
              }
            }
          }
        }
      }
    }
  }

  Rectangle {
    anchors.fill: parent
    color: theme.colormap.bgmain

    ColumnLayout {
      anchors.fill: parent
      spacing: 0

      // ==========================================
      // SECTION: HEADER (Titlebar)
      // ==========================================
      Rectangle {
        id: headerSection
        Layout.fillWidth: true
        Layout.preferredHeight: 26
        color: theme.colormap.headerbg

        RowLayout {
          anchors.left: parent.left
          anchors.right: parent.right
          anchors.verticalCenter: parent.verticalCenter
          anchors.leftMargin: 8
          anchors.rightMargin: 8
          spacing: 0

          MouseArea {
            anchors.fill: parent
            onPressed: root.startSystemMove()
            cursorShape: Qt.SizeAllCursor
          }

          Text {
            id: menuIcon
            text: '󰍜'
            font.family: kodeMono.name
            color: menuMA.containsMouse ? theme.colormap.headerhover : theme.colormap.headericon
            font.pixelSize: 24
            Layout.alignment: Qt.AlignVCenter

            MouseArea {
              id: menuMA
              anchors.fill: parent
              hoverEnabled: true
              onClicked: {
                root.prefDialogVisible = true
              }
            }
          }

          Item {
            Layout.fillWidth: true
            Layout.alignment: Qt.AlignVCenter
            Text {
              id: headerTitle
              anchors.centerIn: parent
              text: 'LOONIX TUNES'
              font.family: kodeMono.name
              font.pixelSize: 12
              color: theme.colormap.headericon
              horizontalAlignment: Text.AlignHCenter
            }
            MouseArea {
              id: headerTitleMouse
              anchors.fill: parent
              onDoubleClicked: {
                handleHeaderDoubleClick()
              }
            }
          }

          Text {
            text: '󰅖'
            font.family: symbols.name
            color: closeMA.containsMouse ? theme.colormap.headerhover : theme.colormap.headericon
            font.pixelSize: 18
            Layout.alignment: Qt.AlignVCenter

            MouseArea {
              id: closeMA
              anchors.fill: parent
              hoverEnabled: true
              onClicked: Qt.quit()
            }
          }
        }
      }

      // ==========================================
      // SECTION: PLAYER CORE
      // ==========================================
      Rectangle {
        id: playerSection
        Layout.fillWidth: true
        Layout.preferredHeight: 100
        color: 'transparent'
        property int currentSongIndex: -1
        property bool wasPlayingBeforeSeek: false

        Rectangle {
          x: 0
          y: 0
          width: 8
          height: parent.height
          color: theme.colormap.bgoverlay
        }

        Rectangle {
          x: parent.width - 8
          y: 0
          width: 8
          height: parent.height
          color: theme.colormap.bgoverlay
        }

        ColumnLayout {
          Layout.fillWidth: true
          anchors.left: parent.left
          anchors.right: parent.right
          anchors.leftMargin: 20
          anchors.rightMargin: 20
          anchors.verticalCenter: parent.verticalCenter
          spacing: 10

          Text {
            id: songTitleDisplay
            Layout.fillWidth: true
            text: musicModel.current_title || 'NO TRACK SELECTED'
            width: parent.width
            horizontalAlignment: Text.AlignHCenter
            font.family: kodeMono.name
            color: theme.colormap.playertitle
            font.pixelSize: 14
            elide: Text.ElideRight
          }

          RowLayout {
            Layout.fillWidth: true
            spacing: 5

            Text {
              id: currentTime
              text: '00:00'
              color: theme.colormap.playersubtext
              font.family: kodeMono.name
              font.pixelSize: 12
            }

            Slider {
              id: seekbar
              Layout.fillWidth: true
              from: 0
              to: 1
              onMoved: {
                // Seek only when user releases slider (Seek Guard)
                // onMoved fires after drag is complete
              }
              onValueChanged: {
                // Update visual only during drag
              }
              onPressedChanged: {
                if (pressed) {
                  // User started dragging - don't seek yet
                  // Just pause if playing
                  if (musicModel.is_playing) {
                    playerSection.wasPlayingBeforeSeek = true
                    musicModel.toggle_play()
                  }
                } else {
                  // User released - do seek now
                  var seekPos = seekbar.value * musicModel.duration
                  musicModel.seek_to(Math.floor(seekPos))
                  // Resume if it was playing before
                  if (playerSection.wasPlayingBeforeSeek) {
                    musicModel.toggle_play()
                    playerSection.wasPlayingBeforeSeek = false
                  }
                }
              }

              // Scroll wheel support for seek
              MouseArea {
                anchors.fill: parent
                acceptedButtons: Qt.NoButton
                onWheel: function(wheel) {
                  if (musicModel.duration <= 0) return
                  var step = 5000 // 5 seconds in ms
                  var delta = wheel.angleDelta.y > 0 ? step : -step
                  var newPos = Math.max(0, Math.min(musicModel.duration, musicModel.position + delta))
                  musicModel.seek_to(Math.floor(newPos))
                }
              }

              handle: Rectangle {
                x: seekbar.leftPadding + seekbar.visualPosition * (seekbar.availableWidth - width)
                y: seekbar.topPadding + seekbar.availableHeight / 2 - height / 2
                implicitWidth: 10
                implicitHeight: 10
                radius: 5
                color: seekbar.pressed ? theme.colormap.playerhover : theme.colormap.playeraccent
              }

              // AB Repeat markers
              Rectangle {
                id: pointA_marker
                visible: musicModel.ab_state > 0
                x: seekbar.leftPadding + (musicModel.duration > 0 ? (musicModel.ab_point_a / musicModel.duration) * seekbar.availableWidth : 0)
                y: seekbar.topPadding
                width: 2
                height: seekbar.availableHeight
                color: "#00FFFF" // Cyan
              }

              Rectangle {
                id: pointB_marker
                visible: musicModel.ab_state === 2
                x: seekbar.leftPadding + (musicModel.duration > 0 ? (musicModel.ab_point_b / musicModel.duration) * seekbar.availableWidth : 0)
                y: seekbar.topPadding
                width: 2
                height: seekbar.availableHeight
                color: "#FF4444" // Red
              }

              background: Rectangle {
                x: seekbar.leftPadding
                y: seekbar.topPadding + seekbar.availableHeight / 2 - height / 2
                implicitWidth: 200
                implicitHeight: 4
                width: seekbar.availableWidth
                height: 4
                radius: 2
                color: theme.colormap.bgoverlay
                Rectangle {
                  width: musicModel.duration > 0 && musicModel.position >= 0
                    ? (musicModel.position / musicModel.duration) * seekbar.availableWidth
                    : 0
                  height: parent.height
                  color: theme.colormap.playeraccent
                  radius: 2
                }
              }
            }

            Text {
              id: totalDuration
              text: musicModel.duration > 0 ? musicModel.format_time(musicModel.duration) : "--:--"
              color: theme.colormap.playersubtext
              font.family: kodeMono.name
              font.pixelSize: 12
            }
          }

          // PLAYER CORE CONTROL
          RowLayout {
            Layout.alignment: Qt.AlignHCenter
            Layout.fillWidth: true
            spacing: 25

            // SUFFLE | RANDOM
            Text {
              text: ''
              font.family: symbols.name
              font.pixelSize: 18
              color: musicModel.shuffle
                ? theme.colormap.playerhover
                : shuffleMA.containsMouse
                ? theme.colormap.playerhover
                : theme.colormap.playersubtext
              MouseArea {
                id: shuffleMA
                anchors.fill: parent
                hoverEnabled: true
                onClicked: musicModel.toggle_shuffle()
              }
            }

            // PREV
            Text {
              text: '󰙤'
              font.family: symbols.name
              font.pixelSize: 24
              color: prevMA.containsMouse ? theme.colormap.playerhover : theme.colormap.playeraccent
              MouseArea {
                id: prevMA
                anchors.fill: parent
                hoverEnabled: true
                onClicked: musicModel.play_prev()
              }
            }


            // PLAY | PAUSE
            Text {
              id: playBtn
              text: musicModel.is_playing ? '' : ''
              font.family: symbols.name
              font.pixelSize: 36
              color: playMA.containsMouse ? theme.colormap.playerhover : theme.colormap.playeraccent
              MouseArea {
                id: playMA
                anchors.fill: parent
                hoverEnabled: true
                onClicked: {
                  if (playerSection.currentSongIndex === -1 && musicModel.count > 0) {
                    playerSection.currentSongIndex = 0
                    musicModel.play_at(0)
                  } else {
                    musicModel.toggle_play()
                  }
                }
                onDoubleClicked: {
                  musicModel.stop_playback()
                }
              }
            }

            // NEXT
            Text {
              text: '󰙢'
              font.family: symbols.name
              font.pixelSize: 24
              color: nextMA.containsMouse ? theme.colormap.playerhover : theme.colormap.playeraccent
              MouseArea {
                id: nextMA
                anchors.fill: parent
                hoverEnabled: true
                onClicked: musicModel.play_next()
              }
            }

            // LOOP PLAYLIST
            Text {
              text: ''
              font.family: symbols.name
              font.pixelSize: 18
              color: musicModel.loop_playlist
                ? theme.colormap.playerhover
                : loopMA.containsMouse
                ? theme.colormap.playerhover
                : theme.colormap.playersubtext
              MouseArea {
                id: loopMA
                anchors.fill: parent
                hoverEnabled: true
                onClicked: musicModel.toggle_repeat()
              }
            }
          } //END PLAYER CORE CONTROL
        } //END PLAYER CORE ROW
      } //END PLAYER CORE

      // LOONIX DRAWER REMOVED - EQ/FX now use popup dialogs

      // ==========================================
      // SECTION: SPECIAL CONTROLS
      // ==========================================
      Rectangle {
        id: specialControlsSection
        Layout.fillWidth: true
        Layout.preferredHeight: 40
        color: 'transparent'

        // Border kiri
        Rectangle {
          width: 8
          anchors.left: parent.left
          anchors.top: parent.top
          anchors.bottom: parent.bottom
          color: theme.colormap.bgoverlay
        }

        // Border kanan
        Rectangle {
          width: 8
          anchors.right: parent.right
          anchors.top: parent.top
          anchors.bottom: parent.bottom
          color: theme.colormap.bgoverlay
        }

        // GANTI RowLayout utama jadi Item biasa biar bisa di-Anchor absolut
        Item {
          anchors.fill: parent
          anchors.leftMargin: 20
          anchors.rightMargin: 20

          // --- LEFT SECTION: PAN / AB---
          RowLayout {
            anchors.left: parent.left
            anchors.top: parent.top
            anchors.bottom: parent.bottom
            spacing: 8

            Text {
              id: panIcon
              text: '󰡌'
              font.family: symbols.name
              font.pixelSize: 18
              color: panMA.containsMouse ? theme.colormap.playerhover : theme.colormap.playersubtext
              Layout.alignment: Qt.AlignVCenter

              MouseArea {
                id: panMA
                anchors.fill: parent
                hoverEnabled: true
                onClicked: musicModel.set_balance(0.0)
              }
            }

            Slider {
              id: balanceSlider
              Layout.preferredWidth: 60
              implicitHeight: 26
              from: -1.0
              to: 1.0
              value: musicModel.balance
              onMoved: musicModel.set_balance(Number(value))

              MouseArea {
                anchors.fill: parent
                acceptedButtons: Qt.NoButton
                onWheel: function(wheel) {
                  var step = 0.05
                  var delta = wheel.angleDelta.y > 0 ? step : -step
                  var newVal = Math.max(-1.0, Math.min(1.0, balanceSlider.value + delta))
                  balanceSlider.value = newVal
                  musicModel.set_balance(newVal)
                }
              }

              handle: Rectangle {
                x: Math.round(
                  balanceSlider.leftPadding +
                    balanceSlider.visualPosition * (balanceSlider.availableWidth - width)
                )
                y: Math.round(balanceSlider.availableHeight / 2 - height / 2)
                width: 10
                height: 10
                radius: 5
                color: balanceSlider.pressed
                  ? theme.colormap.playerhover
                  : theme.colormap.playerhover
                antialiasing: false
              }

              background: Rectangle {
                x: balanceSlider.leftPadding
                y: Math.round(balanceSlider.availableHeight / 2 - height / 2)
                width: balanceSlider.availableWidth
                height: 4
                radius: 2
                color: theme.colormap.bgoverlay
                antialiasing: false

                Rectangle {
                  width: 2
                  height: 8
                  anchors.centerIn: parent
                  color: theme.colormap.playersubtext
                  opacity: 0.5
                }
              }
            }

            // AB REPEAT
            Text {
              id: abRepeatIcon
              text: '󰇉'
              font.family: symbols.name
              font.pixelSize: 18
              color: abRepeatMA.containsMouse ? theme.colormap.playerhover : theme.colormap.playersubtext
              Layout.alignment: Qt.AlignVCenter

              MouseArea {
                id: abRepeatMA
                anchors.fill: parent
                hoverEnabled: true
                onClicked: {
                  musicModel.toggle_abrepeat()
                }
              }
            }
            
          }

          // --- MIDDLE INSTANT FX ---
          RowLayout {
            anchors.horizontalCenter: parent.horizontalCenter
            anchors.top: parent.top
            anchors.bottom: parent.bottom
            spacing: 15

            

            // BASSBOOSTER
            Item {
              id: bassboosterContainer
              width: bassboosterIcon.width
              height: 40
              Layout.alignment: Qt.AlignVCenter

              Text {
                id: bassboosterTooltip
                anchors.horizontalCenter: parent.horizontalCenter
                anchors.bottom: bassboosterIcon.top
                anchors.bottomMargin: 4
                text: "BassBooster"
                font.pixelSize: 14
                font.family: kodeMono.name
                color: theme.colormap.playerhover
                visible: bassboosterMA.containsMouse
              }

              Text {
                id: bassboosterIcon
                anchors.verticalCenter: parent.verticalCenter
                text: musicModel.bassbooster_active ? '󰬉' : '󰯮'
                font.family: symbols.name
                font.pixelSize: 18
                color: musicModel.bassbooster_active || bassboosterMA.containsMouse
                  ? theme.colormap.playerhover
                  : theme.colormap.playersubtext
              }

              MouseArea {
                id: bassboosterMA
                anchors.fill: bassboosterIcon
                hoverEnabled: true
                onClicked: musicModel.toggle_bassbooster()
              }
            }

            

            // CRYSTALIZER
            Item {
              id: crystalizerContainer
              width: crystalizerIcon.width
              height: 40
              Layout.alignment: Qt.AlignVCenter

              Text {
                id: crystalizerTooltip
                anchors.horizontalCenter: parent.horizontalCenter
                anchors.bottom: crystalizerIcon.top
                anchors.bottomMargin: 4
                text: "Crystalizer"
                font.pixelSize: 14  
                font.family: kodeMono.name
                color: theme.colormap.playerhover
                visible: crystalizerMA.containsMouse
              }

              Text {
                id: crystalizerIcon
                anchors.verticalCenter: parent.verticalCenter
                text: musicModel.crystalizer_active ? '󰬊' : '󰯱'
                font.family: symbols.name
                font.pixelSize: 18
                color: musicModel.crystalizer_active || crystalizerMA.containsMouse
                  ? theme.colormap.playerhover
                  : theme.colormap.playersubtext
              }

              MouseArea {
                id: crystalizerMA
                anchors.fill: crystalizerIcon
                hoverEnabled: true
                onClicked: musicModel.toggle_crystalizer()
              }
            }

            // EQUALIZER
            Item {
              id: eqContainer
              width: eqIconSlider.width
              height: 40
              Layout.alignment: Qt.AlignVCenter

              Text {
                id: eqTooltip
                anchors.horizontalCenter: parent.horizontalCenter
                anchors.bottom: eqIconSlider.top
                anchors.bottomMargin: 4
                text: "Equalizer"
                font.pixelSize: 14
                font.family: kodeMono.name
                color: theme.colormap.playerhover
                visible: eqMASlider.containsMouse
              }

              Text {
                id: eqIconSlider
                anchors.verticalCenter: parent.verticalCenter
                text: '󰯷'
                font.family: symbols.name
                font.pixelSize: 18
                color: eqMASlider.containsMouse || eq.visible
                  ? theme.colormap.playerhover
                  : theme.colormap.playersubtext

                MouseArea {
                  id: eqMASlider
                  anchors.fill: parent
                  hoverEnabled: true
                  onClicked: {
                    if (eq.visible) {
                      eq.close()
                    } else {
                      eq.open()
                    }
                  }
                }
              }
            }

            // FX
            Item {
              id: fxContainer
              width: fxIconSlider.width
              height: 40
              Layout.alignment: Qt.AlignVCenter

              Text {
                id: fxTooltip
                anchors.horizontalCenter: parent.horizontalCenter
                anchors.bottom: fxIconSlider.top
                anchors.bottomMargin: 4
                text: "FX"
                font.pixelSize: 14
                font.family: kodeMono.name
                color: theme.colormap.playerhover
                visible: presetMASlider.containsMouse
              }

              Text {
                id: fxIconSlider
                anchors.verticalCenter: parent.verticalCenter
                text: '󰯺'
                font.family: symbols.name
                font.pixelSize: 18

                color: presetMASlider.containsMouse || fx.visible
                  ? theme.colormap.playerhover
                  : theme.colormap.playersubtext

                MouseArea {
                  id: presetMASlider
                  anchors.fill: parent
                  hoverEnabled: true
                  onClicked: {
                    if (fx.visible) {
                      fx.close()
                    } else {
                      fx.open()
                    }
                  }
                }
              }
            }

            // SURROUND
            Item {
              id: surroundContainer
              width: surroundIcon.width
              height: 40
              Layout.alignment: Qt.AlignVCenter

              Text {
                id: surroundTooltip
                anchors.horizontalCenter: parent.horizontalCenter
                anchors.bottom: surroundIcon.top
                anchors.bottomMargin: 4
                text: "Surround"
                font.pixelSize: 14
                font.family: kodeMono.name
                color: theme.colormap.playerhover
                visible: surroundMA.containsMouse
              }

              Text {
                id: surroundIcon
                anchors.verticalCenter: parent.verticalCenter
                text: musicModel.surround_active ? '󰬚' : '󰰡'
                font.family: symbols.name
                font.pixelSize: 18
                color: musicModel.surround_active || surroundMA.containsMouse
                  ? theme.colormap.playerhover
                  : theme.colormap.playersubtext
              }

              MouseArea {
                id: surroundMA
                anchors.fill: surroundIcon
                hoverEnabled: true
                onClicked: musicModel.toggle_surround()
              }
            }

            // THEME icon 󰰤
            Item {
              id: themeContainer
              width: themeIcon.width
              height: 40
              Layout.alignment: Qt.AlignVCenter

              Text {
                id: themeTooltip
                anchors.horizontalCenter: parent.horizontalCenter
                anchors.bottom: themeIcon.top
                anchors.bottomMargin: 4
                text: theme.current_theme
                font.pixelSize: 14
                font.family: kodeMono.name
                color: theme.colormap.playerhover
                visible: themeMA.containsMouse
              }

              Text {
                id: themeIcon
                anchors.verticalCenter: parent.verticalCenter
                text: '󰰤'
                font.family: symbols.name
                font.pixelSize: 18
                color: themeMA.containsMouse
                  ? theme.colormap.playerhover
                  : theme.colormap.playersubtext
              }

              MouseArea {
                id: themeMA
                anchors.fill: themeIcon
                hoverEnabled: true
                onClicked: theme.cycle_theme()
              }
            }

          }

          // --- RIGHT SECTION: VOLUME ---
          RowLayout {
            anchors.right: parent.right
            anchors.top: parent.top
            anchors.bottom: parent.bottom
            spacing: 8

            Text {
              id: volIcon
              text: musicModel.muted ? '󰝟' : '󰕾'
              font.family: symbols.name
              font.pixelSize: 18
              Layout.alignment: Qt.AlignVCenter
              horizontalAlignment: Text.AlignHCenter
              Layout.preferredWidth: 20
              color: volMA.containsMouse ? theme.colormap.playerhover : theme.colormap.playersubtext

              MouseArea {
                id: volMA
                anchors.fill: parent
                hoverEnabled: true
                onClicked: musicModel.toggle_mute()
              }
            }

            Slider {
              id: volSlider
              Layout.preferredWidth: 56
              implicitHeight: 26
              from: 0.0
              to: 1.0
              value: musicModel.volume
              onMoved: musicModel.set_volume(Number(value))

              property bool showTooltip: volSlider.pressed || volTimer.running

              Timer {
                id: volTimer
                interval: 800
              }

              MouseArea {
                anchors.fill: parent
                acceptedButtons: Qt.NoButton
                onWheel: function(wheel) {
                  var step = 0.05
                  var delta = wheel.angleDelta.y > 0 ? step : -step
                  var newVal = Math.max(0.0, Math.min(1.0, volSlider.value + delta))
                  volSlider.value = newVal
                  musicModel.set_volume(newVal)
                  volTimer.restart()
                }
              }

              handle: Rectangle {
                x: Math.round(
                  volSlider.leftPadding +
                    volSlider.visualPosition * (volSlider.availableWidth - width)
                )
                y: Math.round(volSlider.availableHeight / 2 - height / 2)
                width: 10
                height: 10
                radius: 5
                color: volSlider.pressed ? theme.colormap.playerhover : theme.colormap.playeraccent
              }

              Rectangle {
                visible: volSlider.showTooltip
                x: Math.round(
                  volSlider.leftPadding +
                    volSlider.visualPosition * (volSlider.availableWidth - width) +
                    volSlider.handle.width / 2 - width / 2
                )
                y: volSlider.handle.y - height - 3
                width: volPercentText.implicitWidth + 10
                height: volPercentText.implicitHeight + 6
                radius: 4
                color: theme.colormap.bgoverlay
                border.color: theme.colormap.playerhover
                border.width: 1
                antialiasing: false

                Text {
                  id: volPercentText
                  anchors.centerIn: parent
                  text: Math.round(volSlider.value * 100) + "%"
                  color: theme.colormap.playerhover
                  font.family: kodeMono.name
                  font.pixelSize: 11
                  font.bold: true
                }
              }

              background: Rectangle {
                x: volSlider.leftPadding
                y: Math.round(volSlider.availableHeight / 2 - height / 2)
                width: volSlider.availableWidth
                height: 4
                radius: 2
                color: theme.colormap.bgoverlay

                Rectangle {
                  width: Math.round(volSlider.visualPosition * parent.width)
                  height: parent.height
                  color: theme.colormap.playerhover
                  radius: 2
                }
              }
            }
          }
        }
      }

      // PANEL TAB
      Tab {
        id: mainTabBar
        Layout.fillWidth: true
      }

      // ==========================================
      // SECTION: PLAYLIST
      // ==========================================
      Playlist {
        id: playlistSection
        Layout.fillWidth: true
        Layout.fillHeight: true
      }
    }
  }

  // ==========================================
  // SECTION: POPUPS (Panggilan Eksternal)
  // ==========================================
  Eq {
      id: eq
      x: (parent.width - width) / 2
      y: 171
      width: 500
  }

  Fx {
      id: fx
      x: (parent.width - width) / 2
      y: 171
      width: 500
  }

  Pref {
      id: prefPopup
      visible: root.prefDialogVisible
  }

// ==========================================
  // SECTION: CONTEXT MENUS
  // ==========================================
  TabContextMenu {}
  PlaylistContextMenu {}
  TrackInfo {}

  // ==========================================
  // SECTION: SYSTEM & LOGIC CONNECTIONS
  // ==========================================
  Timer {
    id: updatePollTimer
    interval: 500
    repeat: true
    running: false
    onTriggered: musicModel.poll_update_result()
  }

  Connections {
    target: musicModel
    function onUpdate_status_changed() {
      updatePollTimer.running = (musicModel.update_status === "Checking for updates...")
    }
    
    function onPositionChanged() {
      currentTime.text = musicModel.format_time(musicModel.position)
      if (musicModel.duration > 0 && !seekbar.pressed) {
        seekbar.value = musicModel.position / musicModel.duration
      }
    }

    function onDurationChanged() {
      totalDuration.text = musicModel.format_time(musicModel.duration)
    }

    function onBalance_changed() {
      balanceSlider.value = musicModel.balance
    }
  }

  // ==========================================
  // SECTION: KEYBOARD SHORTCUTS
  // ==========================================
  function adjustVolume(delta) {
    var step = 0.05
    var newVal = Math.max(0.0, Math.min(1.0, musicModel.volume + (delta * step)))
    volSlider.value = newVal
    musicModel.set_volume(newVal)
    volTimer.restart()
  }

  Shortcut { sequence: "+"; onActivated: adjustVolume(1) }
  Shortcut { sequence: "="; onActivated: adjustVolume(1) }
  Shortcut { sequence: "-"; onActivated: adjustVolume(-1) }
  Shortcut { sequence: "_"; onActivated: adjustVolume(-1) }
  Shortcut { 
    sequence: "M"
    onActivated: { musicModel.toggle_mute(); volTimer.restart() }
  }
  Shortcut { 
    sequence: "Escape"
    onActivated: { root.renameDialogVisible = false } // Atau tambahin settingsPopup.close() kalau perlu
  }

  // ==========================================
  // SECTION: WINDOW RESIZE HANDLERS (BORDERS)
  // ==========================================
  MouseArea { width: 6; anchors.top: parent.top; anchors.bottom: parent.bottom; anchors.left: parent.left; cursorShape: Qt.SizeHorCursor; onPressed: root.startSystemResize(Qt.LeftEdge) }
  MouseArea { width: 6; anchors.top: parent.top; anchors.bottom: parent.bottom; anchors.right: parent.right; cursorShape: Qt.SizeHorCursor; onPressed: root.startSystemResize(Qt.RightEdge) }
  MouseArea { height: 6; anchors.left: parent.left; anchors.right: parent.right; anchors.top: parent.top; cursorShape: Qt.SizeVerCursor; onPressed: root.startSystemResize(Qt.TopEdge) }
  MouseArea { height: 6; anchors.left: parent.left; anchors.right: parent.right; anchors.bottom: parent.bottom; cursorShape: Qt.SizeVerCursor; onPressed: root.startSystemResize(Qt.BottomEdge) }
  MouseArea { width: 10; height: 10; anchors.left: parent.left; anchors.top: parent.top; cursorShape: Qt.SizeFDiagCursor; onPressed: root.startSystemResize(Qt.LeftEdge | Qt.TopEdge) }
  MouseArea { width: 10; height: 10; anchors.right: parent.right; anchors.top: parent.top; cursorShape: Qt.SizeBDiagCursor; onPressed: root.startSystemResize(Qt.RightEdge | Qt.TopEdge) }
  MouseArea { width: 10; height: 10; anchors.left: parent.left; anchors.bottom: parent.bottom; cursorShape: Qt.SizeBDiagCursor; onPressed: root.startSystemResize(Qt.LeftEdge | Qt.BottomEdge) }
  MouseArea { width: 10; height: 10; anchors.right: parent.right; anchors.bottom: parent.bottom; cursorShape: Qt.SizeFDiagCursor; onPressed: root.startSystemResize(Qt.RightEdge | Qt.BottomEdge) }

}
