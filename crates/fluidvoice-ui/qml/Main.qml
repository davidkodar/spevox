import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtQuick.Window
import QtQuick.Dialogs
import io.github.davidkodar.FluidVoiceLinux

ApplicationWindow {
    id: root
    width: 960
    height: 680
    minimumWidth: 800
    minimumHeight: 500
    visible: true
    title: qsTr("FluidVoice")
    color: "#121212"
    property int settingsSection: 0
    readonly property var destinationTitles: [
        qsTr("Settings"), qsTr("Voice Engine"), qsTr("AI Enhancement"),
        qsTr("Custom Dictionary"), qsTr("Command Mode"), qsTr("File Transcription"),
        qsTr("History"), qsTr("Stats"), qsTr("Getting Started"),
        qsTr("Change logs"), qsTr("Feedback")
    ]
    readonly property var destinationDescriptions: [
        qsTr("Manage FluidVoice behavior, shortcuts, and appearance."),
        qsTr("Choose the microphone, speech model, and spoken language."),
        qsTr("Refine dictated text with an optional AI processing step."),
        qsTr("Teach FluidVoice names, terms, and preferred spellings."),
        qsTr("Run actions and workflows with your voice."),
        qsTr("Create transcripts from existing audio files."),
        qsTr("Review recent dictation and transcription activity."),
        qsTr("See how you use FluidVoice over time."),
        qsTr("Learn the essentials and complete initial setup."),
        qsTr("See what changed in recent versions."),
        qsTr("Share feedback about this unofficial Linux port.")
    ]

    function showSettingsSection(index) {
        settingsSection = index
        settingsFlick.contentY = 0
    }
    onClosing: function(close) {
        close.accepted = false
        root.hide()
    }

    // Mirrors the current FluidVoice dark theme tokens. Qt cannot use SwiftUI's
    // NSVisualEffect materials, so these are deliberately restrained opaque
    // equivalents that remain predictable under Plasma compositing.
    readonly property color accent: "#3ac8c6"
    readonly property color panel: "#151515"
    readonly property color panelRaised: "#1c1c1c"
    readonly property color primaryText: "#f2f2f2"
    readonly property color secondaryText: "#a8a8ad"
    readonly property color tertiaryText: "#737379"
    readonly property color hairline: "#2b2b2e"

    FluidVoiceController {
        id: controller
    }

    FileDialog {
        id: audioFileDialog
        title: qsTr("Choose an audio file")
        nameFilters: [qsTr("PCM WAV audio (*.wav)")]
        onAccepted: controller.transcribeFile(selectedFile.toString())
    }

    Component.onCompleted: {
        controller.initializeAudio()
        controller.initializeDesktopRuntime()
    }

    background: Rectangle {
        color: root.color
        gradient: Gradient {
            GradientStop { position: 0.0; color: "#171717" }
            GradientStop { position: 1.0; color: "#121212" }
        }
    }

    header: Rectangle {
        height: 44
        color: "#0f0f0f"
        border.color: root.hairline
        border.width: 1

        RowLayout {
            anchors.fill: parent
            anchors.leftMargin: 16
            anchors.rightMargin: 16
            spacing: 10

            Text {
                text: root.destinationTitles[root.settingsSection]
                color: root.secondaryText
                font.pixelSize: 13
                font.weight: Font.Medium
            }

            Item { Layout.fillWidth: true }

            Rectangle {
                implicitWidth: statusRow.implicitWidth + 24
                implicitHeight: 28
                radius: 14
                color: controller.recording ? "#173334" : "#1b1b1d"
                border.color: controller.recording ? "#3f7475" : root.hairline

                Row {
                    id: statusRow
                    anchors.centerIn: parent
                    spacing: 8
                    Rectangle {
                        anchors.verticalCenter: parent.verticalCenter
                        width: 8
                        height: 8
                        radius: 4
                        color: root.accent
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
        anchors.margins: 0
        spacing: 0

        Rectangle {
            Layout.preferredWidth: 244
            Layout.fillHeight: true
            color: "#0f0f0f"
            border.color: root.hairline

            ColumnLayout {
                anchors.fill: parent
                anchors.leftMargin: 12
                anchors.rightMargin: 12
                anchors.topMargin: 18
                anchors.bottomMargin: 14
                spacing: 2

                Repeater {
                    model: [
                        { "header": true, "name": qsTr("CONFIGURE") },
                        { "name": qsTr("Settings"), "symbol": "⚙", "page": 0 },
                        { "name": qsTr("Voice Engine"), "symbol": "≋", "page": 1 },
                        { "name": qsTr("AI Enhancement"), "symbol": "✦", "page": 2 },
                        { "name": qsTr("Custom Dictionary"), "symbol": "▤", "page": 3 },
                        { "header": true, "name": qsTr("USE") },
                        { "name": qsTr("Command Mode"), "symbol": ">_", "page": 4 },
                        { "name": qsTr("File Transcription"), "symbol": "▧", "page": 5 },
                        { "header": true, "name": qsTr("ACTIVITY") },
                        { "name": qsTr("History"), "symbol": "↶", "page": 6 },
                        { "name": qsTr("Stats"), "symbol": "▥", "page": 7 },
                        { "header": true, "name": qsTr("HELP") },
                        { "name": qsTr("Getting Started"), "symbol": "⌂", "page": 8 },
                        { "name": qsTr("Change logs"), "symbol": "⌕", "page": 9 },
                        { "name": qsTr("Feedback"), "symbol": "✉", "page": 10 }
                    ]
                    delegate: Rectangle {
                        required property var modelData
                        required property int index
                        Layout.fillWidth: true
                        readonly property bool isHeader: modelData.header === true
                        Layout.topMargin: isHeader && index > 0 ? 9 : 0
                        height: isHeader ? 22 : 34
                        radius: 6
                        color: !isHeader && modelData.page === root.settingsSection ? "#272729" : "transparent"

                        Text {
                            visible: parent.isHeader
                            anchors.left: parent.left
                            anchors.leftMargin: 8
                            anchors.verticalCenter: parent.verticalCenter
                            text: modelData.name
                            color: root.tertiaryText
                            font.pixelSize: 11
                            font.weight: Font.Medium
                        }
                        Row {
                            visible: !parent.isHeader
                            anchors.left: parent.left
                            anchors.leftMargin: 10
                            anchors.verticalCenter: parent.verticalCenter
                            spacing: 10
                            Text {
                                width: 18
                                text: modelData.symbol || ""
                                color: modelData.page === root.settingsSection ? root.primaryText : root.secondaryText
                                font.pixelSize: 14
                                horizontalAlignment: Text.AlignHCenter
                            }
                            Text {
                                text: modelData.name
                                color: modelData.page === root.settingsSection ? root.primaryText : root.secondaryText
                                font.pixelSize: 14
                            }
                        }
                        MouseArea {
                            anchors.fill: parent
                            enabled: !parent.isHeader
                            cursorShape: Qt.PointingHandCursor
                            onClicked: root.showSettingsSection(modelData.page)
                        }
                    }
                }

                Item { Layout.fillHeight: true }

                Text {
                    Layout.alignment: Qt.AlignHCenter
                    text: "Unofficial Linux port · 0.1.0"
                    color: root.tertiaryText
                    font.pixelSize: 11
                }
            }
        }

        Flickable {
            id: settingsFlick
            Layout.fillWidth: true
            Layout.fillHeight: true
            contentHeight: contentColumn.implicitHeight + 48
            clip: true
            ScrollBar.vertical: ScrollBar { policy: ScrollBar.AsNeeded }

            ColumnLayout {
                id: contentColumn
                x: 28
                y: 24
                width: parent.width - 56
                spacing: 16

                ColumnLayout {
                    id: generalSection
                    visible: root.settingsSection === 0
                    spacing: 5
                    Text {
                        text: qsTr("Settings")
                        color: root.primaryText
                        font.pixelSize: 22
                        font.weight: Font.Bold
                    }
                    Text {
                        text: root.destinationDescriptions[0]
                        color: root.secondaryText
                        font.pixelSize: 14
                    }
                }

                Rectangle {
                    visible: root.settingsSection === 0
                    Layout.fillWidth: true
                    height: 112
                    radius: 16
                    color: root.panel
                    border.color: root.hairline

                    ColumnLayout {
                        anchors.fill: parent
                        anchors.margins: 16
                        spacing: 10
                        Text { text: qsTr("APP SETTINGS"); color: root.tertiaryText; font.pixelSize: 11; font.weight: Font.Medium }
                        RowLayout {
                            Layout.fillWidth: true
                            ColumnLayout {
                                Layout.fillWidth: true
                                spacing: 2
                                Text { text: qsTr("Background operation"); color: root.primaryText; font.pixelSize: 14; font.weight: Font.Medium }
                                Text { text: qsTr("FluidVoice stays available in the Plasma system tray when this window is closed."); color: root.secondaryText; font.pixelSize: 13 }
                            }
                            Rectangle {
                                implicitWidth: backgroundStatus.implicitWidth + 18
                                implicitHeight: 26
                                radius: 13
                                color: "#173334"
                                border.color: "#3f7475"
                                Text { id: backgroundStatus; anchors.centerIn: parent; text: qsTr("Active"); color: root.accent; font.pixelSize: 11; font.weight: Font.Medium }
                            }
                        }
                    }
                }

                ColumnLayout {
                    id: audioSection
                    visible: root.settingsSection === 1
                    spacing: 5
                    Text { text: qsTr("Audio"); color: root.primaryText; font.pixelSize: 22; font.weight: Font.Bold }
                    Text { text: qsTr("Choose and calibrate the microphone FluidVoice listens to."); color: root.secondaryText; font.pixelSize: 13 }
                }

                Rectangle {
                    visible: root.settingsSection === 1
                    Layout.fillWidth: true
                    height: 196
                    radius: 16
                    color: root.panel
                    border.color: root.hairline

                    ColumnLayout {
                        anchors.fill: parent
                        anchors.margins: 16
                        spacing: 12
                        Text { text: qsTr("INPUT & MODEL"); color: root.tertiaryText; font.pixelSize: 11; font.weight: Font.Medium }

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
                                    onMoved: controller.updateGainDb(value)
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
                    id: transcriptionSection
                    visible: root.settingsSection === 1
                    spacing: 5
                    Text {
                        text: qsTr("Transcription")
                        color: root.primaryText
                        font.pixelSize: 22
                        font.weight: Font.Bold
                    }
                    Text {
                        text: qsTr("Choose the local speech model and spoken language.")
                        color: root.secondaryText
                        font.pixelSize: 13
                    }
                }

                Rectangle {
                    visible: root.settingsSection === 1
                    Layout.fillWidth: true
                    implicitHeight: speechEngineColumn.implicitHeight + 32
                    radius: 16
                    color: root.panel
                    border.color: root.hairline

                    ColumnLayout {
                        id: speechEngineColumn
                        anchors.fill: parent
                        anchors.margins: 16
                        spacing: 12
                        Text { text: qsTr("SPEECH ENGINE"); color: root.tertiaryText; font.pixelSize: 11; font.weight: Font.Medium }

                        ColumnLayout {
                            Layout.fillWidth: true
                            spacing: 5
                            Text { text: qsTr("Language"); color: root.primaryText; font.pixelSize: 14; font.weight: Font.Medium }
                            ComboBox {
                                Layout.fillWidth: true
                                model: controller.languages
                                currentIndex: controller.selectedLanguage
                                enabled: !controller.recording && !controller.transcribing
                                onActivated: function(index) { controller.selectLanguage(index) }
                            }
                        }

                        RowLayout {
                            Layout.fillWidth: true
                            ColumnLayout {
                                Layout.fillWidth: true
                                spacing: 3
                                Text { text: qsTr("Compute backend"); color: root.primaryText; font.pixelSize: 14; font.weight: Font.Medium }
                                Text {
                                    text: controller.selectedComputeBackend === 2
                                          ? qsTr("Force CPU inference")
                                          : qsTr("Vulkan acceleration with safe CPU fallback")
                                    color: root.secondaryText
                                    font.pixelSize: 12
                                }
                            }
                            ComboBox {
                                Layout.preferredWidth: 210
                                model: controller.computeBackends
                                currentIndex: controller.selectedComputeBackend
                                enabled: !controller.recording && !controller.transcribing
                                onActivated: function(index) { controller.selectComputeBackend(index) }
                            }
                        }

                        Text { text: qsTr("Whisper models"); color: root.primaryText; font.pixelSize: 14; font.weight: Font.Medium }

                        ColumnLayout {
                            id: modelList
                            Layout.fillWidth: true
                            spacing: 8
                            Repeater {
                                model: controller.models
                                delegate: Rectangle {
                                    required property string modelData
                                    required property int index
                                    Layout.fillWidth: true
                                    implicitHeight: 72
                                    radius: 10
                                    color: index === controller.selectedModel ? "#18292a" : "#19191b"
                                    border.color: index === controller.selectedModel ? "#487778" : root.hairline

                                    Item {
                                        anchors.fill: parent
                                        anchors.margins: 12
                                        Rectangle {
                                            id: modelStatusDot
                                            anchors.left: parent.left
                                            anchors.verticalCenter: parent.verticalCenter
                                            width: 8
                                            height: 8
                                            radius: 4
                                            color: index === controller.selectedModel ? root.accent : "#55555a"
                                        }

                                        Item {
                                            id: modelTextArea
                                            anchors.left: modelStatusDot.right
                                            anchors.leftMargin: 10
                                            anchors.right: modelActions.left
                                            anchors.rightMargin: 10
                                            anchors.top: parent.top
                                            anchors.bottom: parent.bottom
                                            clip: true

                                            ColumnLayout {
                                                anchors.fill: parent
                                                spacing: 3
                                                RowLayout {
                                                    Layout.fillWidth: true
                                                    Text { text: modelData; color: root.primaryText; font.pixelSize: 13; font.weight: Font.Medium }
                                                    Text {
                                                        text: controller.modelStates[index] || ""
                                                        color: controller.modelStates[index] === "Downloaded" ? root.accent : root.tertiaryText
                                                        font.pixelSize: 11
                                                    }
                                                }
                                                Text {
                                                    Layout.fillWidth: true
                                                    text: controller.modelDetails[index] || ""
                                                    color: root.secondaryText
                                                    font.pixelSize: 11
                                                    elide: Text.ElideRight
                                                }
                                                ProgressBar {
                                                    Layout.fillWidth: true
                                                    visible: controller.downloadingModel === index
                                                    value: controller.modelDownloadProgress
                                                }
                                            }
                                        }

                                        Item {
                                            id: modelActions
                                            anchors.right: parent.right
                                            anchors.top: parent.top
                                            anchors.bottom: parent.bottom
                                            width: 186

                                            StackLayout {
                                                anchors.left: parent.left
                                                anchors.verticalCenter: parent.verticalCenter
                                                width: 134
                                                height: 40
                                                currentIndex: controller.modelStates[index] === "Downloaded"
                                                              && index === controller.selectedModel ? 1 : 0

                                                Button {
                                                    Layout.fillWidth: true
                                                    Layout.fillHeight: true
                                                    text: controller.downloadingModel === index ? qsTr("Cancel")
                                                          : controller.modelStates[index] !== "Downloaded" ? qsTr("Download")
                                                          : qsTr("Activate")
                                                    enabled: !controller.recording && !controller.transcribing
                                                             && (controller.downloadingModel < 0 || controller.downloadingModel === index)
                                                    onClicked: {
                                                        if (controller.downloadingModel === index)
                                                            controller.cancelModelDownload()
                                                        else if (controller.modelStates[index] !== "Downloaded")
                                                            controller.downloadModel(index)
                                                        else
                                                            controller.selectModel(index)
                                                    }
                                                }
                                                Item {
                                                    Layout.fillWidth: true
                                                    Layout.fillHeight: true
                                                    Rectangle {
                                                        anchors.centerIn: parent
                                                        width: activeLabel.implicitWidth + 22
                                                        height: 30
                                                        radius: 15
                                                        color: "#234b3b"
                                                        border.color: "#39755a"
                                                        Text {
                                                            id: activeLabel
                                                            anchors.centerIn: parent
                                                            text: qsTr("Active")
                                                            color: "#70d59b"
                                                            font.pixelSize: 12
                                                            font.weight: Font.DemiBold
                                                        }
                                                    }
                                                }
                                            }
                                            ToolButton {
                                                anchors.right: parent.right
                                                anchors.verticalCenter: parent.verticalCenter
                                                width: 42
                                                height: 38
                                                visible: controller.modelStates[index] === "Downloaded" && index !== controller.selectedModel
                                                enabled: controller.downloadingModel < 0 && !controller.recording && !controller.transcribing
                                                icon.source: "qrc:/qt/qml/io/github/davidkodar/FluidVoiceLinux/assets/trash.svg"
                                                icon.width: 20
                                                icon.height: 20
                                                display: AbstractButton.IconOnly
                                                ToolTip.visible: hovered
                                                ToolTip.text: qsTr("Delete downloaded model")
                                                onClicked: controller.deleteModel(index)
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Text {
                            text: qsTr("One multilingual model works for every listed language. Downloads are stored locally and audio never leaves this computer.")
                            color: root.accent
                            font.pixelSize: 11
                            wrapMode: Text.Wrap
                            Layout.fillWidth: true
                        }
                    }
                }

                ColumnLayout {
                    id: shortcutsSection
                    visible: root.settingsSection === 0
                    spacing: 5
                    Text { text: qsTr("Shortcuts"); color: root.primaryText; font.pixelSize: 22; font.weight: Font.Bold }
                    Text { text: qsTr("Configure how dictation starts and what appears while you speak."); color: root.secondaryText; font.pixelSize: 13 }
                }

                Rectangle {
                    visible: root.settingsSection === 0
                    Layout.fillWidth: true
                    height: 180
                    radius: 16
                    color: root.panel
                    border.color: root.hairline

                    ColumnLayout {
                        anchors.fill: parent
                        anchors.margins: 16
                        spacing: 14
                        Text { text: qsTr("DICTATION"); color: root.tertiaryText; font.pixelSize: 11; font.weight: Font.Medium }

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

                        Rectangle { Layout.fillWidth: true; height: 1; color: root.hairline }

                        RowLayout {
                            Layout.fillWidth: true
                            ColumnLayout {
                                Layout.fillWidth: true
                                spacing: 4
                                Text { text: qsTr("Recording overlay"); color: root.primaryText; font.pixelSize: 14; font.weight: Font.Medium }
                                Text { text: qsTr("Show the compact listening indicator above other windows."); color: root.secondaryText; font.pixelSize: 12 }
                            }
                            Switch {
                                checked: controller.overlayEnabled
                                onToggled: controller.updateOverlayEnabled(checked)
                            }
                        }
                    }
                }

                Rectangle {
                    visible: root.settingsSection === 1
                    Layout.fillWidth: true
                    height: 102
                    radius: 16
                    color: root.panelRaised
                    border.color: "#315152"

                    RowLayout {
                        anchors.fill: parent
                        anchors.margins: 18
                        spacing: 16
                        Rectangle {
                            width: 48
                            height: 48
                            radius: 12
                            color: "#193536"
                            Text { anchors.centerIn: parent; text: "⌁"; color: root.accent; font.pixelSize: 27 }
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
                    visible: root.settingsSection === 1
                    Layout.fillWidth: true
                    implicitHeight: transcriptColumn.implicitHeight + 36
                    radius: 16
                    color: root.panel
                    border.color: root.hairline

                    ColumnLayout {
                        id: transcriptColumn
                        anchors.left: parent.left
                        anchors.right: parent.right
                        anchors.top: parent.top
                        anchors.margins: 18
                        spacing: 8

                        Text { text: qsTr("LATEST TRANSCRIPT"); color: root.tertiaryText; font.pixelSize: 11; font.weight: Font.Medium }
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

                ColumnLayout {
                    id: appearanceSection
                    visible: root.settingsSection === 0
                    spacing: 5
                    Text { text: qsTr("Appearance"); color: root.primaryText; font.pixelSize: 22; font.weight: Font.Bold }
                    Text { text: qsTr("A native KDE interpretation of FluidVoice's macOS visual language."); color: root.secondaryText; font.pixelSize: 13 }
                }

                Rectangle {
                    visible: root.settingsSection === 0
                    Layout.fillWidth: true
                    height: 158
                    radius: 16
                    color: root.panel
                    border.color: root.hairline

                    ColumnLayout {
                        anchors.fill: parent
                        anchors.margins: 16
                        spacing: 12
                        Text { text: qsTr("INTERFACE"); color: root.tertiaryText; font.pixelSize: 11; font.weight: Font.Medium }
                        RowLayout {
                            Layout.fillWidth: true
                            ColumnLayout {
                                Layout.fillWidth: true
                                spacing: 2
                                Text { text: qsTr("Theme"); color: root.primaryText; font.pixelSize: 14; font.weight: Font.Medium }
                                Text { text: qsTr("Dark appearance optimized for Plasma and the upstream FluidVoice design."); color: root.secondaryText; font.pixelSize: 13 }
                            }
                            Text { text: qsTr("Dark"); color: root.secondaryText; font.pixelSize: 13 }
                        }
                        Rectangle { Layout.fillWidth: true; height: 1; color: root.hairline }
                        RowLayout {
                            Layout.fillWidth: true
                            ColumnLayout {
                                Layout.fillWidth: true
                                spacing: 2
                                Text { text: qsTr("Accent color"); color: root.primaryText; font.pixelSize: 14; font.weight: Font.Medium }
                                Text { text: qsTr("Uses the current FluidVoice default accent."); color: root.secondaryText; font.pixelSize: 13 }
                            }
                            Rectangle { width: 18; height: 18; radius: 9; color: root.accent; border.color: "#66ffffff" }
                            Text { text: qsTr("Cyan"); color: root.secondaryText; font.pixelSize: 13 }
                        }
                    }
                }

                ColumnLayout {
                    visible: root.settingsSection === 2
                    spacing: 5
                    Text {
                        text: root.destinationTitles[root.settingsSection]
                        color: root.primaryText
                        font.pixelSize: 22
                        font.weight: Font.Bold
                    }
                    Text {
                        text: root.destinationDescriptions[root.settingsSection]
                        color: root.secondaryText
                        font.pixelSize: 14
                    }
                }

                Rectangle {
                    visible: root.settingsSection === 2
                    Layout.fillWidth: true
                    implicitHeight: unavailableContent.implicitHeight + 48
                    radius: 16
                    color: root.panel
                    border.color: root.hairline

                    ColumnLayout {
                        id: unavailableContent
                        anchors.left: parent.left
                        anchors.right: parent.right
                        anchors.top: parent.top
                        anchors.margins: 24
                        spacing: 12
                        Rectangle {
                            width: 44
                            height: 44
                            radius: 12
                            color: root.panelRaised
                            border.color: root.hairline
                            Text {
                                anchors.centerIn: parent
                            text: "✦"
                                color: root.secondaryText
                                font.pixelSize: 20
                            }
                        }
                        Text {
                            text: qsTr("Not implemented yet")
                            color: root.primaryText
                            font.pixelSize: 15
                            font.weight: Font.DemiBold
                        }
                        Text {
                            Layout.fillWidth: true
                            text: qsTr("AI Enhancement is shown to match FluidVoice's upstream navigation, but no AI provider or text-processing service is connected in this Linux port yet. Local Whisper transcription continues to work without it.")
                            color: root.secondaryText
                            font.pixelSize: 13
                            wrapMode: Text.Wrap
                        }
                    }
                }

                ColumnLayout {
                    visible: root.settingsSection === 3
                    spacing: 14
                    Text { text: qsTr("Custom Dictionary"); color: root.primaryText; font.pixelSize: 22; font.weight: Font.Bold }
                    Text { text: root.destinationDescriptions[3]; color: root.secondaryText; font.pixelSize: 14 }
                    Rectangle {
                        Layout.fillWidth: true; implicitHeight: dictionaryContent.implicitHeight + 32; radius: 16
                        color: root.panel; border.color: root.hairline
                        ColumnLayout {
                            id: dictionaryContent; anchors.fill: parent; anchors.margins: 16; spacing: 12
                            Text { text: qsTr("PREFERRED TERMS"); color: root.tertiaryText; font.pixelSize: 11; font.weight: Font.Medium }
                            RowLayout {
                                Layout.fillWidth: true
                                TextField { id: dictionaryInput; Layout.fillWidth: true; placeholderText: qsTr("Add a name, acronym, or preferred spelling"); onAccepted: addDictionaryButton.clicked() }
                                Button { id: addDictionaryButton; text: qsTr("Add"); enabled: dictionaryInput.text.trim().length > 0; onClicked: { controller.addDictionaryTerm(dictionaryInput.text); dictionaryInput.clear() } }
                            }
                            Text { visible: controller.dictionaryTerms.length === 0; text: qsTr("No custom terms yet."); color: root.secondaryText; font.pixelSize: 13 }
                            Repeater {
                                model: controller.dictionaryTerms
                                delegate: RowLayout {
                                    required property string modelData; required property int index; Layout.fillWidth: true
                                    Text { Layout.fillWidth: true; text: modelData; color: root.primaryText; font.pixelSize: 13 }
                                    ToolButton { text: "×"; ToolTip.visible: hovered; ToolTip.text: qsTr("Remove"); onClicked: controller.removeDictionaryTerm(index) }
                                }
                            }
                            Text { Layout.fillWidth: true; text: qsTr("Matching words in new transcripts are rewritten with this exact capitalization."); color: root.secondaryText; font.pixelSize: 12; wrapMode: Text.Wrap }
                        }
                    }
                }

                ColumnLayout {
                    visible: root.settingsSection === 4
                    spacing: 14
                    Text { text: qsTr("Command Mode"); color: root.primaryText; font.pixelSize: 22; font.weight: Font.Bold }
                    Text { text: root.destinationDescriptions[4]; color: root.secondaryText; font.pixelSize: 14 }
                    Rectangle {
                        Layout.fillWidth: true; implicitHeight: commandContent.implicitHeight + 32; radius: 16; color: root.panel; border.color: root.hairline
                        ColumnLayout {
                            id: commandContent; anchors.fill: parent; anchors.margins: 16; spacing: 14
                            RowLayout {
                                Layout.fillWidth: true
                                ColumnLayout { Layout.fillWidth: true; spacing: 3
                                    Text { text: qsTr("Spoken formatting commands"); color: root.primaryText; font.pixelSize: 14; font.weight: Font.Medium }
                                    Text { text: qsTr("Convert recognized commands after local transcription."); color: root.secondaryText; font.pixelSize: 12 }
                                }
                                Switch { checked: controller.commandModeEnabled; onToggled: controller.updateCommandModeEnabled(checked) }
                            }
                            Rectangle { Layout.fillWidth: true; height: 1; color: root.hairline }
                            Text { text: qsTr("AVAILABLE COMMANDS"); color: root.tertiaryText; font.pixelSize: 11; font.weight: Font.Medium }
                            Text { Layout.fillWidth: true; text: qsTr("“new line”  “new paragraph”  “comma”  “period”  “question mark”  “exclamation mark”"); color: root.secondaryText; font.pixelSize: 13; wrapMode: Text.Wrap }
                        }
                    }
                }

                ColumnLayout {
                    visible: root.settingsSection === 5
                    spacing: 14
                    Text { text: qsTr("File Transcription"); color: root.primaryText; font.pixelSize: 22; font.weight: Font.Bold }
                    Text { text: root.destinationDescriptions[5]; color: root.secondaryText; font.pixelSize: 14 }
                    Rectangle {
                        Layout.fillWidth: true; implicitHeight: fileContent.implicitHeight + 40; radius: 16; color: root.panel; border.color: root.hairline
                        ColumnLayout {
                            id: fileContent; anchors.fill: parent; anchors.margins: 20; spacing: 12
                            Text { text: qsTr("TRANSCRIBE AUDIO"); color: root.tertiaryText; font.pixelSize: 11; font.weight: Font.Medium }
                            Text { Layout.fillWidth: true; text: qsTr("Uses the active Whisper model and language. Audio remains on this computer."); color: root.secondaryText; font.pixelSize: 13; wrapMode: Text.Wrap }
                            Button { text: controller.transcribing ? qsTr("Transcribing…") : qsTr("Choose WAV file"); enabled: !controller.transcribing && !controller.recording; onClicked: audioFileDialog.open() }
                            Text { Layout.fillWidth: true; text: controller.fileTranscriptionStatus; color: controller.transcribing ? root.accent : root.secondaryText; font.pixelSize: 12; wrapMode: Text.Wrap }
                        }
                    }
                }

                ColumnLayout {
                    visible: root.settingsSection === 6
                    spacing: 14
                    RowLayout {
                        Layout.fillWidth: true
                        Text { text: qsTr("History"); color: root.primaryText; font.pixelSize: 22; font.weight: Font.Bold }
                        Item { Layout.fillWidth: true }
                        Button { text: qsTr("Clear history"); enabled: controller.historyEntries.length > 0; onClicked: controller.clearHistory() }
                    }
                    Text { text: root.destinationDescriptions[6]; color: root.secondaryText; font.pixelSize: 14 }
                    Text { visible: controller.historyEntries.length === 0; text: qsTr("No transcripts yet. Completed dictation and file transcripts appear here."); color: root.secondaryText; font.pixelSize: 13 }
                    Repeater {
                        model: controller.historyEntries
                        delegate: Rectangle {
                            required property string modelData; Layout.fillWidth: true; implicitHeight: historyText.implicitHeight + 28; radius: 10; color: root.panel; border.color: root.hairline
                            Text { id: historyText; anchors.fill: parent; anchors.margins: 14; text: modelData.indexOf("\t") >= 0 ? modelData.substring(modelData.indexOf("\t") + 1) : modelData; color: root.primaryText; font.pixelSize: 13; wrapMode: Text.Wrap }
                        }
                    }
                }

                ColumnLayout {
                    visible: root.settingsSection === 7
                    spacing: 14
                    Text { text: qsTr("Stats"); color: root.primaryText; font.pixelSize: 22; font.weight: Font.Bold }
                    Text { text: root.destinationDescriptions[7]; color: root.secondaryText; font.pixelSize: 14 }
                    RowLayout {
                        Layout.fillWidth: true; spacing: 12
                        Rectangle { Layout.fillWidth: true; height: 120; radius: 16; color: root.panel; border.color: root.hairline
                            Column { anchors.centerIn: parent; spacing: 6; Text { anchors.horizontalCenter: parent.horizontalCenter; text: controller.transcriptCount; color: root.primaryText; font.pixelSize: 30; font.weight: Font.Bold } Text { anchors.horizontalCenter: parent.horizontalCenter; text: qsTr("Transcripts"); color: root.secondaryText; font.pixelSize: 13 } }
                        }
                        Rectangle { Layout.fillWidth: true; height: 120; radius: 16; color: root.panel; border.color: root.hairline
                            Column { anchors.centerIn: parent; spacing: 6; Text { anchors.horizontalCenter: parent.horizontalCenter; text: controller.dictatedWordCount; color: root.primaryText; font.pixelSize: 30; font.weight: Font.Bold } Text { anchors.horizontalCenter: parent.horizontalCenter; text: qsTr("Words processed"); color: root.secondaryText; font.pixelSize: 13 } }
                        }
                    }
                    Text { text: qsTr("Statistics are derived locally from History and never leave this computer."); color: root.secondaryText; font.pixelSize: 12 }
                }

                ColumnLayout {
                    visible: root.settingsSection === 8
                    spacing: 14
                    Text { text: qsTr("Getting Started"); color: root.primaryText; font.pixelSize: 22; font.weight: Font.Bold }
                    Text { text: root.destinationDescriptions[8]; color: root.secondaryText; font.pixelSize: 14 }
                    Repeater {
                        model: [
                            { "title": qsTr("Choose a microphone"), "detail": controller.selectedInput >= 0 ? controller.microphoneName : qsTr("Open Voice Engine and select an input"), "done": controller.selectedInput >= 0 },
                            { "title": qsTr("Install a speech model"), "detail": controller.modelName, "done": controller.modelStates[controller.selectedModel] === "Downloaded" },
                            { "title": qsTr("Test the global shortcut"), "detail": qsTr("Hold %1 and speak").arg(controller.shortcuts[controller.selectedShortcut]), "done": controller.transcriptCount > 0 }
                        ]
                        delegate: Rectangle {
                            required property var modelData; Layout.fillWidth: true; height: 72; radius: 10; color: root.panel; border.color: root.hairline
                            RowLayout { anchors.fill: parent; anchors.margins: 14; spacing: 12
                                Rectangle { width: 26; height: 26; radius: 13; color: modelData.done ? "#234b3b" : root.panelRaised; border.color: modelData.done ? "#39755a" : root.hairline; Text { anchors.centerIn: parent; text: modelData.done ? "✓" : "·"; color: modelData.done ? "#70d59b" : root.secondaryText; font.pixelSize: 13 } }
                                ColumnLayout { Layout.fillWidth: true; spacing: 2; Text { text: modelData.title; color: root.primaryText; font.pixelSize: 14; font.weight: Font.Medium } Text { text: modelData.detail; color: root.secondaryText; font.pixelSize: 12 } }
                            }
                        }
                    }
                }

                ColumnLayout {
                    visible: root.settingsSection === 9
                    spacing: 14
                    Text { text: qsTr("Change logs"); color: root.primaryText; font.pixelSize: 22; font.weight: Font.Bold }
                    Text { text: root.destinationDescriptions[9]; color: root.secondaryText; font.pixelSize: 14 }
                    Rectangle { Layout.fillWidth: true; implicitHeight: changeContent.implicitHeight + 32; radius: 16; color: root.panel; border.color: root.hairline
                        ColumnLayout { id: changeContent; anchors.fill: parent; anchors.margins: 16; spacing: 10
                            Text { text: qsTr("0.1.0 · Private preview"); color: root.primaryText; font.pixelSize: 15; font.weight: Font.DemiBold }
                            Text { Layout.fillWidth: true; text: qsTr("Native KDE Wayland dictation, PipeWire microphone selection, local Whisper models, language selection, live overlay, model management, custom dictionary, formatting commands, WAV transcription, history, and statistics."); color: root.secondaryText; font.pixelSize: 13; wrapMode: Text.Wrap }
                        }
                    }
                }

                ColumnLayout {
                    visible: root.settingsSection === 10
                    spacing: 14
                    Text { text: qsTr("Feedback"); color: root.primaryText; font.pixelSize: 22; font.weight: Font.Bold }
                    Text { text: root.destinationDescriptions[10]; color: root.secondaryText; font.pixelSize: 14 }
                    Rectangle { Layout.fillWidth: true; implicitHeight: feedbackContent.implicitHeight + 40; radius: 16; color: root.panel; border.color: root.hairline
                        ColumnLayout { id: feedbackContent; anchors.fill: parent; anchors.margins: 20; spacing: 12
                            Text { text: qsTr("HELP IMPROVE FLUIDVOICE LINUX"); color: root.tertiaryText; font.pixelSize: 11; font.weight: Font.Medium }
                            Text { Layout.fillWidth: true; text: qsTr("Report a bug or share an idea through GitHub. The browser opens outside FluidVoice; nothing is submitted automatically."); color: root.secondaryText; font.pixelSize: 13; wrapMode: Text.Wrap }
                            Button { text: qsTr("Open GitHub issues"); onClicked: Qt.openUrlExternally("https://github.com/davidkodar/fluidvoice-linux/issues") }
                        }
                    }
                }
            }
        }
    }

    Window {
        id: overlay
        width: 380
        height: 156
        visible: controller.overlayVisible
        color: "transparent"
        transientParent: null
        modality: Qt.NonModal
        flags: Qt.Tool | Qt.FramelessWindowHint | Qt.WindowStaysOnTopHint
               | Qt.WindowDoesNotAcceptFocus
        title: qsTr("FluidVoice Recording")
        property string animatedTranscript: ""
        property string targetTranscript: ""

        function commonPrefixLength(left, right) {
            var limit = Math.min(left.length, right.length)
            var index = 0
            while (index < limit && left[index] === right[index])
                index++
            return index
        }

        function animateToward(text) {
            targetTranscript = text
            if (text.length === 0) {
                animatedTranscript = ""
                revealTimer.stop()
                return
            }
            var shared = commonPrefixLength(animatedTranscript, text)
            animatedTranscript = animatedTranscript.substring(0, shared)
            revealTimer.restart()
        }

        Connections {
            target: controller
            function onLiveTranscriptChanged() {
                overlay.animateToward(controller.liveTranscript)
            }
        }

        Timer {
            id: revealTimer
            interval: 22
            repeat: true
            onTriggered: {
                if (overlay.animatedTranscript === overlay.targetTranscript) {
                    stop()
                    return
                }
                var remaining = overlay.targetTranscript.length - overlay.animatedTranscript.length
                var step = remaining > 36 ? 4 : remaining > 16 ? 2 : 1
                overlay.animatedTranscript = overlay.targetTranscript.substring(
                    0, overlay.animatedTranscript.length + step)
            }
        }

        Rectangle {
            anchors.fill: parent
            anchors.margins: 8
            radius: 18
            color: "#fa000000"
            border.color: controller.recording ? "#8063d391" : "#32ffffff"
            border.width: 1

            ColumnLayout {
                anchors.fill: parent
                anchors.margins: 16
                spacing: 10

                RowLayout {
                    Layout.fillWidth: true
                    Text {
                        text: controller.recording ? qsTr("Dictate") : controller.transcribing ? qsTr("Processing") : qsTr("FluidVoice")
                        color: controller.recording ? root.accent : "#f2f2f2"
                        font.pixelSize: 13
                        font.weight: Font.DemiBold
                    }
                    Item { Layout.fillWidth: true }
                    Text {
                        text: controller.recording ? qsTr("Release to finish") : controller.transcribing ? qsTr("On-device") : "Ctrl Alt D"
                        color: "#8d8d92"
                        font.pixelSize: 11
                    }
                }

                Text {
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    text: overlay.animatedTranscript.length > 0
                          ? overlay.animatedTranscript
                          : qsTr("Speak naturally — text will appear here")
                    color: overlay.animatedTranscript.length > 0 ? "#eeeeef" : "#77777c"
                    font.pixelSize: 13
                    lineHeight: 1.15
                    wrapMode: Text.Wrap
                    elide: Text.ElideLeft
                    maximumLineCount: 3
                    verticalAlignment: Text.AlignVCenter
                }

                RowLayout {
                    Layout.fillWidth: true
                    spacing: 10
                    Image {
                        Layout.preferredWidth: 22
                        Layout.preferredHeight: 22
                        Layout.minimumWidth: 22
                        Layout.minimumHeight: 22
                        Layout.maximumWidth: 22
                        Layout.maximumHeight: 22
                        source: "qrc:/qt/qml/io/github/davidkodar/FluidVoiceLinux/assets/fluidvoice-app.png"
                        sourceSize.width: 44
                        sourceSize.height: 44
                        fillMode: Image.PreserveAspectFit
                        smooth: true
                    }
                    Item { Layout.fillWidth: true }
                    Row {
                        spacing: 4
                        Repeater {
                            model: 9
                            Rectangle {
                                required property int index
                                readonly property var barShape: [0.35, 0.55, 0.78, 1.0, 0.72, 1.0, 0.78, 0.55, 0.35]
                                width: 3
                                height: controller.recording ? 3 + controller.audioLevel * 18 * barShape[index] : 3
                                radius: 2
                                color: controller.recording && controller.audioLevel > 0.02 ? root.accent : "#5f5f64"
                                anchors.verticalCenter: parent.verticalCenter
                                Behavior on height { NumberAnimation { duration: 55; easing.type: Easing.OutQuad } }
                            }
                        }
                    }
                    Item { Layout.fillWidth: true }
                    Text {
                        text: controller.recording ? qsTr("Listening") : controller.transcribing ? qsTr("Transcribing") : qsTr("Ready")
                        color: "#8d8d92"
                        font.pixelSize: 11
                    }
                }
            }
        }
    }
}
