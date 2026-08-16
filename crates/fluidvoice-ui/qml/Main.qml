import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtQuick.Window
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

    function showSettingsSection(index) {
        var target = generalSection
        if (index === 1)
            target = audioSection
        else if (index === 2)
            target = transcriptionSection
        else if (index === 3)
            target = shortcutsSection
        else if (index === 4)
            target = appearanceSection
        settingsSection = index
        settingsFlick.contentY = Math.max(0, Math.min(
            target.y - 8,
            settingsFlick.contentHeight - settingsFlick.height))
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
        height: 52
        color: "#0f0f0f"
        border.color: root.hairline
        border.width: 1

        RowLayout {
            anchors.fill: parent
            anchors.leftMargin: 20
            anchors.rightMargin: 16
            spacing: 10

            Image {
                Layout.preferredWidth: 28
                Layout.preferredHeight: 28
                Layout.minimumWidth: 28
                Layout.minimumHeight: 28
                Layout.maximumWidth: 28
                Layout.maximumHeight: 28
                source: "qrc:/qt/qml/io/github/davidkodar/FluidVoiceLinux/assets/fluidvoice-app.png"
                sourceSize.width: 56
                sourceSize.height: 56
                fillMode: Image.PreserveAspectFit
                smooth: true
                mipmap: true
            }

            Text {
                text: "FluidVoice"
                color: root.primaryText
                font.pixelSize: 15
                font.weight: Font.DemiBold
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
            Layout.preferredWidth: 220
            Layout.fillHeight: true
            color: "#0f0f0f"
            border.color: root.hairline

            ColumnLayout {
                anchors.fill: parent
                anchors.leftMargin: 12
                anchors.rightMargin: 12
                anchors.topMargin: 18
                anchors.bottomMargin: 14
                spacing: 4

                Text {
                    text: qsTr("CONFIGURE")
                    color: root.tertiaryText
                    font.pixelSize: 11
                    font.weight: Font.Medium
                    Layout.leftMargin: 8
                    Layout.bottomMargin: 4
                }

                Repeater {
                    model: ["General", "Audio", "Transcription", "Shortcuts", "Appearance"]
                    delegate: Rectangle {
                        required property string modelData
                        required property int index
                        Layout.fillWidth: true
                        height: 32
                        radius: 6
                        color: index === root.settingsSection ? "#213738" : "transparent"

                        Text {
                            anchors.left: parent.left
                            anchors.leftMargin: 10
                            anchors.verticalCenter: parent.verticalCenter
                            text: modelData
                            color: index === root.settingsSection ? root.primaryText : root.secondaryText
                            font.pixelSize: 14
                            font.weight: Font.Normal
                        }
                        MouseArea {
                            anchors.fill: parent
                            cursorShape: Qt.PointingHandCursor
                            onClicked: root.showSettingsSection(index)
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
            // The trailing breathing room lets the last sidebar destination align
            // at the same top position as the earlier sections.
            contentHeight: contentColumn.implicitHeight + Math.max(52, height - 120)
            clip: true

            ColumnLayout {
                id: contentColumn
                x: 28
                y: 24
                width: parent.width - 56
                spacing: 16

                ColumnLayout {
                    id: generalSection
                    spacing: 5
                    Text {
                        text: qsTr("General")
                        color: root.primaryText
                        font.pixelSize: 22
                        font.weight: Font.Bold
                    }
                    Text {
                        text: qsTr("Choose how FluidVoice listens and responds.")
                        color: root.secondaryText
                        font.pixelSize: 14
                    }
                }

                Rectangle {
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
                    spacing: 5
                    Text { text: qsTr("Audio"); color: root.primaryText; font.pixelSize: 22; font.weight: Font.Bold }
                    Text { text: qsTr("Choose and calibrate the microphone FluidVoice listens to."); color: root.secondaryText; font.pixelSize: 13 }
                }

                Rectangle {
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
                    spacing: 5
                    Text { text: qsTr("Shortcuts"); color: root.primaryText; font.pixelSize: 22; font.weight: Font.Bold }
                    Text { text: qsTr("Configure how dictation starts and what appears while you speak."); color: root.secondaryText; font.pixelSize: 13 }
                }

                Rectangle {
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
                    spacing: 5
                    Text { text: qsTr("Appearance"); color: root.primaryText; font.pixelSize: 22; font.weight: Font.Bold }
                    Text { text: qsTr("A native KDE interpretation of FluidVoice's macOS visual language."); color: root.secondaryText; font.pixelSize: 13 }
                }

                Rectangle {
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
