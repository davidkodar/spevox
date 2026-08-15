import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtQuick.Window
import Qt.labs.platform as Platform
import io.github.davidkodar.FluidVoiceLinux

ApplicationWindow {
    id: root
    width: 880
    height: 620
    minimumWidth: 760
    minimumHeight: 540
    visible: true
    title: qsTr("FluidVoice")
    color: "#111216"

    readonly property color accent: "#8b7cff"
    readonly property color panel: "#1a1b21"
    readonly property color panelRaised: "#22232b"
    readonly property color primaryText: "#f6f4ff"
    readonly property color secondaryText: "#aaa8b4"

    FluidVoiceController {
        id: controller
    }

    Component.onCompleted: controller.initializeAudio()

    Platform.SystemTrayIcon {
        visible: true
        icon.name: "audio-input-microphone"
        tooltip: qsTr("FluidVoice — %1").arg(controller.statusText)
        menu: Platform.Menu {
            Platform.MenuItem {
                text: qsTr("Open FluidVoice")
                onTriggered: {
                    root.show()
                    root.raise()
                    root.requestActivate()
                }
            }
            Platform.MenuItem {
                text: controller.recording ? qsTr("Stop recording") : qsTr("Start recording")
                onTriggered: controller.toggleRecording()
            }
            Platform.MenuSeparator {}
            Platform.MenuItem {
                text: qsTr("Quit")
                onTriggered: Qt.quit()
            }
        }
        onActivated: function(reason) {
            if (reason === Platform.SystemTrayIcon.Trigger) {
                root.visible = !root.visible
                if (root.visible) {
                    root.raise()
                    root.requestActivate()
                }
            }
        }
    }

    background: Rectangle {
        color: root.color
        gradient: Gradient {
            GradientStop { position: 0.0; color: "#171822" }
            GradientStop { position: 1.0; color: "#0e0f13" }
        }
    }

    header: Rectangle {
        height: 76
        color: "#15161b"
        border.color: "#292a32"
        border.width: 1

        RowLayout {
            anchors.fill: parent
            anchors.leftMargin: 28
            anchors.rightMargin: 24
            spacing: 14

            Rectangle {
                width: 42
                height: 42
                radius: 13
                color: root.accent

                Text {
                    anchors.centerIn: parent
                    text: "◉"
                    color: "white"
                    font.pixelSize: 23
                    font.weight: Font.DemiBold
                }
            }

            ColumnLayout {
                spacing: 1
                Text {
                    text: "FluidVoice"
                    color: root.primaryText
                    font.pixelSize: 20
                    font.weight: Font.DemiBold
                }
                Text {
                    text: qsTr("Native dictation for KDE Plasma")
                    color: root.secondaryText
                    font.pixelSize: 12
                }
            }

            Item { Layout.fillWidth: true }

            Rectangle {
                implicitWidth: statusRow.implicitWidth + 24
                implicitHeight: 34
                radius: 17
                color: controller.recording ? "#352f5d" : "#202129"
                border.color: controller.recording ? root.accent : "#343640"

                Row {
                    id: statusRow
                    anchors.centerIn: parent
                    spacing: 8
                    Rectangle {
                        anchors.verticalCenter: parent.verticalCenter
                        width: 8
                        height: 8
                        radius: 4
                        color: controller.recording ? "#ad9fff" : "#65d49a"
                    }
                    Text {
                        text: controller.statusText
                        color: root.primaryText
                        font.pixelSize: 12
                        font.weight: Font.Medium
                    }
                }
            }
        }
    }

    RowLayout {
        anchors.fill: parent
        anchors.margins: 24
        spacing: 20

        Rectangle {
            Layout.preferredWidth: 210
            Layout.fillHeight: true
            radius: 20
            color: "#17181e"
            border.color: "#292a33"

            ColumnLayout {
                anchors.fill: parent
                anchors.margins: 14
                spacing: 7

                Repeater {
                    model: ["General", "Audio", "Transcription", "Shortcuts", "Appearance"]
                    delegate: Rectangle {
                        required property string modelData
                        required property int index
                        Layout.fillWidth: true
                        height: 44
                        radius: 12
                        color: index === 0 ? "#2b2940" : "transparent"

                        Text {
                            anchors.left: parent.left
                            anchors.leftMargin: 15
                            anchors.verticalCenter: parent.verticalCenter
                            text: modelData
                            color: index === 0 ? "#f4f0ff" : root.secondaryText
                            font.pixelSize: 13
                            font.weight: index === 0 ? Font.DemiBold : Font.Normal
                        }
                    }
                }

                Item { Layout.fillHeight: true }

                Text {
                    Layout.alignment: Qt.AlignHCenter
                    text: "Private preview · 0.1.0"
                    color: "#74727c"
                    font.pixelSize: 10
                }
            }
        }

        Flickable {
            Layout.fillWidth: true
            Layout.fillHeight: true
            contentHeight: contentColumn.implicitHeight
            clip: true

            ColumnLayout {
                id: contentColumn
                width: parent.width
                spacing: 16

                ColumnLayout {
                    spacing: 5
                    Text {
                        text: qsTr("General")
                        color: root.primaryText
                        font.pixelSize: 26
                        font.weight: Font.DemiBold
                    }
                    Text {
                        text: qsTr("Choose how FluidVoice listens and responds.")
                        color: root.secondaryText
                        font.pixelSize: 13
                    }
                }

                Rectangle {
                    Layout.fillWidth: true
                    height: 138
                    radius: 18
                    color: root.panel
                    border.color: "#2d2e37"

                    ColumnLayout {
                        anchors.fill: parent
                        anchors.margins: 20
                        spacing: 14
                        Text { text: qsTr("INPUT & MODEL"); color: "#777581"; font.pixelSize: 10; font.letterSpacing: 1.2 }

                        RowLayout {
                            Layout.fillWidth: true
                            ColumnLayout {
                                Layout.fillWidth: true
                                spacing: 3
                                Text { text: qsTr("Microphone"); color: root.primaryText; font.pixelSize: 13; font.weight: Font.Medium }
                                Text { text: controller.microphoneName; color: root.secondaryText; font.pixelSize: 12; elide: Text.ElideRight; Layout.fillWidth: true }
                            }
                            Rectangle { width: 1; height: 38; color: "#34353e" }
                            ColumnLayout {
                                Layout.fillWidth: true
                                spacing: 3
                                Text { text: qsTr("Local model"); color: root.primaryText; font.pixelSize: 13; font.weight: Font.Medium }
                                Text { text: controller.modelName; color: root.secondaryText; font.pixelSize: 12; elide: Text.ElideRight; Layout.fillWidth: true }
                            }
                        }
                    }
                }

                Rectangle {
                    Layout.fillWidth: true
                    height: 180
                    radius: 18
                    color: root.panel
                    border.color: "#2d2e37"

                    ColumnLayout {
                        anchors.fill: parent
                        anchors.margins: 20
                        spacing: 14
                        Text { text: qsTr("DICTATION"); color: "#777581"; font.pixelSize: 10; font.letterSpacing: 1.2 }

                        RowLayout {
                            Layout.fillWidth: true
                            ColumnLayout {
                                Layout.fillWidth: true
                                spacing: 4
                                Text { text: qsTr("Hold to dictate"); color: root.primaryText; font.pixelSize: 14; font.weight: Font.Medium }
                                Text { text: qsTr("Recording stops when the shortcut is released."); color: root.secondaryText; font.pixelSize: 12 }
                            }
                            Rectangle {
                                width: shortcutLabel.implicitWidth + 22
                                height: 32
                                radius: 9
                                color: "#25262e"
                                border.color: "#3a3b45"
                                Text { id: shortcutLabel; anchors.centerIn: parent; text: "Ctrl  Alt  D"; color: "#dedbe8"; font.pixelSize: 11; font.family: "monospace" }
                            }
                        }

                        Rectangle { Layout.fillWidth: true; height: 1; color: "#2d2e36" }

                        RowLayout {
                            Layout.fillWidth: true
                            ColumnLayout {
                                Layout.fillWidth: true
                                spacing: 4
                                Text { text: qsTr("Recording overlay"); color: root.primaryText; font.pixelSize: 14; font.weight: Font.Medium }
                                Text { text: qsTr("Show the compact listening indicator above other windows."); color: root.secondaryText; font.pixelSize: 12 }
                            }
                            Switch {
                                checked: controller.overlayVisible
                                onToggled: controller.setOverlayPreview(checked)
                            }
                        }
                    }
                }

                Rectangle {
                    Layout.fillWidth: true
                    height: 102
                    radius: 18
                    color: "#191922"
                    border.color: "#393451"

                    RowLayout {
                        anchors.fill: parent
                        anchors.margins: 18
                        spacing: 16
                        Rectangle {
                            width: 48
                            height: 48
                            radius: 15
                            color: "#302b50"
                            Text { anchors.centerIn: parent; text: "⌁"; color: "#b9aeff"; font.pixelSize: 27 }
                        }
                        ColumnLayout {
                            Layout.fillWidth: true
                            spacing: 3
                            Text { text: controller.recording ? qsTr("Microphone is live") : qsTr("Test microphone capture"); color: root.primaryText; font.pixelSize: 14; font.weight: Font.DemiBold }
                            Text { text: controller.recording ? qsTr("Input level: %1%").arg(Math.round(controller.audioLevel * 100)) : qsTr("Capture audio through PipeWire without transcription."); color: root.secondaryText; font.pixelSize: 12 }
                        }
                        Button {
                            text: controller.recording ? qsTr("Stop recording") : qsTr("Start recording")
                            onClicked: controller.toggleRecording()
                        }
                    }
                }
            }
        }
    }

    Window {
        id: overlay
        width: 354
        height: 82
        visible: controller.overlayVisible
        color: "transparent"
        flags: Qt.Tool | Qt.FramelessWindowHint | Qt.WindowStaysOnTopHint
        title: qsTr("FluidVoice Recording")

        Rectangle {
            anchors.fill: parent
            anchors.margins: 5
            radius: 25
            color: "#eb1e2028"
            border.color: controller.recording ? "#7769d8" : "#3c3d48"
            border.width: 1

            RowLayout {
                anchors.fill: parent
                anchors.leftMargin: 19
                anchors.rightMargin: 17
                spacing: 14

                Rectangle {
                    width: 42
                    height: 42
                    radius: 15
                    color: controller.recording ? root.accent : "#32333c"
                    Text { anchors.centerIn: parent; text: "●"; color: "white"; font.pixelSize: 15 }
                }

                ColumnLayout {
                    Layout.fillWidth: true
                    spacing: 3
                    Text { text: controller.recording ? qsTr("Listening…") : qsTr("Ready to dictate"); color: "#faf8ff"; font.pixelSize: 14; font.weight: Font.DemiBold }
                    Row {
                        spacing: 4
                        Repeater {
                            model: 13
                            Rectangle {
                                required property int index
                                width: 3
                                height: controller.recording ? 8 + ((index * 7) % 17) : 3
                                radius: 2
                                color: controller.recording ? "#a99dff" : "#777681"
                                anchors.verticalCenter: parent.verticalCenter
                                Behavior on height { NumberAnimation { duration: 160 } }
                            }
                        }
                    }
                }

                Text {
                    text: controller.recording ? qsTr("Release to finish") : "Ctrl Alt D"
                    color: "#aaa8b5"
                    font.pixelSize: 10
                }
            }
        }
    }
}
