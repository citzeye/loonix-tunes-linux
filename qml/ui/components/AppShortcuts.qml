import QtQuick
import QtQuick.Controls

Item {
    id: appShortcuts

    function isTyping() {
        var focus = activeFocusItem;
        while (focus) {
            if (focus.TextInput || focus.TextField) return true;
            focus = focus.activeFocusItem;
        }
        return false;
    }

    // === PLAYBACK ===
    Shortcut {
        context: Qt.ApplicationShortcut
        sequence: "Space"
        onActivated: if (!isTyping()) musicModel.toggle_play();
    }
    Shortcut {
        context: Qt.ApplicationShortcut
        sequence: "M"
        onActivated: if (!isTyping()) musicModel.toggle_mute();
    }
    Shortcut {
        context: Qt.ApplicationShortcut
        sequence: "Ctrl+S"
        onActivated: musicModel.stop_playback();
    }
    Shortcut {
        context: Qt.ApplicationShortcut
        sequence: "Ctrl+Right"
        onActivated: musicModel.play_next();
    }
    Shortcut {
        context: Qt.ApplicationShortcut
        sequence: "Ctrl+Left"
        onActivated: musicModel.play_prev();
    }
    Shortcut {
        context: Qt.ApplicationShortcut
        sequence: "Shift+Right"
        onActivated: {
            var np = Math.min(musicModel.duration, musicModel.position + 5000);
            musicModel.seek_to(Math.floor(np));
        }
    }
    Shortcut {
        context: Qt.ApplicationShortcut
        sequence: "Shift+Left"
        onActivated: {
            var np = Math.max(0, musicModel.position - 5000);
            musicModel.seek_to(Math.floor(np));
        }
    }

    // === VOLUME ===
    Shortcut {
        context: Qt.ApplicationShortcut
        sequence: "Ctrl+Up"
        onActivated: musicModel.set_volume(Math.min(1, musicModel.volume + 0.05));
    }
    Shortcut {
        context: Qt.ApplicationShortcut
        sequence: "Ctrl+Down"
        onActivated: musicModel.set_volume(Math.max(0, musicModel.volume - 0.05));
    }

    // === MODES ===
    Shortcut {
        context: Qt.ApplicationShortcut
        sequence: "Ctrl+H"
        onActivated: musicModel.toggle_shuffle();
    }
    Shortcut {
        context: Qt.ApplicationShortcut
        sequence: "Ctrl+R"
        onActivated: musicModel.toggle_repeat();
    }
    Shortcut {
        context: Qt.ApplicationShortcut
        sequence: "Ctrl+L"
        onActivated: musicModel.toggle_abloop();
    }

    // === TAB NAV ===
    Shortcut {
        context: Qt.ApplicationShortcut
        sequence: "1"
        onActivated: if (!isTyping()) musicModel.switch_to_music();
    }
    Shortcut {
        context: Qt.ApplicationShortcut
        sequence: "2"
        onActivated: if (!isTyping()) musicModel.switch_to_favorites();
    }
    Shortcut {
        context: Qt.ApplicationShortcut
        sequence: "Q"
        onActivated: if (!isTyping()) musicModel.switch_to_queue();
    }
    Shortcut {
        context: Qt.ApplicationShortcut
        sequence: "3"
        onActivated: if (!isTyping()) musicModel.switch_to_external_files();
    }

    // === QUEUE ===
    Shortcut {
        context: Qt.ApplicationShortcut
        sequence: "Ctrl+Shift+A"
        onActivated: musicModel.add_to_queue(musicModel.current_path, musicModel.current_title);
    }
    Shortcut {
        context: Qt.ApplicationShortcut
        sequence: "Delete"
        onActivated: musicModel.remove_from_queue(musicModel.current_index);
    }
    Shortcut {
        context: Qt.ApplicationShortcut
        sequence: "Ctrl+Shift+C"
        onActivated: musicModel.clear_queue();
    }

    // === SEARCH ===
    Shortcut {
        context: Qt.ApplicationShortcut
        sequence: "Ctrl+F"
        onActivated: {
            var inputs = root.children.filter(c => c.objectName === "searchInput");
            if (inputs.length > 0) {
                inputs[0].forceActiveFocus();
            } else {
                var playlistItems = root.children.filter(function(c) { return c.hasOwnProperty && c.searchInput; });
                if (playlistItems.length > 0 && playlistItems[0].searchInput) {
                    playlistItems[0].searchInput.forceActiveFocus();
                }
            }
        }
    }

    // === LIBRARY ===
    Shortcut {
        context: Qt.ApplicationShortcut
        sequence: "F5"
        onActivated: musicModel.scan_music();
    }
    Shortcut {
        context: Qt.ApplicationShortcut
        sequence: "Ctrl+N"
        onActivated: musicModel.add_song("");
    }
    Shortcut {
        context: Qt.ApplicationShortcut
        sequence: "Shift+F"
        onActivated: if (!isTyping()) musicModel.toggle_favorite(musicModel.current_path, musicModel.current_title);
    }

    // === DSP TOGGLES ===
    Shortcut {
        context: Qt.ApplicationShortcut
        sequence: "D"
        onActivated: if (!isTyping()) dspModel.toggle_dsp();
    }
    Shortcut {
        context: Qt.ApplicationShortcut
        sequence: "B"
        onActivated: if (!isTyping()) dspModel.toggle_bass_booster();
    }
    Shortcut {
        context: Qt.ApplicationShortcut
        sequence: "R"
        onActivated: if (!isTyping()) dspModel.toggle_reverb();
    }
    Shortcut {
        context: Qt.ApplicationShortcut
        sequence: "S"
        onActivated: if (!isTyping()) dspModel.toggle_surround();
    }
    Shortcut {
        context: Qt.ApplicationShortcut
        sequence: "C"
        onActivated: if (!isTyping()) dspModel.toggle_crystalizer();
    }
    Shortcut {
        context: Qt.ApplicationShortcut
        sequence: "X"
        onActivated: if (!isTyping()) dspModel.toggle_compressor();
    }
    Shortcut {
        context: Qt.ApplicationShortcut
        sequence: "L"
        onActivated: if (!isTyping()) dspModel.toggle_middle_clarity();
    }
    Shortcut {
        context: Qt.ApplicationShortcut
        sequence: "W"
        onActivated: if (!isTyping()) dspModel.toggle_stereo_width();
    }
    Shortcut {
        context: Qt.ApplicationShortcut
        sequence: "N"
        onActivated: if (!isTyping()) dspModel.toggle_normalizer();
    }

    // === DSP PRESETS ===
    Shortcut {
        context: Qt.ApplicationShortcut
        sequence: "Ctrl+P"
        onActivated: dspModel.load_preset(0);
    }
    Shortcut {
        context: Qt.ApplicationShortcut
        sequence: "Ctrl+Shift+S"
        onActivated: dspModel.save_user_preset(0, "User");
    }
    Shortcut {
        context: Qt.ApplicationShortcut
        sequence: "Ctrl+0"
        onActivated: dspModel.reset_all();
    }

    // === THEME ===
    Shortcut {
        context: Qt.ApplicationShortcut
        sequence: "T"
        onActivated: if (!isTyping()) theme.cycle_theme();
    }

    // === WINDOW ===
    Shortcut {
        context: Qt.ApplicationShortcut
        sequence: "Ctrl+M"
        onActivated: root.showMinimized();
    }
    Shortcut {
        context: Qt.ApplicationShortcut
        sequence: "Ctrl+W"
        onActivated: Qt.quit();
    }
    Shortcut {
        context: Qt.ApplicationShortcut
        sequence: "Ctrl+,"
        onActivated: root.prefDialogVisible = true;
    }
    Shortcut {
        context: Qt.ApplicationShortcut
        sequence: "Escape"
        onActivated: {
            root.prefDialogVisible = false;
            root.renameDialogVisible = false;
        }
    }

    // === FULLSCREEN ===
    Shortcut {
        context: Qt.ApplicationShortcut
        sequence: "F11"
        onActivated: root.visibility === Window.FullScreen ? root.showNormal() : root.showFullScreen();
    }

    // === CONTEXT MENU ===
    Shortcut {
        context: Qt.ApplicationShortcut
        sequence: "Shift+F10"
        onActivated: {
            if (musicModel.current_index >= 0) {
                root.playlistContextMenuVisible = true;
            }
        }
    }
}