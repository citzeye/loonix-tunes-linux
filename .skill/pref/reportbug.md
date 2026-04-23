1. Backend: Fungsi "Penyusun Laporan" (src/ui/theme.rs atau file lain)
Kita bikin fungsi di Rust buat nyusun URL GitHub dan ngebukanya di browser.

Rust
// Tambahin di impl ThemeManager atau sejenisnya
#[qt_method]
pub fn report_bug_on_github(&self, bug_title: String, bug_desc: String) {
    let repo_url = "https://github.com/citzeye/loonix-tunes-linux/issues/new";
    
    // Ambil info sistem biar user gak perlu ngetik manual
    let os_info = std::env::consts::OS;
    let arch = std::env::consts::ARCH;
    
    // Susun template body-nya
    let body = format!(
        "### Describe the bug\n{}\n\n### System Info\n- OS: {}\n- Arch: {}\n- Version: v1.0.2",
        bug_desc, os_info, arch
    );

    // Encode biar URL-nya gak berantakan
    let encoded_title = urlencoding::encode(&bug_title);
    let encoded_body = urlencoding::encode(&body);

    let final_url = format!("{}?title={}&body={}", repo_url, encoded_title, encoded_body);

    // Buka browser
    let _ = std::process::Command::new("xdg-open")
        .arg(final_url)
        .spawn();
}
(Note: Pastiin tambah urlencoding = "2.1" di Cargo.toml biar karakter aneh nggak bikin URL patah).

2. Frontend: qml/ui/pref/ReportBug.qml
Bikin tampilannya simpel, compact, dan tetep selaras sama warna colormap.

QML
import QtQuick 2.15
import QtQuick.Layouts 1.15
import QtQuick.Controls 2.15

Rectangle {
    id: root
    color: theme.colormap.bgmain
    radius: 6
    clip: true

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: 15
        spacing: 12

        Text {
            text: "REPORT BUG / FEEDBACK"
            font.family: kodeMono.name
            font.pixelSize: 14
            font.bold: true
            color: theme.colormap.playeraccent
        }

        // Input Judul
        Rectangle {
            Layout.fillWidth: true
            Layout.preferredHeight: 35
            color: theme.colormap.bgoverlay
            border.color: titleInput.activeFocus ? theme.colormap.playeraccent : theme.colormap.graysolid
            
            TextInput {
                id: titleInput
                anchors.fill: parent
                anchors.margins: 8
                color: theme.colormap.tabtext
                font.pixelSize: 12
                verticalAlignment: Text.AlignVCenter
                clip: true
                // Placeholder
                Text {
                    text: "Judul masalah..."
                    color: theme.colormap.playersubtext
                    visible: !parent.text && !parent.activeFocus
                    anchors.fill: parent
                    verticalAlignment: Text.AlignVCenter
                }
            }
        }

        // Input Deskripsi
        Rectangle {
            Layout.fillWidth: true
            Layout.fillHeight: true
            color: theme.colormap.bgoverlay
            border.color: descInput.activeFocus ? theme.colormap.playeraccent : theme.colormap.graysolid

            Flickable {
                anchors.fill: parent
                contentWidth: width
                contentHeight: descInput.implicitHeight
                clip: true

                TextEdit {
                    id: descInput
                    width: parent.width
                    padding: 8
                    color: theme.colormap.tabtext
                    font.pixelSize: 12
                    wrapMode: TextEdit.Wrap
                    selectByMouse: true
                    
                    Text {
                        text: "Jelaskan kronologi bugnya bray..."
                        color: theme.colormap.playersubtext
                        visible: !parent.text && !parent.activeFocus
                        x: 8; y: 8
                    }
                }
            }
        }

        // Tombol Submit
        Rectangle {
            Layout.alignment: Qt.AlignRight
            width: 120
            height: 35
            radius: 4
            color: (titleInput.text && descInput.text) ? theme.colormap.playeraccent : theme.colormap.graysolid
            
            Text {
                anchors.centerIn: parent
                text: "OPEN GITHUB"
                font.bold: true
                font.pixelSize: 11
                color: theme.colormap.bgmain
            }

            MouseArea {
                anchors.fill: parent
                cursorShape: Qt.PointingHandCursor
                onClicked: {
                    if (titleInput.text && descInput.text) {
                        theme.report_bug_on_github(titleInput.text, descInput.text)
                        // Reset form setelah kirim
                        titleInput.text = ""
                        descInput.text = ""
                    }
                }
            }
        }
    }
}