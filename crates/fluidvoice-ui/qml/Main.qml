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
    visible: false
    title: qsTr("FluidVoice")
    color: "#111216"
    property bool quitting: false
    property int settingsSection: 0
    onClosing: function(close) {
        if (quitting) {
            close.accepted = true
            return
        }
        close.accepted = false
        root.hide()
    }

    readonly property color accent: "#8b7cff"
    readonly property color panel: "#1a1b21"
    readonly property color panelRaised: "#22232b"
    readonly property color primaryText: "#f6f4ff"
    readonly property color secondaryText: "#aaa8b4"

    FluidVoiceController {
        id: controller
    }

    Component.onCompleted: {
        controller.initializeAudio()
        controller.initializeDesktopRuntime()
    }

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
                onTriggered: {
                    root.quitting = true
                    Qt.quit()
                }
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
                        color: index === root.settingsSection ? "#2b2940" : "transparent"

                        Text {
                            anchors.left: parent.left
                            anchors.leftMargin: 15
                            anchors.verticalCenter: parent.verticalCenter
                            text: modelData
                            color: index === root.settingsSection ? "#f4f0ff" : root.secondaryText
                            font.pixelSize: 13
                            font.weight: index === root.settingsSection ? Font.DemiBold : Font.Normal
                        }
                        MouseArea {
                            anchors.fill: parent
                            cursorShape: Qt.PointingHandCursor
                            onClicked: {
                                root.settingsSection = index
                                settingsFlick.contentY = [0, 0, 250, 480, 650][index]
                            }
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
            id: settingsFlick
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
                    height: 196
                    radius: 18
                    color: root.panel
                    border.color: "#2d2e37"

                    ColumnLayout {
                        anchors.fill: parent
                        anchors.margins: 20
                        spacing: 14
                        Text { text: qsTr("INPUT & MODEL"); color: "#777581"; font.pixelSize: 10; font.letterSpacing: 1.2 }

                        ColumnLayout {
                            Layout.fillWidth: true
                            spacing: 6
                            Text { text: qsTr("Microphone source"); color: root.primaryText; font.pixelSize: 13; font.weight: Font.Medium }
                            ComboBox {
                                Layout.fillWidth: true
                                model: controller.inputSources
                                currentIndex: controller.selectedInput
                                enabled: !controller.recording && count > 0
                                onActivated: function(index) { controller.selectInput(index) }
                            }
                            RowLayout {
                                Layout.fillWidth: true
                                Text { text: qsTr("Software gain"); color: root.secondaryText; font.pixelSize: 12 }
                                Slider {
                                    Layout.fillWidth: true
                                    from: -12
                                    to: 24
                                    stepSize: 1
                                    value: controller.gainDb
                                    enabled: !controller.recording
                                    onMoved: controller.gainDb = value
                                }
                                Text {
                                    text: (controller.gainDb >= 0 ? "+" : "") + Math.round(controller.gainDb) + " dB"
                                    color: root.primaryText
                                    font.pixelSize: 11
                                    font.family: "monospace"
                                }
                            }
                        }
                    }
                }

                ColumnLayout {
                    spacing: 5
                    Text {
                        text: qsTr("Transcription")
                        color: root.primaryText
                        font.pixelSize: 22
                        font.weight: Font.DemiBold
                    }
                    Text {
                        text: qsTr("Choose the local speech model and spoken language.")
                        color: root.secondaryText
                        font.pixelSize: 12
                    }
                }

                Rectangle {
                    Layout.fillWidth: true
                    height: 176
                    radius: 18
                    color: root.panel
                    border.color: "#2d2e37"

                    ColumnLayout {
                        anchors.fill: parent
                        anchors.margins: 20
                        spacing: 12
                        Text { text: qsTr("SPEECH ENGINE"); color: "#777581"; font.pixelSize: 10; font.letterSpacing: 1.2 }
                        RowLayout {
                            Layout.fillWidth: true
                            spacing: 14
                            ColumnLayout {
                                Layout.fillWidth: true
                                Text { text: qsTr("Language"); color: root.primaryText; font.pixelSize: 13; font.weight: Font.Medium }
                                ComboBox {
                                    Layout.fillWidth: true
                                    model: controller.languages
                                    currentIndex: controller.selectedLanguage
                                    enabled: !controller.recording && !controller.transcribing
                                    onActivated: function(index) { controller.selectLanguage(index) }
                                }
                            }
                            ColumnLayout {
                                Layout.fillWidth: true
                                Text { text: qsTr("Whisper model"); color: root.primaryText; font.pixelSize: 13; font.weight: Font.Medium }
                                ComboBox {
                                    Layout.fillWidth: true
                                    model: controller.models
                                    currentIndex: controller.selectedModel
                                    enabled: !controller.recording && !controller.transcribing && count > 0
                                    displayText: count > 0 ? currentText : qsTr("No model installed")
                                    onActivated: function(index) { controller.selectModel(index) }
                                }
                            }
                        }
                        Text {
                            text: qsTr("Runs entirely on this computer. Audio and transcripts never leave FluidVoice.")
                            color: "#7fd7a4"
                            font.pixelSize: 11
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
                            ComboBox {
                                Layout.preferredWidth: 190
                                model: controller.shortcuts
                                currentIndex: controller.selectedShortcut
                                enabled: !controller.recording && !controller.transcribing
                                onActivated: function(index) { controller.selectShortcut(index) }
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
                            Text {
                                text: controller.recording
                                      ? qsTr("%1 dBFS · %2 live updates").arg(controller.inputDb.toFixed(1)).arg(controller.audioUpdates)
                                      : qsTr("Press Test input, then speak into the selected microphone.")
                                color: controller.recording && controller.audioUpdates > 0 ? "#82dda9" : root.secondaryText
                                font.pixelSize: 12
                            }
                            Rectangle {
                                Layout.fillWidth: true
                                Layout.maximumWidth: 240
                                height: 6
                                radius: 3
                                color: "#32333d"

                                Rectangle {
                                    width: parent.width * controller.audioLevel
                                    height: parent.height
                                    radius: parent.radius
                                    color: controller.audioLevel > 0.82 ? "#ff8f9c" : root.accent
                                    Behavior on width { NumberAnimation { duration: 55 } }
                                    Behavior on color { ColorAnimation { duration: 100 } }
                                }
                            }
                        }
                        Button {
                            text: controller.recording ? qsTr("Stop test") : qsTr("Test input")
                            enabled: controller.selectedInput >= 0 && !controller.transcribing
                            onClicked: controller.toggleRecording()
                        }
                    }
                }

                Rectangle {
                    Layout.fillWidth: true
                    implicitHeight: transcriptColumn.implicitHeight + 36
                    radius: 18
                    color: root.panel
                    border.color: "#2d2e37"

                    ColumnLayout {
                        id: transcriptColumn
                        anchors.left: parent.left
                        anchors.right: parent.right
                        anchors.top: parent.top
                        anchors.margins: 18
                        spacing: 8

                        Text { text: qsTr("LATEST TRANSCRIPT"); color: "#777581"; font.pixelSize: 10; font.letterSpacing: 1.2 }
                        Text {
                            Layout.fillWidth: true
                            text: controller.transcriptText.length > 0
                                  ? controller.transcriptText
                                  : qsTr("Your local transcript will appear here after stopping the input test.")
                            color: controller.transcriptText.length > 0 ? root.primaryText : root.secondaryText
                            font.pixelSize: 13
                            wrapMode: Text.Wrap
                        }
                    }
                }
            }
        }
    }

    Window {
        id: overlay
        width: 520
        height: 82
        visible: controller.overlayVisible
        color: "transparent"
        transientParent: null
        modality: Qt.NonModal
        flags: Qt.Tool | Qt.FramelessWindowHint | Qt.WindowStaysOnTopHint
               | Qt.WindowDoesNotAcceptFocus
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
                    Text { text: controller.recording ? qsTr("Listening…") : controller.transcribing ? qsTr("Transcribing…") : qsTr("Ready to dictate"); color: "#faf8ff"; font.pixelSize: 14; font.weight: Font.DemiBold }
                    RowLayout {
                        Layout.fillWidth: true
                        spacing: 10
                        Row {
                            spacing: 3
                            Repeater {
                                model: 9
                                Rectangle {
                                    required property int index
                                    readonly property var barShape: [0.35, 0.55, 0.78, 1.0, 0.72, 1.0, 0.78, 0.55, 0.35]
                                    width: 3
                                    height: controller.recording ? 3 + controller.audioLevel * 18 * barShape[index] : 3
                                    radius: 2
                                    color: controller.recording && controller.audioLevel > 0.02 ? "#b5aaff" : "#777681"
                                    anchors.verticalCenter: parent.verticalCenter
                                    Behavior on height { NumberAnimation { duration: 55; easing.type: Easing.OutQuad } }
                                    Behavior on color { ColorAnimation { duration: 100 } }
                                }
                            }
                        }
                        Text {
                            Layout.fillWidth: true
                            text: controller.liveTranscript.length > 0
                                  ? controller.liveTranscript
                                  : qsTr("Speak naturally — text will appear here")
                            color: controller.liveTranscript.length > 0 ? "#d9d5e7" : "#85838e"
                            font.pixelSize: 10
                            elide: Text.ElideLeft
                            maximumLineCount: 1
                        }
                    }
                }

                Text {
                    text: controller.recording ? qsTr("Release to finish") : controller.transcribing ? qsTr("On-device") : "Ctrl Alt D"
                    color: "#aaa8b5"
                    font.pixelSize: 10
                }
            }
        }
    }
}
