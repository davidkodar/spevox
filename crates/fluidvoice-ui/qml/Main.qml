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
    color: root.windowBackground
    property int settingsSection: 0
    property int statsChartDays: 7
    property int onboardingStep: 0
    property var cachedAiHistoryStats: ({ "total": 0, "enhanced": 0, "fallback": 0, "attempts": 0, "latencyTotal": 0, "latencyCount": 0, "providers": ({}) })
    property var cachedFilteredHistory: []

    SystemPalette {
        id: systemPalette
        colorGroup: SystemPalette.Active
    }
    readonly property var destinationTitles: [
        qsTr("Settings"), qsTr("Voice Engine"), qsTr("AI Enhancement"),
        qsTr("Custom Dictionary"), qsTr("Command Mode"), qsTr("File Transcription"),
        qsTr("History"), qsTr("Stats"), qsTr("Getting Started"),
        qsTr("Change logs"), qsTr("Feedback"), qsTr("About & License")
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
        qsTr("Share feedback about this unofficial Linux port."),
        qsTr("View authorship, upstream credit, warranty, and license information.")
    ]

    function showSettingsSection(index) {
        settingsSection = index
        settingsFlick.contentY = 0
    }
    function historyTimestamp(entry) {
        var separator = entry.indexOf("\t")
        if (separator < 0)
            return 0
        var seconds = Number(entry.substring(0, separator))
        return isFinite(seconds) ? seconds : 0
    }
    function historyText(entry) {
        var fields = entry.split("\t")
        return fields.length > 1 ? fields[1] : entry
    }
    function historyRawText(entry) {
        var fields = entry.split("\t")
        return fields.length > 2 ? fields[2] : historyText(entry)
    }
    function historyAiStatus(entry) {
        var fields = entry.split("\t")
        return fields.length > 5 ? fields[5] : "not_recorded"
    }
    function historyAiLatency(entry) {
        var fields = entry.split("\t")
        return fields.length > 6 && isFinite(Number(fields[6])) ? Number(fields[6]) : 0
    }
    function aiHistoryStats() {
        var result = { "total": controller.historyEntries.length, "enhanced": 0, "fallback": 0, "attempts": 0, "latencyTotal": 0, "latencyCount": 0, "providers": {} }
        for (var i = 0; i < controller.historyEntries.length; ++i) {
            var fields = controller.historyEntries[i].split("\t")
            var status = fields.length > 5 ? fields[5] : "not_recorded"
            if (status !== "enhanced" && status !== "fallback")
                continue
            result.attempts += 1
            if (status === "enhanced")
                result.enhanced += 1
            else
                result.fallback += 1
            var latency = fields.length > 6 ? Number(fields[6]) : 0
            if (isFinite(latency) && latency > 0) {
                result.latencyTotal += latency
                result.latencyCount += 1
            }
            var provider = fields[3] || qsTr("Unknown provider")
            var model = fields[4] || qsTr("default model")
            var key = provider + " · " + model
            result.providers[key] = (result.providers[key] || 0) + 1
        }
        return result
    }
    function aiProviderSummary() {
        var providers = cachedAiHistoryStats.providers
        var rows = []
        for (var key in providers)
            rows.push({ "key": key, "count": providers[key] })
        rows.sort(function(a, b) { return b.count - a.count })
        if (rows.length === 0)
            return qsTr("No AI-enhanced dictations yet")
        var labels = []
        for (var i = 0; i < Math.min(4, rows.length); ++i)
            labels.push(rows[i].key + " (" + rows[i].count + ")")
        return labels.join("\n")
    }
    function wordCountForText(text) {
        text = text.trim()
        return text.length === 0 ? 0 : text.split(/\s+/).length
    }
    function historyChangeSummary(entry) {
        var raw = historyRawText(entry)
        var finalText = historyText(entry)
        var wordDelta = wordCountForText(finalText) - wordCountForText(raw)
        var characterDelta = finalText.length - raw.length
        function signed(value) { return value > 0 ? "+" + value : String(value) }
        return qsTr("Words %1 · characters %2").arg(signed(wordDelta)).arg(signed(characterDelta))
    }
    function htmlEscape(text) {
        return text.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;").replace(/\"/g, "&quot;")
    }
    function historyDiffHtml(entry) {
        var before = historyRawText(entry).trim().split(/\s+/)
        var after = historyText(entry).trim().split(/\s+/)
        if (before.length === 1 && before[0] === "") before = []
        if (after.length === 1 && after[0] === "") after = []
        if (before.length > 120 || after.length > 120)
            return "<span style='color:#d98a8a'>" + htmlEscape(historyRawText(entry)) + "</span><br/><span style='color:#70d59b'>" + htmlEscape(historyText(entry)) + "</span>"
        var table = []
        for (var i = 0; i <= before.length; ++i) {
            table[i] = []
            for (var j = 0; j <= after.length; ++j) table[i][j] = 0
        }
        for (i = before.length - 1; i >= 0; --i)
            for (j = after.length - 1; j >= 0; --j)
                table[i][j] = before[i] === after[j] ? table[i + 1][j + 1] + 1 : Math.max(table[i + 1][j], table[i][j + 1])
        var output = []; i = 0; j = 0
        while (i < before.length || j < after.length) {
            if (i < before.length && j < after.length && before[i] === after[j]) {
                output.push(htmlEscape(before[i])); ++i; ++j
            } else if (j < after.length && (i === before.length || table[i][j + 1] >= table[i + 1][j])) {
                output.push("<span style='color:#70d59b;background-color:#173c2b'>+" + htmlEscape(after[j]) + "</span>"); ++j
            } else {
                output.push("<span style='color:#e58b94;text-decoration:line-through'>−" + htmlEscape(before[i]) + "</span>"); ++i
            }
        }
        return output.join(" ")
    }
    function historyAiSummary(entry) {
        var fields = entry.split("\t")
        if (fields.length < 6 || fields[5] === "disabled" || fields[5] === "not_recorded")
            return qsTr("Local transcription")
        var provider = fields[3] || qsTr("AI")
        var model = fields[4] ? " · " + fields[4] : ""
        var latency = fields.length > 6 && Number(fields[6]) > 0 ? " · " + fields[6] + " ms" : ""
        return provider + model + " · " + fields[5] + latency
    }
    function historySource(entry) {
        var fields = entry.split("\t")
        return fields.length > 7 && fields[7] === "file" ? qsTr("Audio file") : qsTr("Dictation")
    }
    function historyAudioPath(entry) {
        var fields = entry.split("\t")
        return fields.length > 8 ? fields[8] : ""
    }
    function historyCleanupMode(entry) {
        var fields = entry.split("\t")
        var mode = fields.length > 9 ? fields[9] : "legacy"
        var language = fields.length > 10 && fields[10].length > 0 ? " · " + fields[10] : ""
        return mode + language
    }
    function meetingSegmentField(entry, index) {
        var fields = entry.split("\t")
        return fields.length > index ? fields[index] : ""
    }
    function meetingTimestamp(milliseconds) {
        var total = Math.max(0, Math.floor(Number(milliseconds) / 1000))
        var hours = Math.floor(total / 3600)
        var minutes = Math.floor(total / 60) % 60
        var seconds = total % 60
        return ("0" + hours).slice(-2) + ":" + ("0" + minutes).slice(-2) + ":" + ("0" + seconds).slice(-2)
    }
    function audioBudgetIndex() {
        var budgets = [100, 500, 1000, 2500, 5000, 10000]
        var index = budgets.indexOf(controller.audioHistoryBudgetMb)
        return index >= 0 ? index : 1
    }
    function historyWords(entry) {
        var text = historyText(entry).trim()
        return text.length === 0 ? 0 : text.split(/\s+/).length
    }
    function historyDate(entry) {
        var seconds = historyTimestamp(entry)
        return seconds > 0 ? Qt.formatDateTime(new Date(seconds * 1000), "d MMM yyyy, HH:mm") : qsTr("Date unavailable")
    }
    function historyRelativeTime(entry) {
        var seconds = historyTimestamp(entry)
        if (seconds <= 0)
            return qsTr("Unknown time")
        var elapsed = Math.max(0, Math.floor(Date.now() / 1000) - seconds)
        if (elapsed < 60)
            return qsTr("Just now")
        if (elapsed < 3600)
            return qsTr("%1m ago").arg(Math.floor(elapsed / 60))
        if (elapsed < 86400)
            return qsTr("%1h ago").arg(Math.floor(elapsed / 3600))
        if (elapsed < 604800)
            return qsTr("%1d ago").arg(Math.floor(elapsed / 86400))
        return Qt.formatDateTime(new Date(seconds * 1000), "d MMM")
    }
    function dayKeyFromSeconds(seconds) {
        var date = new Date(seconds * 1000)
        return Date.UTC(date.getFullYear(), date.getMonth(), date.getDate()) / 86400000
    }
    function todayHistoryWords() {
        var today = new Date()
        var key = Date.UTC(today.getFullYear(), today.getMonth(), today.getDate()) / 86400000
        var total = 0
        for (var i = 0; i < controller.historyEntries.length; ++i)
            if (dayKeyFromSeconds(historyTimestamp(controller.historyEntries[i])) === key)
                total += historyWords(controller.historyEntries[i])
        return total
    }
    function todayHistorySessions() {
        var today = new Date()
        var key = Date.UTC(today.getFullYear(), today.getMonth(), today.getDate()) / 86400000
        var total = 0
        for (var i = 0; i < controller.historyEntries.length; ++i)
            if (dayKeyFromSeconds(historyTimestamp(controller.historyEntries[i])) === key)
                ++total
        return total
    }
    function activityStats(days) {
        var now = Math.floor(Date.now() / 1000)
        var cutoff = days > 0 ? now - days * 86400 : 0
        var result = { sessions: 0, words: 0, dictations: 0, files: 0, activeDays: {}, longest: 0 }
        for (var i = 0; i < controller.historyEntries.length; ++i) {
            var entry = controller.historyEntries[i]
            var timestamp = historyTimestamp(entry)
            if (timestamp < cutoff) continue
            var words = historyWords(entry)
            result.sessions++
            result.words += words
            result.longest = Math.max(result.longest, words)
            result.activeDays[dayKeyFromSeconds(timestamp)] = true
            if (historySource(entry) === qsTr("Audio file")) result.files++
            else result.dictations++
        }
        var count = 0
        for (var key in result.activeDays) count++
        result.activeDayCount = count
        result.average = result.sessions > 0 ? Math.round(result.words / result.sessions) : 0
        return result
    }
    function aiEditImpact() {
        var result = { changed: 0, attempts: 0, wordDelta: 0, characterDelta: 0 }
        for (var i = 0; i < controller.historyEntries.length; ++i) {
            var entry = controller.historyEntries[i]
            var status = historyAiStatus(entry)
            if (status !== "enhanced" && status !== "fallback") continue
            result.attempts++
            var words = wordCountForText(historyText(entry)) - wordCountForText(historyRawText(entry))
            var characters = historyText(entry).length - historyRawText(entry).length
            result.wordDelta += words
            result.characterDelta += characters
            if (words !== 0 || characters !== 0 || historyText(entry) !== historyRawText(entry)) result.changed++
        }
        return result
    }
    function personalRecords() {
        var days = {}
        var sessions = {}
        var longest = 0
        var hours = new Array(24).fill(0)
        for (var i = 0; i < controller.historyEntries.length; ++i) {
            var entry = controller.historyEntries[i]
            var key = dayKeyFromSeconds(historyTimestamp(entry))
            days[key] = (days[key] || 0) + historyWords(entry)
            sessions[key] = (sessions[key] || 0) + 1
            longest = Math.max(longest, historyWords(entry))
            var timestamp = historyTimestamp(entry)
            if (timestamp > 0) hours[new Date(timestamp * 1000).getHours()]++
        }
        var mostWords = 0, mostSessions = 0, peakHour = -1, peakCount = 0
        for (var day in days) mostWords = Math.max(mostWords, days[day])
        for (day in sessions) mostSessions = Math.max(mostSessions, sessions[day])
        for (var hour = 0; hour < 24; ++hour) if (hours[hour] > peakCount) { peakCount = hours[hour]; peakHour = hour }
        return { longest: longest, mostWords: mostWords, mostSessions: mostSessions,
                 peakHour: peakHour < 0 ? qsTr("No data") : ("0" + peakHour).slice(-2) + ":00–" + ("0" + ((peakHour + 1) % 24)).slice(-2) + ":00" }
    }
    function wordsOnDay(daysAgo) {
        var date = new Date()
        date = new Date(date.getFullYear(), date.getMonth(), date.getDate() - daysAgo)
        var key = Date.UTC(date.getFullYear(), date.getMonth(), date.getDate()) / 86400000
        var total = 0
        for (var i = 0; i < controller.historyEntries.length; ++i)
            if (dayKeyFromSeconds(historyTimestamp(controller.historyEntries[i])) === key)
                total += historyWords(controller.historyEntries[i])
        return total
    }
    function maxDailyWords(days) {
        var maximum = 1
        for (var i = 0; i < days; ++i)
            maximum = Math.max(maximum, wordsOnDay(i))
        return maximum
    }
    function activeDayKeys() {
        var keys = []
        for (var i = 0; i < controller.historyEntries.length; ++i) {
            var seconds = historyTimestamp(controller.historyEntries[i])
            if (seconds <= 0)
                continue
            var key = dayKeyFromSeconds(seconds)
            if (keys.indexOf(key) < 0)
                keys.push(key)
        }
        keys.sort(function(a, b) { return b - a })
        return keys
    }
    function currentStreak() {
        var keys = activeDayKeys()
        if (keys.length === 0)
            return 0
        var today = new Date()
        var cursor = Date.UTC(today.getFullYear(), today.getMonth(), today.getDate()) / 86400000
        while (controller.skipWeekends && (new Date(cursor * 86400000).getUTCDay() === 0 || new Date(cursor * 86400000).getUTCDay() === 6)) cursor--
        if (keys.indexOf(cursor) < 0) {
            cursor--
            while (controller.skipWeekends && (new Date(cursor * 86400000).getUTCDay() === 0 || new Date(cursor * 86400000).getUTCDay() === 6)) cursor--
        }
        var streak = 0
        while (keys.indexOf(cursor) >= 0) {
            ++streak
            cursor -= 1
            while (controller.skipWeekends && (new Date(cursor * 86400000).getUTCDay() === 0 || new Date(cursor * 86400000).getUTCDay() === 6)) cursor--
        }
        return streak
    }
    function bestStreak() {
        var keys = activeDayKeys().sort(function(a, b) { return a - b })
        if (controller.skipWeekends)
            keys = keys.filter(function(key) {
                var day = new Date(key * 86400000).getUTCDay()
                return day !== 0 && day !== 6
            })
        if (keys.length === 0)
            return 0
        var best = 1
        var streak = 1
        for (var i = 1; i < keys.length; ++i) {
            var expected = keys[i - 1] + 1
            while (controller.skipWeekends && (new Date(expected * 86400000).getUTCDay() === 0 || new Date(expected * 86400000).getUTCDay() === 6)) expected++
            streak = keys[i] === expected ? streak + 1 : 1
            best = Math.max(best, streak)
        }
        return best
    }
    function timeSaved(words) {
        var minutes = Math.max(0, words / Math.max(10, controller.typingWpm) - words / 150)
        if (minutes < 1)
            return "< 1m"
        if (minutes < 60)
            return Math.floor(minutes) + "m"
        return Math.floor(minutes / 60) + "h " + Math.floor(minutes % 60) + "m"
    }
    function filteredHistory(query) {
        var normalized = query.trim().toLowerCase()
        var entries = []
        for (var i = 0; i < controller.historyEntries.length; ++i) {
            var entry = controller.historyEntries[i]
            if (normalized.length === 0 || historyText(entry).toLowerCase().indexOf(normalized) >= 0)
                entries.push(entry)
        }
        return entries
    }
    function refreshHistoryDerivedState() {
        cachedAiHistoryStats = aiHistoryStats()
        cachedFilteredHistory = filteredHistory(historySearch.text)
    }
    Connections {
        target: controller
        function onHistoryEntriesChanged() { root.refreshHistoryDerivedState() }
    }
    onClosing: function(close) {
        close.accepted = false
        root.hide()
    }

    // System mode follows Plasma's active palette. The explicit FluidVoice
    // themes retain upstream visual identity for users who prefer it.
    readonly property bool darkTheme: controller.selectedTheme === 1
                                      || (controller.selectedTheme === 0
                                          && root.palette.window.hslLightness < 0.5)
    readonly property color accent: controller.selectedAccent === 0 ? systemPalette.highlight
                                    : controller.selectedAccent === 1 ? "#3ac8c6"
                                    : controller.selectedAccent === 2 ? "#55c98b"
                                    : "#9b87f5"
    readonly property color windowBackground: controller.selectedTheme === 0 ? root.palette.window
                                              : darkTheme ? "#121212" : "#f4f4f5"
    readonly property color contentBackground: controller.selectedTheme === 0 ? root.palette.base
                                               : darkTheme ? "#171717" : "#fafafa"
    readonly property color sidebarBackground: controller.selectedTheme === 0 ? root.palette.alternateBase
                                               : darkTheme ? "#0f0f0f" : "#eeeeef"
    readonly property color panel: controller.selectedTheme === 0 ? root.palette.base
                                   : darkTheme ? "#151515" : "#ffffff"
    readonly property color panelRaised: controller.selectedTheme === 0 ? root.palette.button
                                         : darkTheme ? "#1c1c1c" : "#e8e8eb"
    readonly property color primaryText: controller.selectedTheme === 0 ? root.palette.text
                                         : darkTheme ? "#f2f2f2" : "#202024"
    readonly property color secondaryText: controller.selectedTheme === 0 ? root.palette.placeholderText
                                           : darkTheme ? "#a8a8ad" : "#616168"
    readonly property color tertiaryText: darkTheme ? "#737379" : "#7a7a82"
    readonly property color hairline: controller.selectedTheme === 0 ? root.palette.mid
                                      : darkTheme ? "#2b2b2e" : "#d7d7da"
    readonly property color selectionSurface: Qt.rgba(root.accent.r, root.accent.g, root.accent.b,
                                                       darkTheme ? 0.30 : 0.18)
    readonly property color accentText: root.accent.hslLightness > 0.62 ? "#202024" : "#ffffff"
    readonly property color success: "#56d394"
    readonly property color warning: "#f0b45a"

    // Feed the chosen accent back into Qt Quick Controls so sliders, switches,
    // selections, progress bars, and focused controls all update with it.
    palette.highlight: root.accent
    palette.highlightedText: root.accentText

    FluidVoiceController {
        id: controller
    }

    Connections {
        target: controller
        function onWriteModeActivationChanged() {
            root.showSettingsSection(2)
            root.show()
            root.raise()
            root.requestActivate()
            rewriteInstruction.forceActiveFocus()
        }
    }

    FileDialog {
        id: audioFileDialog
        title: qsTr("Choose an audio file")
        nameFilters: [qsTr("Audio files (*.wav *.mp3 *.flac *.ogg *.opus *.m4a *.aac *.webm *.mp4)")]
        onAccepted: controller.transcribeFile(selectedFile.toString())
    }

    Timer {
        id: rewriteDelay
        property string instruction: ""
        interval: 250
        repeat: false
        onTriggered: controller.rewriteSelectedText(instruction)
    }

    Timer {
        id: overlayPreviewTimer
        interval: 4000
        onTriggered: if (!controller.recording && !controller.transcribing && !controller.overlayResultAvailable) controller.setOverlayPreview(false)
    }

    FileDialog {
        id: historyJsonDialog
        title: qsTr("Export history as JSON")
        fileMode: FileDialog.SaveFile
        defaultSuffix: "json"
        nameFilters: [qsTr("JSON files (*.json)")]
        onAccepted: controller.exportHistory(selectedFile.toString(), "json")
    }

    FileDialog {
        id: historyCsvDialog
        title: qsTr("Export history as CSV")
        fileMode: FileDialog.SaveFile
        defaultSuffix: "csv"
        nameFilters: [qsTr("CSV files (*.csv)")]
        onAccepted: controller.exportHistory(selectedFile.toString(), "csv")
    }

    FileDialog {
        id: audioHistoryZipDialog
        title: qsTr("Export retained audio history")
        fileMode: FileDialog.SaveFile
        defaultSuffix: "zip"
        nameFilters: [qsTr("ZIP archives (*.zip)")]
        onAccepted: controller.exportAudioHistory(selectedFile.toString())
    }

    FileDialog {
        id: dictionaryImportDialog
        title: qsTr("Import dictionary")
        nameFilters: [qsTr("Dictionary files (*.csv *.tsv *.txt)")]
        onAccepted: controller.importDictionary(selectedFile.toString(), dictionaryConflictMode.currentIndex)
    }

    FileDialog {
        id: dictionaryExportDialog
        title: qsTr("Export dictionary")
        fileMode: FileDialog.SaveFile
        defaultSuffix: "csv"
        nameFilters: [qsTr("CSV files (*.csv)")]
        onAccepted: controller.exportDictionary(selectedFile.toString())
    }

    FileDialog {
        id: meetingExportDialog
        title: qsTr("Export timestamped transcript")
        fileMode: FileDialog.SaveFile
        defaultSuffix: meetingExportFormat.currentText.toLowerCase()
        nameFilters: [qsTr("Transcript files (*.%1)").arg(meetingExportFormat.currentText.toLowerCase())]
        onAccepted: controller.exportMeeting(selectedFile.toString(), meetingExportFormat.currentText.toLowerCase())
    }

    Dialog {
        id: resetStatsDialog
        title: qsTr("Reset all statistics?")
        modal: true
        anchors.centerIn: parent
        standardButtons: Dialog.Cancel | Dialog.Ok
        onAccepted: controller.clearHistory()
        contentItem: Text {
            text: qsTr("This permanently deletes all %1 history entries and retained recordings. This action cannot be undone.").arg(controller.transcriptCount)
            color: root.primaryText; wrapMode: Text.Wrap; width: 420
        }
    }

    Dialog {
        id: onboardingDialog
        modal: true
        closePolicy: Popup.NoAutoClose
        anchors.centerIn: parent
        width: Math.min(root.width - 48, 760)
        height: Math.min(root.height - 48, 570)
        padding: 0
        background: Rectangle { radius: 20; color: root.panel; border.color: root.hairline }
        contentItem: ColumnLayout {
            anchors.fill: parent
            anchors.margins: 24
            spacing: 16
            RowLayout {
                Layout.fillWidth: true
                Image { source: "qrc:/qt/qml/io/github/davidkodar/FluidVoiceLinux/assets/fluidvoice-app.png"; sourceSize.width: 42; sourceSize.height: 42; fillMode: Image.PreserveAspectFit }
                ColumnLayout {
                    Layout.fillWidth: true; spacing: 2
                    Text { text: qsTr("Welcome to FluidVoice"); color: root.primaryText; font.pixelSize: 22; font.weight: Font.Bold }
                    Text { text: qsTr("Private, native dictation for KDE Plasma"); color: root.secondaryText; font.pixelSize: 12 }
                }
                Text { text: qsTr("Step %1 of 4").arg(root.onboardingStep + 1); color: root.tertiaryText; font.pixelSize: 11 }
            }
            ProgressBar { Layout.fillWidth: true; from: 0; to: 4; value: root.onboardingStep + 1 }
            StackLayout {
                Layout.fillWidth: true; Layout.fillHeight: true; currentIndex: root.onboardingStep
                ColumnLayout {
                    spacing: 12
                    Text { text: qsTr("1. Microphone and privacy"); color: root.primaryText; font.pixelSize: 19; font.weight: Font.DemiBold }
                    Text { Layout.fillWidth: true; text: qsTr("FluidVoice records only while you hold the dictation shortcut. Speech recognition runs locally. Audio history is off by default, and cloud AI enhancement remains optional and disabled until you configure it."); color: root.secondaryText; font.pixelSize: 13; wrapMode: Text.Wrap }
                    Rectangle { Layout.fillWidth: true; implicitHeight: 88; radius: 12; color: root.panelRaised; border.color: root.hairline
                        RowLayout { anchors.fill: parent; anchors.margins: 14; spacing: 14
                            Rectangle { width: 12; height: 12; radius: 6; color: controller.inputSources.length > 0 ? root.success : root.warning }
                            ColumnLayout { Layout.fillWidth: true
                                Text { text: controller.inputSources.length > 0 ? qsTr("Microphone detected") : qsTr("Waiting for microphone access"); color: root.primaryText; font.pixelSize: 14; font.weight: Font.Medium }
                                Text { Layout.fillWidth: true; text: controller.microphoneName.length > 0 ? controller.microphoneName : qsTr("Open Voice Engine after setup to select and test the correct input."); color: root.secondaryText; font.pixelSize: 11; wrapMode: Text.Wrap }
                            }
                        }
                    }
                    Item { Layout.fillHeight: true }
                }
                ColumnLayout {
                    spacing: 10
                    Text { text: qsTr("2. Choose the right Whisper model"); color: root.primaryText; font.pixelSize: 19; font.weight: Font.DemiBold }
                    Text { Layout.fillWidth: true; text: qsTr("Larger models generally improve recognition—especially outside English—but require more memory and processing time. Start conservatively, test your own voice, then move up only if accuracy needs improvement."); color: root.secondaryText; font.pixelSize: 12; wrapMode: Text.Wrap }
                    GridLayout {
                        Layout.fillWidth: true; columns: 4; columnSpacing: 8; rowSpacing: 7
                        Repeater { model: [qsTr("Model"), qsTr("Best for"), qsTr("Speed"), qsTr("Download")]
                            Text { required property string modelData; text: modelData; color: root.tertiaryText; font.pixelSize: 10; font.weight: Font.DemiBold }
                        }
                        Repeater { model: [
                            ["Tiny / Base", qsTr("Fast English tests"), qsTr("Fastest"), "75–141 MB"],
                            ["Small", qsTr("Balanced everyday use"), qsTr("Fast"), "465 MB"],
                            ["Medium", qsTr("Better multilingual accuracy"), qsTr("Slower"), "1.4 GB"],
                            ["Large Turbo", qsTr("High accuracy, modern GPU"), qsTr("Medium"), "1.5 GB"],
                            ["Large v3", qsTr("Maximum accuracy"), qsTr("Slowest"), "2.9 GB"]
                        ]
                            delegate: Repeater { required property var modelData; model: modelData
                                Text { required property string modelData; Layout.fillWidth: true; text: modelData; color: root.primaryText; font.pixelSize: 11; wrapMode: Text.Wrap }
                            }
                        }
                    }
                    Rectangle { Layout.fillWidth: true; implicitHeight: 58; radius: 10; color: root.selectionSurface; border.color: root.accent
                        Text { anchors.fill: parent; anchors.margins: 12; text: qsTr("Recommendation: start with Base for English or Small for general multilingual use. Try Medium or Large Turbo when accuracy matters more than speed."); color: root.accent; font.pixelSize: 11; wrapMode: Text.Wrap; verticalAlignment: Text.AlignVCenter }
                    }
                    Item { Layout.fillHeight: true }
                }
                ColumnLayout {
                    spacing: 12
                    Text { text: qsTr("3. Performance and experimental engines"); color: root.primaryText; font.pixelSize: 19; font.weight: Font.DemiBold }
                    Text { Layout.fillWidth: true; text: qsTr("Automatic (Vulkan) is recommended: FluidVoice uses a compatible AMD, Intel, or NVIDIA GPU when available and safely falls back to CPU. CPU mode is the most compatible but can be substantially slower with large models."); color: root.secondaryText; font.pixelSize: 13; wrapMode: Text.Wrap }
                    Rectangle { Layout.fillWidth: true; implicitHeight: 88; radius: 12; color: root.panelRaised; border.color: root.hairline
                        ColumnLayout { anchors.fill: parent; anchors.margins: 14
                            Text { text: qsTr("Built-in Whisper · recommended"); color: root.primaryText; font.pixelSize: 14; font.weight: Font.DemiBold }
                            Text { Layout.fillWidth: true; text: qsTr("The stable default with broad language support, predictable downloads, local processing, and CPU/Vulkan operation."); color: root.secondaryText; font.pixelSize: 11; wrapMode: Text.Wrap }
                        }
                    }
                    Rectangle { Layout.fillWidth: true; implicitHeight: 104; radius: 12; color: root.panelRaised; border.color: root.warning
                        ColumnLayout { anchors.fill: parent; anchors.margins: 14
                            Text { text: qsTr("Native engines · experimental"); color: root.warning; font.pixelSize: 14; font.weight: Font.DemiBold }
                            Text { Layout.fillWidth: true; text: qsTr("Parakeet and Nemotron may be faster for particular workflows, but installation, language accuracy, live preview, and GPU behavior vary. They always retain a Whisper fallback and should be tested before relying on them."); color: root.secondaryText; font.pixelSize: 11; wrapMode: Text.Wrap }
                        }
                    }
                    Item { Layout.fillHeight: true }
                }
                ColumnLayout {
                    spacing: 12
                    Text { text: qsTr("4. Ready for a test dictation"); color: root.primaryText; font.pixelSize: 19; font.weight: Font.DemiBold }
                    Text { Layout.fillWidth: true; text: qsTr("After finishing, open Voice Engine, test the selected microphone, download a model, and hold Ctrl+Alt+D while speaking. Release the shortcut to transcribe and paste into the active application."); color: root.secondaryText; font.pixelSize: 13; wrapMode: Text.Wrap }
                    Rectangle { Layout.fillWidth: true; implicitHeight: readinessColumn.implicitHeight + 28; radius: 12; color: root.panelRaised; border.color: root.hairline
                        ColumnLayout { id: readinessColumn; anchors.fill: parent; anchors.margins: 14; spacing: 8
                            Text { text: (controller.inputSources.length > 0 ? "✓ " : "○ ") + qsTr("Microphone detected"); color: controller.inputSources.length > 0 ? root.success : root.secondaryText; font.pixelSize: 12 }
                            Text { text: (controller.modelName.length > 0 ? "✓ " : "○ ") + qsTr("Speech model selected or ready to download"); color: controller.modelName.length > 0 ? root.success : root.secondaryText; font.pixelSize: 12 }
                            Text { text: "✓ " + qsTr("Shortcut: Ctrl+Alt+D"); color: root.success; font.pixelSize: 12 }
                            Text { text: "✓ " + qsTr("Local transcription and safe clipboard recovery"); color: root.success; font.pixelSize: 12 }
                        }
                    }
                    Text { Layout.fillWidth: true; text: qsTr("You can reopen this guide at any time from Getting Started."); color: root.tertiaryText; font.pixelSize: 11; wrapMode: Text.Wrap }
                    Item { Layout.fillHeight: true }
                }
            }
            RowLayout {
                Layout.fillWidth: true
                Button { visible: root.onboardingStep > 0; text: qsTr("Back"); onClicked: root.onboardingStep-- }
                Item { Layout.fillWidth: true }
                Button { visible: root.onboardingStep < 3; text: qsTr("Continue"); highlighted: true; onClicked: root.onboardingStep++ }
                Button { visible: root.onboardingStep === 3; text: qsTr("Finish and open Voice Engine"); highlighted: true; onClicked: { controller.completeOnboarding(); onboardingDialog.close(); root.showSettingsSection(1) } }
            }
        }
    }

    Component.onCompleted: {
        refreshHistoryDerivedState()
        controller.initializeAudio()
        controller.initializeDesktopRuntime()
        if (!controller.onboardingCompleted)
            Qt.callLater(function() { root.onboardingStep = 0; onboardingDialog.open() })
    }

    background: Rectangle {
        color: root.color
        gradient: Gradient {
            GradientStop { position: 0.0; color: root.contentBackground }
            GradientStop { position: 1.0; color: root.windowBackground }
        }
    }

    header: Rectangle {
        height: 44
        color: root.sidebarBackground
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
                color: controller.recording ? root.selectionSurface : root.panelRaised
                border.color: controller.recording ? root.accent : root.hairline

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
            color: root.sidebarBackground
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
                        { "name": qsTr("Change logs"), "symbol": "≡", "page": 9 },
                        { "name": qsTr("Feedback"), "symbol": "✉", "page": 10 },
                        { "name": qsTr("About & License"), "symbol": "ⓘ", "page": 11 }
                    ]
                    delegate: Rectangle {
                        required property var modelData
                        required property int index
                        Layout.fillWidth: true
                        readonly property bool isHeader: modelData.header === true
                        Layout.topMargin: isHeader && index > 0 ? 9 : 0
                        height: isHeader ? 22 : 34
                        radius: 6
                        color: !isHeader && modelData.page === root.settingsSection ? root.selectionSurface : "transparent"

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
                        Rectangle {
                            visible: !parent.isHeader && modelData.page === root.settingsSection
                            anchors.left: parent.left
                            anchors.leftMargin: 2
                            anchors.verticalCenter: parent.verticalCenter
                            width: 3
                            height: 20
                            radius: 2
                            color: root.accent
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
                    text: qsTr("Unofficial Linux port · %1").arg(controller.appVersion)
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
                        Item {
                            Layout.fillWidth: true
                            height: 48
                            Column {
                                anchors.left: parent.left
                                anchors.right: backgroundStatusPill.left
                                anchors.rightMargin: 20
                                anchors.verticalCenter: parent.verticalCenter
                                spacing: 2
                                Text { text: qsTr("Background operation"); color: root.primaryText; font.pixelSize: 14; font.weight: Font.Medium }
                                Text { text: qsTr("FluidVoice stays available in the Plasma system tray when this window is closed."); color: root.secondaryText; font.pixelSize: 13 }
                            }
                            Rectangle {
                                id: backgroundStatusPill
                                anchors.right: parent.right
                                anchors.verticalCenter: parent.verticalCenter
                                implicitWidth: backgroundStatus.implicitWidth + 18
                                implicitHeight: 26
                                radius: 13
                                color: root.selectionSurface
                                border.color: root.accent
                                Text { id: backgroundStatus; anchors.centerIn: parent; text: qsTr("Active"); color: root.accent; font.pixelSize: 11; font.weight: Font.Medium }
                            }
                        }
                    }
                }

                Rectangle {
                    visible: root.settingsSection === 0
                    Layout.fillWidth: true
                    implicitHeight: deliveryDiagnostics.implicitHeight + 32
                    radius: 16
                    color: root.panel
                    border.color: root.hairline
                    ColumnLayout {
                        id: deliveryDiagnostics; anchors.fill: parent; anchors.margins: 16; spacing: 10
                        RowLayout {
                            Layout.fillWidth: true
                            ColumnLayout {
                                Layout.fillWidth: true; spacing: 3
                                Text { text: qsTr("Text delivery"); color: root.primaryText; font.pixelSize: 14; font.weight: Font.Medium }
                                Text { Layout.fillWidth: true; text: qsTr("FluidVoice uses the Plasma keyboard portal for direct paste and always keeps successful transcripts on the clipboard."); color: root.secondaryText; font.pixelSize: 12; wrapMode: Text.Wrap }
                            }
                            Button { text: qsTr("Run check"); onClicked: controller.diagnoseTextDelivery() }
                        }
                        Text { Layout.fillWidth: true; text: controller.textDeliveryStatus; color: controller.textDeliveryStatus.indexOf("ready") >= 0 ? root.accent : root.secondaryText; font.pixelSize: 11; wrapMode: Text.Wrap }
                    }
                }

                Rectangle {
                    visible: root.settingsSection === 0
                    Layout.fillWidth: true
                    implicitHeight: localApiSettings.implicitHeight + 32
                    radius: 16
                    color: root.panel
                    border.color: root.hairline
                    ColumnLayout {
                        id: localApiSettings
                        anchors.fill: parent
                        anchors.margins: 16
                        spacing: 12
                        Text { text: qsTr("LOCAL API"); color: root.tertiaryText; font.pixelSize: 11; font.weight: Font.Medium }
                        RowLayout {
                            Layout.fillWidth: true
                            ColumnLayout {
                                Layout.fillWidth: true; spacing: 3
                                Text { text: qsTr("Loopback automation"); color: root.primaryText; font.pixelSize: 14; font.weight: Font.Medium }
                                Text { Layout.fillWidth: true; text: qsTr("Optional authenticated API bound only to 127.0.0.1. Disabled by default."); color: root.secondaryText; font.pixelSize: 12; wrapMode: Text.Wrap }
                            }
                            Switch {
                                checked: controller.localApiEnabled
                                onToggled: controller.updateLocalApi(checked, localApiPort.value)
                            }
                        }
                        RowLayout {
                            Layout.fillWidth: true
                            Text { text: qsTr("Port"); color: root.secondaryText; font.pixelSize: 12 }
                            Item { Layout.fillWidth: true }
                            SpinBox {
                                id: localApiPort
                                from: 1024; to: 65535
                                value: controller.localApiPort
                                editable: true
                                onValueModified: controller.updateLocalApi(controller.localApiEnabled, value)
                            }
                        }
                        RowLayout {
                            Layout.fillWidth: true
                            Button { text: qsTr("Show token location"); onClicked: controller.showLocalApiTokenLocation() }
                            Button { text: qsTr("Rotate token"); onClicked: controller.rotateLocalApiToken() }
                            Item { Layout.fillWidth: true }
                        }
                        Text { Layout.fillWidth: true; text: controller.localApiStatus; color: root.secondaryText; font.pixelSize: 11; wrapMode: Text.Wrap }
                        Text { Layout.fillWidth: true; text: qsTr("Endpoints: GET /v1/health, GET /v1/status, POST /v1/dictation/toggle. Browser Origin requests are rejected."); color: root.tertiaryText; font.pixelSize: 11; wrapMode: Text.Wrap }
                    }
                }

                Rectangle {
                    visible: root.settingsSection === 0
                    Layout.fillWidth: true
                    implicitHeight: overlayAppearance.implicitHeight + 32
                    radius: 16
                    color: root.panel
                    border.color: root.hairline
                    ColumnLayout {
                        id: overlayAppearance; anchors.fill: parent; anchors.margins: 16; spacing: 12
                        RowLayout { Layout.fillWidth: true
                            Text { text: qsTr("OVERLAY APPEARANCE"); color: root.tertiaryText; font.pixelSize: 11; font.weight: Font.Medium }
                            Item { Layout.fillWidth: true }
                            Button { text: qsTr("Preview"); onClicked: { controller.setOverlayPreview(true); overlayPreviewTimer.restart() } }
                        }
                        GridLayout {
                            Layout.fillWidth: true; columns: 2; columnSpacing: 18; rowSpacing: 10
                            Text { text: qsTr("Size"); color: root.secondaryText; font.pixelSize: 12 }
                            ComboBox { Layout.fillWidth: true; model: controller.overlaySizes; currentIndex: controller.selectedOverlaySize; onActivated: function(index) { controller.updateOverlayPreferences(index, controller.selectedOverlayPosition, controller.overlayShowText, controller.overlayOpacity) } }
                            Text { text: qsTr("Position"); color: root.secondaryText; font.pixelSize: 12 }
                            ComboBox { Layout.fillWidth: true; model: controller.overlayPositions; currentIndex: controller.selectedOverlayPosition; onActivated: function(index) { controller.updateOverlayPreferences(controller.selectedOverlaySize, index, controller.overlayShowText, controller.overlayOpacity) } }
                            Text { text: qsTr("Live text"); color: root.secondaryText; font.pixelSize: 12 }
                            Switch { checked: controller.overlayShowText; onToggled: controller.updateOverlayPreferences(controller.selectedOverlaySize, controller.selectedOverlayPosition, checked, controller.overlayOpacity) }
                            Text { text: qsTr("Opacity"); color: root.secondaryText; font.pixelSize: 12 }
                            RowLayout { Layout.fillWidth: true
                                Slider { Layout.fillWidth: true; from: 0.55; to: 1.0; stepSize: 0.05; value: controller.overlayOpacity; onMoved: controller.updateOverlayPreferences(controller.selectedOverlaySize, controller.selectedOverlayPosition, controller.overlayShowText, value) }
                                Text { text: Math.round(controller.overlayOpacity * 100) + "%"; color: root.secondaryText; font.pixelSize: 11; Layout.preferredWidth: 42 }
                            }
                        }
                        Text { Layout.fillWidth: true; text: qsTr("Plasma controls final Wayland window placement; FluidVoice requests the selected screen position and keeps the result recoverable if the compositor adjusts it."); color: root.tertiaryText; font.pixelSize: 10; wrapMode: Text.Wrap }
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

                        ComboBox {
                            Layout.fillWidth: true
                            model: controller.speechEngines
                            currentIndex: controller.selectedSpeechEngine
                            enabled: !controller.recording && !controller.transcribing
                            onActivated: function(index) { controller.selectSpeechEngine(index) }
                        }
                        TextField {
                            visible: controller.selectedSpeechEngine === 5
                            Layout.fillWidth: true
                            text: controller.localSpeechUrl
                            placeholderText: qsTr("http://127.0.0.1:8080")
                            onEditingFinished: controller.updateLocalSpeechUrl(text)
                        }
                        Text {
                            visible: controller.selectedSpeechEngine === 5
                            Layout.fillWidth: true
                            text: qsTr("Experimental OpenAI-compatible /v1/audio/transcriptions endpoint. Only HTTP loopback is accepted; live preview is unavailable with an external process.")
                            color: root.accent; font.pixelSize: 11; wrapMode: Text.Wrap
                        }

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
                            visible: controller.selectedSpeechEngine !== 5
                            Layout.fillWidth: true
                            Layout.preferredHeight: visible ? 62 : 0
                            spacing: 24
                            ColumnLayout {
                                Layout.fillWidth: true
                                spacing: 3
                                Text { text: qsTr("Compute backend"); color: root.primaryText; font.pixelSize: 14; font.weight: Font.Medium }
                                Text {
                                    Layout.fillWidth: true
                                    text: controller.selectedComputeBackend === 2
                                          ? qsTr("Force CPU inference")
                                          : controller.selectedComputeBackend === 0
                                            ? qsTr("Try Vulkan first, then install and use CPU automatically")
                                            : qsTr("Require Vulkan acceleration")
                                    color: root.secondaryText
                                    font.pixelSize: 12
                                    wrapMode: Text.Wrap
                                }
                            }
                            ComboBox {
                                id: computeBackendSelector
                                Layout.preferredWidth: 320
                                Layout.alignment: Qt.AlignRight | Qt.AlignVCenter
                                model: controller.computeBackends
                                currentIndex: controller.selectedComputeBackend
                                enabled: !controller.recording && !controller.transcribing
                                onActivated: function(index) { controller.selectComputeBackend(index) }
                            }
                        }
                        RowLayout {
                            visible: controller.selectedSpeechEngine !== 5
                            Layout.fillWidth: true
                            Text { Layout.fillWidth: true; text: controller.computeStatus; color: root.tertiaryText; font.pixelSize: 10; wrapMode: Text.Wrap }
                            Button { text: qsTr("Refresh diagnostics"); onClicked: controller.diagnoseComputeBackend(); Accessible.name: qsTr("Refresh compute backend diagnostics") }
                        }

                        Rectangle {
                            visible: controller.selectedSpeechEngine >= 1 && controller.selectedSpeechEngine <= 4
                            Layout.fillWidth: true
                            implicitHeight: parakeetSetup.implicitHeight + 28
                            radius: 12
                            color: root.panelRaised
                            border.color: root.hairline
                            ColumnLayout {
                                id: parakeetSetup
                                anchors.fill: parent
                                anchors.margins: 14
                                spacing: 10
                                Text { text: controller.speechEngines[controller.selectedSpeechEngine]; color: root.primaryText; font.pixelSize: 15; font.weight: Font.DemiBold }
                                Text { Layout.fillWidth: true; text: controller.nativeModelDetail; color: root.tertiaryText; font.pixelSize: 11; wrapMode: Text.Wrap }
                                Rectangle {
                                    visible: controller.selectedSpeechEngine === 2
                                    Layout.preferredWidth: supportNotice.implicitWidth + 20
                                    Layout.preferredHeight: 28
                                    radius: 14
                                    color: root.selectionSurface
                                    border.color: root.accent
                                    Text {
                                        id: supportNotice
                                        anchors.centerIn: parent
                                        text: qsTr("Language support varies")
                                        color: root.accent
                                        font.pixelSize: 11
                                        font.weight: Font.Medium
                                    }
                                    HoverHandler { id: supportNoticeHover }
                                    ToolTip.visible: supportNoticeHover.hovered
                                    ToolTip.delay: 350
                                    ToolTip.text: qsTr("Recognition quality varies by language. Languages outside Nemotron's primary transcription tier may be less accurate. For maximum multilingual accuracy, try Whisper Medium or Large Turbo.")
                                }
                                Text {
                                    visible: controller.selectedSpeechEngine === 2
                                    Layout.fillWidth: true
                                    text: qsTr("Recognition quality varies by language. For maximum multilingual accuracy, try Whisper Medium or Large Turbo.")
                                    color: root.secondaryText
                                    font.pixelSize: 11
                                    wrapMode: Text.Wrap
                                }
                                Text {
                                    visible: (controller.selectedSpeechEngine === 3 || controller.selectedSpeechEngine === 4)
                                             && controller.selectedLanguage !== 0
                                             && controller.languages[controller.selectedLanguage] !== "English"
                                    Layout.fillWidth: true
                                    text: qsTr("This engine recognizes English only. With a fixed non-English language, FluidVoice uses the selected multilingual Whisper model for both preview and final text.")
                                    color: root.accent
                                    font.pixelSize: 11
                                    wrapMode: Text.Wrap
                                }
                                Text { Layout.fillWidth: true; text: controller.parakeetStatus; color: root.secondaryText; font.pixelSize: 12; wrapMode: Text.Wrap }
                                RowLayout {
                                    visible: controller.parakeetBusy
                                    Layout.fillWidth: true
                                    BusyIndicator { running: controller.parakeetBusy; implicitWidth: 24; implicitHeight: 24 }
                                    Text { Layout.fillWidth: true; text: controller.parakeetDownloadProgress > 0 ? qsTr("Downloading and verifying the model…") : qsTr("Compiling the native runtime in the background; please keep FluidVoice open."); color: root.accent; font.pixelSize: 12; wrapMode: Text.Wrap }
                                }
                                RowLayout {
                                    Layout.fillWidth: true
                                    Text { Layout.fillWidth: true; text: controller.parakeetRuntimeInstalled ? qsTr("Shared native runtime installed") : qsTr("Shared native runtime required"); color: controller.parakeetRuntimeInstalled ? root.accent : root.secondaryText }
                                    Button { Layout.preferredWidth: 160; text: controller.parakeetRuntimeInstalled ? qsTr("Rebuild") : qsTr("Install runtime"); enabled: !controller.parakeetBusy; onClicked: controller.installParakeetRuntime() }
                                }
                                RowLayout {
                                    Layout.fillWidth: true
                                    Text { Layout.fillWidth: true; text: controller.parakeetModelInstalled ? qsTr("Selected model downloaded and verified") : qsTr("Selected model is not downloaded"); color: controller.parakeetModelInstalled ? root.accent : root.secondaryText }
                                    Button { visible: !controller.parakeetModelInstalled; Layout.preferredWidth: 160; enabled: !controller.parakeetBusy || controller.parakeetDownloadProgress > 0; text: controller.parakeetDownloadProgress > 0 ? qsTr("Cancel") : qsTr("Install"); onClicked: controller.parakeetDownloadProgress > 0 ? controller.cancelParakeetDownload() : controller.downloadParakeetModel() }
                                    Button { visible: controller.parakeetModelInstalled; Layout.preferredWidth: 160; text: qsTr("Delete model"); enabled: !controller.parakeetBusy; onClicked: controller.deleteParakeetModel() }
                                }
                                ProgressBar { visible: controller.parakeetBusy && controller.parakeetDownloadProgress > 0; Layout.fillWidth: true; value: controller.parakeetDownloadProgress }
                                RowLayout {
                                    Layout.fillWidth: true
                                    Text { Layout.fillWidth: true; text: qsTr("Runs locally using the pinned NeMo-Speech.cpp runtime. GPU acceleration is cross-vendor through Vulkan; no NVIDIA GPU or CUDA is required. Whisper provides live text and automatic fallback."); color: root.tertiaryText; font.pixelSize: 11; wrapMode: Text.Wrap }
                                    Button { Layout.preferredWidth: 160; text: qsTr("Check setup"); enabled: !controller.parakeetBusy; onClicked: controller.diagnoseParakeet() }
                                }
                                Text { Layout.fillWidth: true; text: qsTr("Install performs the complete verified setup in one click. The runtime is shared, so additional native models only require their model download."); color: root.tertiaryText; font.pixelSize: 11; wrapMode: Text.Wrap }
                            }
                        }

                        Text { visible: controller.selectedSpeechEngine === 0; text: qsTr("Whisper models"); color: root.primaryText; font.pixelSize: 14; font.weight: Font.Medium }

                        ColumnLayout {
                            id: modelList
                            visible: controller.selectedSpeechEngine === 0
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
                                    color: index === controller.selectedModel ? root.selectionSurface : root.panelRaised
                                    border.color: index === controller.selectedModel ? root.accent : root.hairline

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
                                            color: index === controller.selectedModel ? root.accent : root.tertiaryText
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
                            visible: controller.selectedSpeechEngine === 0
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
                    height: 276
                    radius: 16
                    color: root.panel
                    border.color: root.hairline

                    ColumnLayout {
                        anchors.fill: parent
                        anchors.margins: 16
                        spacing: 14
                        Text { text: qsTr("DICTATION"); color: root.tertiaryText; font.pixelSize: 11; font.weight: Font.Medium }

                        Item {
                            Layout.fillWidth: true
                            height: 56
                            Column {
                                anchors.left: parent.left
                                anchors.right: shortcutSelector.left
                                anchors.rightMargin: 24
                                anchors.verticalCenter: parent.verticalCenter
                                spacing: 4
                                Text { text: qsTr("Hold to dictate"); color: root.primaryText; font.pixelSize: 14; font.weight: Font.Medium }
                                Text { text: qsTr("Recording stops when the shortcut is released."); color: root.secondaryText; font.pixelSize: 12 }
                            }
                            ComboBox {
                                id: shortcutSelector
                                anchors.right: parent.right
                                anchors.verticalCenter: parent.verticalCenter
                                width: 260
                                model: controller.shortcuts
                                currentIndex: controller.selectedShortcut
                                enabled: !controller.recording && !controller.transcribing
                                onActivated: function(index) { controller.selectShortcut(index) }
                            }
                        }

                        Rectangle { Layout.fillWidth: true; height: 1; color: root.hairline }

                        Item {
                            Layout.fillWidth: true
                            height: 56
                            Column {
                                anchors.left: parent.left
                                anchors.right: overlaySwitch.left
                                anchors.rightMargin: 24
                                anchors.verticalCenter: parent.verticalCenter
                                spacing: 4
                                Text { text: qsTr("Dictation popup"); color: root.primaryText; font.pixelSize: 14; font.weight: Font.Medium }
                                Text { text: qsTr("Show listening and live text above other windows while dictating."); color: root.secondaryText; font.pixelSize: 12 }
                            }
                            Switch {
                                id: overlaySwitch
                                anchors.right: parent.right
                                anchors.verticalCenter: parent.verticalCenter
                                checked: controller.overlayEnabled
                                onToggled: controller.updateOverlayEnabled(checked)
                            }
                        }

                        Rectangle { Layout.fillWidth: true; height: 1; color: root.hairline }

                        Item {
                            Layout.fillWidth: true
                            height: 56
                            Column {
                                anchors.left: parent.left
                                anchors.right: resultOverlaySwitch.left
                                anchors.rightMargin: 24
                                anchors.verticalCenter: parent.verticalCenter
                                spacing: 4
                                Text { text: qsTr("Keep result popup"); color: root.primaryText; font.pixelSize: 14; font.weight: Font.Medium }
                                Text { text: qsTr("Keep the final text and recovery menu visible after it is pasted."); color: root.secondaryText; font.pixelSize: 12 }
                            }
                            Switch {
                                id: resultOverlaySwitch
                                anchors.right: parent.right
                                anchors.verticalCenter: parent.verticalCenter
                                enabled: controller.overlayEnabled
                                checked: controller.overlayKeepResult
                                onToggled: controller.updateOverlayKeepResult(checked)
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
                    border.color: root.accent

                    RowLayout {
                        anchors.fill: parent
                        anchors.margins: 18
                        spacing: 16
                        Rectangle {
                            width: 48
                            height: 48
                            radius: 12
                            color: root.selectionSurface
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
                                color: root.hairline

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
                    height: 198
                    radius: 16
                    color: root.panel
                    border.color: root.hairline

                    ColumnLayout {
                        anchors.fill: parent
                        anchors.margins: 16
                        spacing: 12
                        Text { text: qsTr("INTERFACE"); color: root.tertiaryText; font.pixelSize: 11; font.weight: Font.Medium }
                        Item {
                            Layout.fillWidth: true
                            height: 52
                            Column {
                                anchors.left: parent.left
                                anchors.right: themeSelector.left
                                anchors.rightMargin: 24
                                anchors.verticalCenter: parent.verticalCenter
                                spacing: 2
                                Text { text: qsTr("Theme"); color: root.primaryText; font.pixelSize: 14; font.weight: Font.Medium }
                                Text { text: qsTr("Follow Plasma or use an explicit FluidVoice appearance."); color: root.secondaryText; font.pixelSize: 13 }
                            }
                            ComboBox {
                                id: themeSelector
                                anchors.right: parent.right
                                anchors.verticalCenter: parent.verticalCenter
                                width: 260
                                model: controller.themeOptions
                                currentIndex: controller.selectedTheme
                                onActivated: function(index) { controller.selectTheme(index) }
                            }
                        }
                        Rectangle { Layout.fillWidth: true; height: 1; color: root.hairline }
                        Item {
                            Layout.fillWidth: true
                            height: 52
                            Column {
                                anchors.left: parent.left
                                anchors.right: accentSelector.left
                                anchors.rightMargin: 24
                                anchors.verticalCenter: parent.verticalCenter
                                spacing: 2
                                Text { text: qsTr("Accent color"); color: root.primaryText; font.pixelSize: 14; font.weight: Font.Medium }
                                Text { text: qsTr("Use Plasma's accent or a FluidVoice color."); color: root.secondaryText; font.pixelSize: 13 }
                            }
                            ComboBox {
                                id: accentSelector
                                anchors.right: parent.right
                                anchors.verticalCenter: parent.verticalCenter
                                width: 260
                                model: controller.accentOptions
                                currentIndex: controller.selectedAccent
                                onActivated: function(index) { controller.selectAccent(index) }
                            }
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
                    implicitHeight: aiContent.implicitHeight + 40
                    radius: 16
                    color: root.panel
                    border.color: root.hairline

                    ColumnLayout {
                        id: aiContent; anchors.fill: parent; anchors.margins: 20; spacing: 14
                        RowLayout {
                            Layout.fillWidth: true
                            ColumnLayout { Layout.fillWidth: true; spacing: 3
                                Text { text: qsTr("Enhance dictated text"); color: root.primaryText; font.pixelSize: 15; font.weight: Font.DemiBold }
                                Text { text: qsTr("Optional cleanup after local speech recognition. Raw text is used if enhancement fails."); color: root.secondaryText; font.pixelSize: 12; wrapMode: Text.Wrap; Layout.fillWidth: true }
                            }
                            Switch { checked: controller.aiEnabled; onToggled: controller.updateAiEnabled(checked) }
                        }
                        Rectangle { Layout.fillWidth: true; height: 1; color: root.hairline }
                        RowLayout {
                            Layout.fillWidth: true
                            ColumnLayout {
                                Layout.fillWidth: true; spacing: 3
                                Text { text: qsTr("Local providers only"); color: root.primaryText; font.pixelSize: 14; font.weight: Font.DemiBold }
                                Text { Layout.fillWidth: true; text: qsTr("Prevent FluidVoice from sending transcripts or cleanup prompts to any network AI provider."); color: root.secondaryText; font.pixelSize: 12; wrapMode: Text.Wrap }
                            }
                            Switch { checked: controller.aiLocalOnly; onToggled: controller.updateAiLocalOnly(checked) }
                        }
                        Rectangle { Layout.fillWidth: true; height: 1; color: root.hairline }
                        Text { text: qsTr("PROVIDER"); color: root.tertiaryText; font.pixelSize: 11; font.weight: Font.Medium }
                        ComboBox {
                            Layout.fillWidth: true; model: controller.aiProviders; currentIndex: controller.selectedAiProvider
                            enabled: !controller.recording && !controller.transcribing
                            onActivated: function(index) { controller.selectAiProvider(index) }
                        }
                        Rectangle {
                            visible: controller.selectedAiProvider === 7
                            Layout.fillWidth: true
                            implicitHeight: ollamaSetup.implicitHeight + 32
                            radius: 12
                            color: root.panelRaised
                            border.color: root.hairline
                            ColumnLayout {
                                id: ollamaSetup; anchors.fill: parent; anchors.margins: 16; spacing: 10
                                RowLayout {
                                    Layout.fillWidth: true
                                    ColumnLayout { Layout.fillWidth: true; spacing: 2
                                        Text { text: qsTr("OLLAMA SETUP"); color: root.tertiaryText; font.pixelSize: 11; font.weight: Font.Medium }
                                        Text { Layout.fillWidth: true; text: qsTr("Install Ollama once, start its local server, then download a model without leaving FluidVoice."); color: root.secondaryText; font.pixelSize: 12; wrapMode: Text.Wrap }
                                    }
                                    BusyIndicator { running: controller.ollamaBusy; visible: running; implicitWidth: 30; implicitHeight: 30 }
                                }
                                Text { Layout.fillWidth: true; text: controller.ollamaStatus; color: root.primaryText; font.pixelSize: 12; wrapMode: Text.Wrap }
                                RowLayout {
                                    Layout.fillWidth: true
                                    Button { text: qsTr("Check setup"); enabled: !controller.ollamaBusy; onClicked: controller.diagnoseOllama() }
                                    Button { text: qsTr("Start server"); enabled: controller.ollamaInstalled && !controller.ollamaBusy; onClicked: controller.startOllama() }
                                    Button { text: qsTr("Official install guide"); enabled: !controller.ollamaBusy; onClicked: Qt.openUrlExternally("https://ollama.com/download/linux") }
                                    Item { Layout.fillWidth: true }
                                }
                                Rectangle { Layout.fillWidth: true; height: 1; color: root.hairline }
                                Text { text: qsTr("LOCAL CLEANUP MODEL"); color: root.tertiaryText; font.pixelSize: 11; font.weight: Font.Medium }
                                RowLayout {
                                    Layout.fillWidth: true
                                    TextField { id: ollamaModelToPull; Layout.fillWidth: true; text: "qwen2.5:7b"; placeholderText: qsTr("Ollama model name") }
                                    Button { text: controller.ollamaBusy ? qsTr("Working…") : qsTr("Download model"); enabled: controller.ollamaInstalled && !controller.ollamaBusy && ollamaModelToPull.text.trim().length > 0; onClicked: controller.pullOllamaModel(ollamaModelToPull.text) }
                                }
                                Text { Layout.fillWidth: true; text: qsTr("Models are downloaded by Ollama to this computer and may use several gigabytes. qwen2.5:7b is a balanced default; choose a smaller model if memory is limited."); color: root.tertiaryText; font.pixelSize: 11; wrapMode: Text.Wrap }
                            }
                        }
                        GridLayout {
                            Layout.fillWidth: true; columns: 2; columnSpacing: 14; rowSpacing: 10
                            Text { text: qsTr("Model"); color: root.secondaryText; font.pixelSize: 12 }
                            RowLayout {
                                Layout.fillWidth: true
                                TextField {
                                    visible: !controller.aiLocalEndpoint || controller.aiLocalModels.length === 0
                                    Layout.fillWidth: true; text: controller.aiModel
                                    onEditingFinished: controller.updateAiModel(text)
                                }
                                ComboBox {
                                    visible: controller.aiLocalEndpoint && controller.aiLocalModels.length > 0
                                    Layout.fillWidth: true; model: controller.aiLocalModels
                                    currentIndex: Math.max(0, controller.aiLocalModels.indexOf(controller.aiModel))
                                    onActivated: function(index) { controller.selectLocalAiModel(index) }
                                }
                                Button {
                                    visible: controller.aiLocalEndpoint
                                    text: controller.assistantBusy ? qsTr("Searching…") : qsTr("Find models")
                                    enabled: !controller.assistantBusy
                                    onClicked: controller.discoverLocalAiModels()
                                }
                            }
                            Text { text: qsTr("Base URL"); color: root.secondaryText; font.pixelSize: 12 }
                            TextField { Layout.fillWidth: true; text: controller.aiBaseUrl; onEditingFinished: controller.updateAiBaseUrl(text) }
                            Text { visible: !controller.aiLocalEndpoint; text: qsTr("API key"); color: root.secondaryText; font.pixelSize: 12 }
                            RowLayout {
                                visible: !controller.aiLocalEndpoint; Layout.fillWidth: true
                                TextField { id: aiApiKey; Layout.fillWidth: true; echoMode: TextInput.Password; placeholderText: controller.aiKeyConfigured ? qsTr("Stored securely · enter to replace") : qsTr("Required for this provider") }
                                Button { text: qsTr("Save key"); enabled: aiApiKey.text.length > 0; onClicked: { controller.saveAiApiKey(aiApiKey.text); aiApiKey.clear() } }
                            }
                        }
                        Rectangle {
                            Layout.fillWidth: true; implicitHeight: privacyText.implicitHeight + 24; radius: 10
                            color: controller.aiLocalEndpoint ? Qt.rgba(root.accent.r, root.accent.g, root.accent.b, 0.12) : Qt.rgba(0.85, 0.64, 0.25, 0.10)
                            border.color: controller.aiLocalEndpoint ? root.accent : "#d9a441"
                            Text {
                                id: privacyText; anchors.fill: parent; anchors.margins: 12
                                text: controller.aiLocalEndpoint ? qsTr("Fully local · Whisper audio transcription, the raw transcript, cleanup prompt, and enhanced text all remain on this computer.") : qsTr("Cloud processing · when enhancement is enabled, the cleanup prompt and raw transcript are sent to the selected provider. Microphone audio is never sent.")
                                color: controller.aiLocalEndpoint ? root.accent : "#d9a441"; font.pixelSize: 11; wrapMode: Text.Wrap
                            }
                        }
                        Rectangle {
                            Layout.fillWidth: true; implicitHeight: cleanupGuidance.implicitHeight + 24; radius: 10
                            color: root.panelRaised; border.color: root.hairline
                            ColumnLayout {
                                id: cleanupGuidance; anchors.fill: parent; anchors.margins: 12; spacing: 4
                                Text { text: qsTr("MODEL QUALITY GUIDANCE"); color: root.tertiaryText; font.pixelSize: 10; font.weight: Font.DemiBold }
                                Text {
                                    Layout.fillWidth: true; wrapMode: Text.Wrap; color: root.secondaryText; font.pixelSize: 11
                                    text: controller.aiLocalEndpoint
                                          ? qsTr("Small local models are faster but may miss punctuation, self-corrections, or less common languages. A capable 7B-class instruct model is the recommended baseline; larger models usually follow cleanup constraints more reliably but use more memory and add latency.")
                                          : qsTr("Cloud model quality and privacy vary by provider. FluidVoice sends only the cleanup prompt and transcript, uses raw text if enhancement fails, and never uploads microphone audio.")
                                }
                            }
                        }
                        Text { text: qsTr("APPLICATION PROFILE"); color: root.tertiaryText; font.pixelSize: 11; font.weight: Font.Medium }
                        ComboBox {
                            Layout.fillWidth: true; model: controller.aiProfileNames
                            currentIndex: controller.selectedAiProfile
                            onActivated: function(index) { controller.selectAiProfile(index) }
                        }
                        RowLayout {
                            Layout.fillWidth: true
                            ColumnLayout { Layout.fillWidth: true; spacing: 2
                                Text { text: qsTr("Automatic KWin profile selection"); color: root.primaryText; font.pixelSize: 12; font.weight: Font.Medium }
                                Text { Layout.fillWidth: true; text: qsTr("Opt in to the packaged KWin bridge. It reports only application class and window title over the local session bus—never document or keyboard content."); color: root.secondaryText; font.pixelSize: 11; wrapMode: Text.Wrap }
                            }
                            Switch { checked: controller.autoProfilesEnabled; onToggled: controller.updateAutoProfilesEnabled(checked) }
                        }
                        Text { Layout.fillWidth: true; text: qsTr("Active application: %1").arg(controller.activeApplication); color: root.secondaryText; font.pixelSize: 11; wrapMode: Text.Wrap }
                        RowLayout {
                            Layout.fillWidth: true
                            TextField { id: aiProfileName; Layout.fillWidth: true; placeholderText: qsTr("Application or workflow name"); text: controller.aiProfileName }
                            Button { text: qsTr("Save profile"); enabled: aiProfileName.text.trim().length > 0 && aiProfilePrompt.text.trim().length > 0; onClicked: controller.saveAiProfile(aiProfileName.text, aiProfilePrompt.text, aiProfileMatch.text) }
                            Button { text: qsTr("Delete"); enabled: controller.selectedAiProfile > 0; onClicked: controller.deleteAiProfile() }
                        }
                        TextField { id: aiProfileMatch; Layout.fillWidth: true; placeholderText: qsTr("Application match, e.g. org.kde.konsole, firefox"); text: controller.aiProfileMatch }
                        Text { Layout.fillWidth: true; text: qsTr("Comma-separated, case-insensitive fragments are matched against the KWin application class and title. Leave blank for a manual-only profile."); color: root.tertiaryText; font.pixelSize: 10; wrapMode: Text.Wrap }
                        TextArea {
                            id: aiProfilePrompt; Layout.fillWidth: true; implicitHeight: 100
                            placeholderText: qsTr("Profile-specific cleanup instructions")
                            text: controller.aiProfilePrompt; wrapMode: TextEdit.Wrap
                            background: Rectangle { color: root.panelRaised; border.color: root.hairline; radius: 8 }
                        }
                        Text { text: qsTr("CLEANUP PROMPT"); color: root.tertiaryText; font.pixelSize: 11; font.weight: Font.Medium }
                        TextArea {
                            Layout.fillWidth: true; implicitHeight: 150; text: controller.aiPrompt; wrapMode: TextEdit.Wrap
                            background: Rectangle { color: root.panelRaised; border.color: root.hairline; radius: 8 }
                            onEditingFinished: controller.updateAiPrompt(text)
                        }
                        Text { Layout.fillWidth: true; text: qsTr("The selected dictation language is always appended as a strict output-language contract. Custom prompts may use {{language_name}}, {{language_code}}, or {{language}}."); color: root.tertiaryText; font.pixelSize: 10; wrapMode: Text.Wrap }
                        RowLayout {
                            Layout.fillWidth: true
                            Text { Layout.fillWidth: true; text: controller.aiStatus; color: root.secondaryText; font.pixelSize: 11; wrapMode: Text.Wrap }
                            Button { text: controller.assistantBusy ? qsTr("Testing…") : qsTr("Verify provider"); enabled: !controller.assistantBusy; onClicked: controller.testAiProvider() }
                        }
                        Text { Layout.fillWidth: true; text: qsTr("Fluid Intelligence / Fluid-1 is not included. This implementation uses your selected standard or local provider and preserves raw-text fallback behavior."); color: root.tertiaryText; font.pixelSize: 11; wrapMode: Text.Wrap }
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
                            RowLayout {
                                Layout.fillWidth: true
                                Text { text: qsTr("SPOKEN → PREFERRED"); color: root.tertiaryText; font.pixelSize: 11; font.weight: Font.Medium }
                                Item { Layout.fillWidth: true }
                                ComboBox { id: dictionaryConflictMode; model: [qsTr("Merge · keep existing"), qsTr("Merge · overwrite matches"), qsTr("Replace all")]; currentIndex: 0 }
                                Button { text: qsTr("Import"); onClicked: dictionaryImportDialog.open() }
                                Button { text: qsTr("Export"); enabled: controller.dictionaryTerms.length > 0; onClicked: dictionaryExportDialog.open() }
                            }
                            RowLayout {
                                Layout.fillWidth: true
                                TextField { id: dictionarySpoken; Layout.fillWidth: true; placeholderText: qsTr("What Whisper usually hears") }
                                Text { text: "→"; color: root.secondaryText; font.pixelSize: 15 }
                                TextField { id: dictionaryPreferred; Layout.fillWidth: true; placeholderText: qsTr("Preferred word or phrase"); onAccepted: addDictionaryButton.clicked() }
                                Button { id: addDictionaryButton; text: qsTr("Add or update"); enabled: dictionarySpoken.text.trim().length > 0 && dictionaryPreferred.text.trim().length > 0; onClicked: { controller.addDictionaryReplacement(dictionarySpoken.text, dictionaryPreferred.text); dictionarySpoken.clear(); dictionaryPreferred.clear() } }
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
                            Text { Layout.fillWidth: true; text: qsTr("Replacements use case-insensitive whole-word or whole-phrase matching. Legacy one-column entries remain valid capitalization rules. CSV/TSV imports accept spoken and preferred columns; choose the conflict policy before importing."); color: root.secondaryText; font.pixelSize: 12; wrapMode: Text.Wrap }
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
                            Rectangle { Layout.fillWidth: true; height: 1; color: root.hairline }
                            Text { text: qsTr("COMMAND ASSISTANT"); color: root.tertiaryText; font.pixelSize: 11; font.weight: Font.Medium }
                            Text { Layout.fillWidth: true; text: qsTr("Ask the configured provider for KDE help, or request an allowlisted action: open settings, open terminal, open file manager, or lock screen. Every desktop action requires confirmation; arbitrary shell execution is never allowed."); color: root.secondaryText; font.pixelSize: 12; wrapMode: Text.Wrap }
                            RowLayout {
                                Layout.fillWidth: true
                                TextField { id: commandInput; Layout.fillWidth: true; placeholderText: qsTr("Ask or request a desktop action"); onAccepted: commandSubmit.clicked() }
                                Button { id: commandSubmit; text: controller.assistantBusy ? qsTr("Thinking…") : qsTr("Send"); enabled: !controller.assistantBusy && commandInput.text.trim().length > 0; onClicked: { controller.submitCommand(commandInput.text); commandInput.clear() } }
                            }
                            Rectangle {
                                Layout.fillWidth: true; implicitHeight: commandOutputText.implicitHeight + 24; radius: 10; color: root.panelRaised; border.color: root.hairline
                                Text { id: commandOutputText; anchors.fill: parent; anchors.margins: 12; text: controller.commandOutput; color: root.primaryText; font.pixelSize: 12; wrapMode: Text.Wrap }
                            }
                            RowLayout {
                                visible: controller.pendingCommand.length > 0; Layout.fillWidth: true
                                Text { Layout.fillWidth: true; text: qsTr("Confirm: %1").arg(controller.pendingCommand); color: "#d9a441"; font.pixelSize: 12; wrapMode: Text.Wrap }
                                Button { text: qsTr("Cancel"); onClicked: controller.cancelPendingCommand() }
                                Button { text: qsTr("Run action"); onClicked: controller.approvePendingCommand() }
                            }
                        }
                    }
                    Rectangle {
                        Layout.fillWidth: true; implicitHeight: rewriteContent.implicitHeight + 32; radius: 16; color: root.panel; border.color: root.hairline
                        ColumnLayout {
                            id: rewriteContent; anchors.fill: parent; anchors.margins: 16; spacing: 12
                            Text { text: qsTr("WRITE MODE"); color: root.tertiaryText; font.pixelSize: 11; font.weight: Font.Medium }
                            Text { Layout.fillWidth: true; text: qsTr("Press Ctrl+Alt+W anywhere to open Write Mode. Select text in another application, describe the result you want, and FluidVoice will replace it through the provider and model configured above. Plasma may request keyboard-control permission."); color: root.secondaryText; font.pixelSize: 12; wrapMode: Text.Wrap }
                            RowLayout {
                                Layout.fillWidth: true
                                Text { Layout.fillWidth: true; text: qsTr("Provider: %1 · Model: %2").arg(controller.aiProviders[controller.selectedAiProvider]).arg(controller.aiModel.length > 0 ? controller.aiModel : qsTr("provider default")); color: root.secondaryText; font.pixelSize: 11; elide: Text.ElideRight }
                                Text { text: qsTr("Ctrl Alt W"); color: root.primaryText; font.pixelSize: 11; font.family: "monospace" }
                            }
                            TextField { id: rewriteInstruction; Layout.fillWidth: true; placeholderText: qsTr("For example: Draft a friendly reply, or make my selection concise") }
                            RowLayout {
                                Layout.fillWidth: true
                                Button { text: controller.assistantBusy ? qsTr("Writing…") : qsTr("Create draft"); enabled: !controller.assistantBusy && rewriteInstruction.text.trim().length > 0; onClicked: controller.writeFromInstruction(rewriteInstruction.text) }
                                Button { text: controller.assistantBusy ? qsTr("Rewriting…") : qsTr("Rewrite selection"); enabled: !controller.assistantBusy && rewriteInstruction.text.trim().length > 0; onClicked: { rewriteDelay.instruction = rewriteInstruction.text; root.hide(); rewriteDelay.restart() } }
                                Item { Layout.fillWidth: true }
                                Button { text: qsTr("Retry"); enabled: !controller.assistantBusy && controller.writeModeRetryAvailable; onClicked: controller.retryWriteMode() }
                                Button { text: qsTr("Undo"); enabled: controller.lastRawText.length > 0; onClicked: controller.undoLastAi() }
                                Button { text: qsTr("Copy result"); enabled: controller.transcriptText.length > 0; onClicked: controller.copyLastResult(false) }
                            }
                            Rectangle {
                                visible: controller.transcriptText.length > 0
                                Layout.fillWidth: true; implicitHeight: writeResult.implicitHeight + 24; radius: 10; color: root.panelRaised; border.color: root.hairline
                                Text { id: writeResult; anchors.fill: parent; anchors.margins: 12; text: controller.transcriptText; color: root.primaryText; font.pixelSize: 12; wrapMode: Text.Wrap }
                            }
                            Text { Layout.fillWidth: true; text: controller.aiStatus; color: root.secondaryText; font.pixelSize: 11; wrapMode: Text.Wrap }
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
                            Text { text: qsTr("MEETING & LONG-RECORDING TRANSCRIPTION"); color: root.tertiaryText; font.pixelSize: 11; font.weight: Font.Medium }
                            Text { Layout.fillWidth: true; text: qsTr("Uses the active Whisper model and language. Long audio is decoded locally and processed in bounded 30-second segments with timestamps. Optional Sortformer diarization can label up to four speakers without changing ordinary dictation."); color: root.secondaryText; font.pixelSize: 13; wrapMode: Text.Wrap }
                            Rectangle {
                                Layout.fillWidth: true
                                implicitHeight: diarizationSetup.implicitHeight + 28
                                radius: 12
                                color: root.panelRaised
                                border.color: controller.diarizationEnabled ? root.accent : root.hairline
                                ColumnLayout {
                                    id: diarizationSetup
                                    anchors.fill: parent
                                    anchors.margins: 14
                                    spacing: 10
                                    RowLayout {
                                        Layout.fillWidth: true
                                        ColumnLayout {
                                            Layout.fillWidth: true
                                            spacing: 3
                                            RowLayout {
                                                Text { text: qsTr("Speaker diarization"); color: root.primaryText; font.pixelSize: 14; font.weight: Font.DemiBold }
                                                Rectangle {
                                                    implicitWidth: experimentalLabel.implicitWidth + 16; implicitHeight: 24; radius: 12
                                                    color: root.selectionSurface; border.color: root.accent
                                                    Text { id: experimentalLabel; anchors.centerIn: parent; text: qsTr("Experimental"); color: root.accent; font.pixelSize: 10; font.weight: Font.Medium }
                                                }
                                            }
                                            Text { Layout.fillWidth: true; text: qsTr("Identify who spoke when with local Sortformer v2. The regular Whisper transcript is always preserved if diarization fails."); color: root.secondaryText; font.pixelSize: 11; wrapMode: Text.Wrap }
                                        }
                                        Switch { checked: controller.diarizationEnabled; enabled: !controller.transcribing && !controller.sortformerBusy; onToggled: controller.updateDiarizationEnabled(checked) }
                                    }
                                    Text { Layout.fillWidth: true; text: controller.sortformerStatus; color: controller.sortformerInstalled ? root.accent : root.secondaryText; font.pixelSize: 11; wrapMode: Text.Wrap }
                                    RowLayout {
                                        visible: controller.sortformerBusy
                                        Layout.fillWidth: true
                                        BusyIndicator { running: controller.sortformerBusy; implicitWidth: 24; implicitHeight: 24 }
                                        Text { Layout.fillWidth: true; text: controller.sortformerDownloadProgress > 0 ? qsTr("Downloading and verifying the diarization model…") : qsTr("Building the shared native runtime…"); color: root.accent; font.pixelSize: 11; wrapMode: Text.Wrap }
                                    }
                                    ProgressBar { visible: controller.sortformerBusy && controller.sortformerDownloadProgress > 0; Layout.fillWidth: true; value: controller.sortformerDownloadProgress }
                                    RowLayout {
                                        Layout.fillWidth: true
                                        Text {
                                            Layout.fillWidth: true
                                            text: controller.sortformerInstalled
                                                ? qsTr("Verified model installed · maximum 4 speakers")
                                                : qsTr("140 MiB verified FluidVoice GGUF conversion · shared native runtime")
                                            color: root.tertiaryText; font.pixelSize: 10; wrapMode: Text.Wrap
                                        }
                                        Button {
                                            Layout.preferredWidth: 150
                                            text: controller.sortformerBusy && controller.sortformerDownloadProgress > 0 ? qsTr("Cancel") : controller.sortformerInstalled ? qsTr("Repair setup") : qsTr("Set up")
                                            enabled: !controller.sortformerBusy || controller.sortformerDownloadProgress > 0
                                            onClicked: controller.sortformerBusy ? controller.cancelSortformerDownload() : controller.setupSortformer()
                                        }
                                        Button { Layout.preferredWidth: 120; text: qsTr("Check setup"); enabled: !controller.sortformerBusy; onClicked: controller.diagnoseSortformer() }
                                        Button { visible: controller.sortformerInstalled; Layout.preferredWidth: 110; text: qsTr("Delete"); enabled: !controller.sortformerBusy && !controller.transcribing; onClicked: controller.deleteSortformer() }
                                    }
                                    Text { Layout.fillWidth: true; text: qsTr("Best suited to meetings and interviews. Accuracy may degrade with more than four speakers, overlapping speech, noise, long recordings, or non-English conversation. CPU and cross-vendor Vulkan are supported."); color: root.tertiaryText; font.pixelSize: 10; wrapMode: Text.Wrap }
                                }
                            }
                            RowLayout { Layout.fillWidth: true
                                Button { text: controller.transcribing ? qsTr("Transcribing…") : qsTr("Choose audio or video"); enabled: !controller.transcribing && !controller.recording; onClicked: audioFileDialog.open(); Accessible.name: qsTr("Choose audio or video for transcription") }
                                Button { visible: controller.lastMeetingFile.length > 0; text: qsTr("Retry last file"); enabled: !controller.transcribing && !controller.recording; onClicked: controller.retryMeetingTranscription(); Accessible.name: qsTr("Retry transcription with current settings") }
                                Button { visible: controller.transcribing; text: qsTr("Cancel"); onClicked: controller.cancelMeetingTranscription() }
                                Item { Layout.fillWidth: true }
                                ComboBox { id: meetingExportFormat; model: ["TXT", "MD", "SRT", "VTT", "JSON"]; currentIndex: 2 }
                                Button { text: qsTr("Export"); enabled: controller.meetingSegments.length > 0 && !controller.transcribing; onClicked: meetingExportDialog.open() }
                            }
                            Text { visible: controller.lastMeetingFile.length > 0; Layout.fillWidth: true; text: qsTr("Last file: %1").arg(controller.lastMeetingFile); color: root.tertiaryText; font.pixelSize: 10; elide: Text.ElideMiddle }
                            ProgressBar { visible: controller.transcribing || controller.meetingProgress > 0; Layout.fillWidth: true; from: 0; to: 1; value: controller.meetingProgress }
                            Text { Layout.fillWidth: true; text: controller.fileTranscriptionStatus; color: controller.transcribing ? root.accent : root.secondaryText; font.pixelSize: 12; wrapMode: Text.Wrap }
                            Rectangle {
                                visible: controller.meetingSpeakers.length > 0
                                Layout.fillWidth: true
                                implicitHeight: speakerNames.implicitHeight + 24
                                radius: 10; color: root.background; border.color: root.hairline
                                RowLayout {
                                    id: speakerNames
                                    anchors.fill: parent; anchors.margins: 12; spacing: 10
                                    ColumnLayout { Layout.fillWidth: true; spacing: 2
                                        Text { text: qsTr("Speaker names"); color: root.primaryText; font.pixelSize: 12; font.weight: Font.DemiBold }
                                        Text { Layout.fillWidth: true; text: qsTr("Rename a detected speaker for this result, History, and every export format."); color: root.secondaryText; font.pixelSize: 10; wrapMode: Text.Wrap }
                                    }
                                    ComboBox { id: meetingSpeakerSelector; Layout.preferredWidth: 150; model: controller.meetingSpeakers; Accessible.name: qsTr("Detected speaker") }
                                    TextField { id: meetingSpeakerName; Layout.preferredWidth: 180; placeholderText: qsTr("Speaker name"); maximumLength: 40; Accessible.name: qsTr("New speaker name") }
                                    Button {
                                        text: qsTr("Rename")
                                        enabled: meetingSpeakerSelector.currentIndex >= 0 && meetingSpeakerName.text.trim().length > 0
                                        onClicked: {
                                            controller.renameMeetingSpeaker(meetingSpeakerSelector.currentText, meetingSpeakerName.text)
                                            meetingSpeakerName.clear()
                                        }
                                        Accessible.name: qsTr("Rename selected speaker")
                                    }
                                }
                            }
                            ListView {
                                visible: controller.meetingSegments.length > 0
                                Layout.fillWidth: true; Layout.preferredHeight: Math.min(320, contentHeight); clip: true; spacing: 6
                                model: controller.meetingSegments
                                delegate: Rectangle {
                                    required property string modelData; required property int index
                                    width: ListView.view.width; height: segmentText.implicitHeight + 24; radius: 8; color: root.panelRaised; border.color: root.hairline
                                    RowLayout { anchors.fill: parent; anchors.margins: 10; spacing: 12
                                        Text { text: root.meetingTimestamp(root.meetingSegmentField(modelData, 0)); color: root.accent; font.pixelSize: 10; font.family: "monospace"; Layout.alignment: Qt.AlignTop }
                                        Text { text: root.meetingSegmentField(modelData, 2).length > 0 ? root.meetingSegmentField(modelData, 2) : qsTr("Speaker unassigned"); color: root.tertiaryText; font.pixelSize: 10; Layout.preferredWidth: 110; Layout.alignment: Qt.AlignTop }
                                        Text { id: segmentText; Layout.fillWidth: true; text: root.meetingSegmentField(modelData, 3); color: root.primaryText; font.pixelSize: 11; wrapMode: Text.Wrap }
                                    }
                                }
                            }
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
                        Button { text: qsTr("Export JSON"); enabled: controller.historyEntries.length > 0; onClicked: historyJsonDialog.open() }
                        Button { text: qsTr("Export CSV"); enabled: controller.historyEntries.length > 0; onClicked: historyCsvDialog.open() }
                        Button { text: controller.exportBusy ? qsTr("Exporting…") : qsTr("Export audio ZIP"); enabled: !controller.exportBusy && controller.audioHistoryStatus.indexOf("0 retained") < 0 && controller.audioHistoryStatus.indexOf("No retained") < 0; onClicked: audioHistoryZipDialog.open() }
                        Button { text: qsTr("Clear history"); enabled: controller.historyEntries.length > 0; onClicked: controller.clearHistory() }
                    }
                    Text { text: root.destinationDescriptions[6]; color: root.secondaryText; font.pixelSize: 14 }
                    Rectangle {
                        Layout.fillWidth: true; implicitHeight: audioHistorySettings.implicitHeight + 32; radius: 16; color: root.panel; border.color: root.hairline
                        ColumnLayout {
                            id: audioHistorySettings; anchors.fill: parent; anchors.margins: 16; spacing: 10
                            RowLayout {
                                Layout.fillWidth: true
                                ColumnLayout { Layout.fillWidth: true; spacing: 2
                                    Text { text: qsTr("Optional audio history"); color: root.primaryText; font.pixelSize: 14; font.weight: Font.DemiBold }
                                    Text { Layout.fillWidth: true; text: qsTr("Retain a local 16 kHz WAV copy of successful microphone dictations. Disabled by default; microphone audio is never uploaded."); color: root.secondaryText; font.pixelSize: 11; wrapMode: Text.Wrap }
                                }
                                Switch { id: audioHistorySwitch; checked: controller.audioHistoryEnabled; onToggled: controller.updateAudioHistory(checked, controller.audioHistoryBudgetMb) }
                            }
                            RowLayout {
                                Layout.fillWidth: true
                                Text { text: qsTr("Storage budget"); color: root.secondaryText; font.pixelSize: 12 }
                                Item { Layout.fillWidth: true }
                                ComboBox {
                                    model: [qsTr("100 MB"), qsTr("500 MB"), qsTr("1 GB"), qsTr("2.5 GB"), qsTr("5 GB"), qsTr("10 GB")]
                                    currentIndex: root.audioBudgetIndex()
                                    enabled: !controller.recording
                                    onActivated: function(index) { controller.updateAudioHistory(audioHistorySwitch.checked, [100, 500, 1000, 2500, 5000, 10000][index]) }
                                }
                            }
                            Text { Layout.fillWidth: true; text: controller.audioHistoryStatus; color: root.tertiaryText; font.pixelSize: 11; wrapMode: Text.Wrap }
                        }
                    }
                    TextField {
                        id: historySearch
                        Layout.fillWidth: true
                        placeholderText: qsTr("Search transcriptions…")
                        leftPadding: 14
                        rightPadding: 14
                        onTextChanged: root.cachedFilteredHistory = root.filteredHistory(text)
                    }
                    RowLayout {
                        Layout.fillWidth: true
                        Text { text: historySearch.text.length > 0 ? qsTr("%1 matching entries").arg(root.cachedFilteredHistory.length) : qsTr("%1 entries").arg(controller.historyEntries.length); color: root.secondaryText; font.pixelSize: 11 }
                        Item { Layout.fillWidth: true }
                        Text { visible: controller.historyEntries.length > 0; text: qsTr("Newest first"); color: root.tertiaryText; font.pixelSize: 11 }
                    }
                    Text { visible: controller.historyEntries.length === 0; text: qsTr("No transcripts yet. Completed dictation and file transcripts appear here."); color: root.secondaryText; font.pixelSize: 13 }
                    Text { visible: controller.historyEntries.length > 0 && root.cachedFilteredHistory.length === 0; text: qsTr("No results. Try a different search term."); color: root.secondaryText; font.pixelSize: 13 }
                    ListView {
                        id: historyList
                        Layout.fillWidth: true
                        Layout.preferredHeight: Math.max(360, settingsFlick.height - 430)
                        clip: true
                        spacing: 12
                        cacheBuffer: 240
                        boundsBehavior: Flickable.StopAtBounds
                        ScrollBar.vertical: ScrollBar { policy: ScrollBar.AsNeeded }
                        model: root.cachedFilteredHistory
                        delegate: Rectangle {
                            required property string modelData
                            width: ListView.view.width
                            height: historyContent.implicitHeight + 28
                            radius: 10
                            color: root.panel
                            border.color: root.hairline
                            ColumnLayout {
                                id: historyContent; anchors.fill: parent; anchors.margins: 14; spacing: 8
                                RowLayout {
                                    Layout.fillWidth: true
                                    Text { text: qsTr("FluidVoice Linux"); color: root.secondaryText; font.pixelSize: 11; font.weight: Font.DemiBold }
                                    Item { Layout.fillWidth: true }
                                    Text { text: root.historyRelativeTime(modelData); color: root.tertiaryText; font.pixelSize: 10 }
                                }
                                Text { Layout.fillWidth: true; text: root.historyText(modelData); color: root.primaryText; font.pixelSize: 13; wrapMode: Text.Wrap }
                                Rectangle {
                                    visible: root.historyAiStatus(modelData) === "enhanced" || root.historyAiStatus(modelData) === "fallback"
                                    Layout.fillWidth: true; implicitHeight: comparisonContent.implicitHeight + 24; radius: 8
                                    color: root.panelRaised; border.color: root.hairline
                                    ColumnLayout {
                                        id: comparisonContent; anchors.fill: parent; anchors.margins: 12; spacing: 7
                                        RowLayout {
                                            Layout.fillWidth: true
                                            Text { text: qsTr("AI CHANGES"); color: root.accent; font.pixelSize: 10; font.weight: Font.DemiBold }
                                            Item { Layout.fillWidth: true }
                                            Text { text: root.historyChangeSummary(modelData); color: root.tertiaryText; font.pixelSize: 10 }
                                        }
                                        Text { Layout.fillWidth: true; text: root.historyRawText(modelData) === root.historyText(modelData) ? qsTr("No textual differences—the raw transcript was retained.") : root.historyDiffHtml(modelData); textFormat: Text.RichText; color: root.secondaryText; font.pixelSize: 11; wrapMode: Text.Wrap }
                                        RowLayout {
                                            Layout.fillWidth: true
                                            Button { text: qsTr("Copy raw"); onClicked: controller.copyHistoryText(modelData, 0) }
                                            Button { text: qsTr("Copy final"); onClicked: controller.copyHistoryText(modelData, 1) }
                                            Button { text: qsTr("Copy both"); onClicked: controller.copyHistoryText(modelData, 2) }
                                            Item { Layout.fillWidth: true }
                                            Button { text: qsTr("Undo AI to clipboard"); onClicked: controller.copyHistoryText(modelData, 3) }
                                        }
                                    }
                                }
                                Text { Layout.fillWidth: true; text: root.historySource(modelData) + " · " + root.historyAiSummary(modelData) + " · " + root.historyCleanupMode(modelData); color: root.tertiaryText; font.pixelSize: 10; elide: Text.ElideRight }
                                RowLayout {
                                    visible: root.historyAudioPath(modelData).length > 0
                                    Layout.fillWidth: true
                                    Text { text: qsTr("Local recording retained"); color: root.accent; font.pixelSize: 10 }
                                    Item { Layout.fillWidth: true }
                                    Button { text: qsTr("Play recording"); onClicked: Qt.openUrlExternally("file://" + root.historyAudioPath(modelData)) }
                                    Button { text: qsTr("Delete recording"); onClicked: controller.deleteHistoryAudio(modelData) }
                                }
                                RowLayout {
                                    Layout.fillWidth: true
                                    Text { text: root.historyDate(modelData); color: root.secondaryText; font.pixelSize: 11 }
                                    Item { Layout.fillWidth: true }
                                    Text { text: qsTr("%1 words").arg(root.historyWords(modelData)); color: root.secondaryText; font.pixelSize: 11 }
                                }
                            }
                        }
                    }
                }

                ColumnLayout {
                    visible: root.settingsSection === 7
                    spacing: 14
                    Text { text: qsTr("Stats"); color: root.primaryText; font.pixelSize: 22; font.weight: Font.Bold }
                    Text { text: root.destinationDescriptions[7]; color: root.secondaryText; font.pixelSize: 14 }
                    Rectangle {
                        Layout.fillWidth: true; implicitHeight: todayStats.implicitHeight + 40; radius: 16; color: root.selectionSurface; border.color: root.accent
                        ColumnLayout {
                            id: todayStats; anchors.fill: parent; anchors.margins: 20; spacing: 14
                            RowLayout {
                                Layout.fillWidth: true
                                ColumnLayout { spacing: 3
                                    Text { text: qsTr("Today"); color: root.primaryText; font.pixelSize: 24; font.weight: Font.Bold }
                                    Text { text: root.todayHistoryWords() > 0 ? qsTr("Every dictated word adds up.") : qsTr("Ready when you are. Start dictating to save time."); color: root.secondaryText; font.pixelSize: 12 }
                                }
                                Item { Layout.fillWidth: true }
                                Rectangle { visible: root.currentStreak() > 0; implicitWidth: streakLabel.implicitWidth + 20; implicitHeight: 26; radius: 13; color: root.panelRaised; Text { id: streakLabel; anchors.centerIn: parent; text: qsTr("🔥 %1 day streak").arg(root.currentStreak()); color: root.primaryText; font.pixelSize: 11; font.weight: Font.DemiBold } }
                            }
                            RowLayout {
                                Layout.fillWidth: true; spacing: 26
                                ColumnLayout { Text { text: root.todayHistoryWords(); color: root.primaryText; font.pixelSize: 22; font.weight: Font.Bold } Text { text: qsTr("words"); color: root.secondaryText; font.pixelSize: 10 } }
                                Rectangle { width: 1; height: 34; color: root.hairline }
                                ColumnLayout { Text { text: root.timeSaved(root.todayHistoryWords()); color: root.primaryText; font.pixelSize: 22; font.weight: Font.Bold } Text { text: qsTr("estimated saved"); color: root.secondaryText; font.pixelSize: 10 } }
                                Rectangle { width: 1; height: 34; color: root.hairline }
                                ColumnLayout { Text { text: root.todayHistorySessions(); color: root.primaryText; font.pixelSize: 22; font.weight: Font.Bold } Text { text: qsTr("sessions"); color: root.secondaryText; font.pixelSize: 10 } }
                                Item { Layout.fillWidth: true }
                            }
                        }
                    }
                    RowLayout {
                        Layout.fillWidth: true; spacing: 12
                        Rectangle { Layout.fillWidth: true; height: 120; radius: 16; color: root.panel; border.color: root.hairline
                            Column { anchors.centerIn: parent; spacing: 6; Text { anchors.horizontalCenter: parent.horizontalCenter; text: root.timeSaved(controller.dictatedWordCount); color: root.primaryText; font.pixelSize: 30; font.weight: Font.Bold } Text { anchors.horizontalCenter: parent.horizontalCenter; text: qsTr("Time saved"); color: root.secondaryText; font.pixelSize: 13 } Text { anchors.horizontalCenter: parent.horizontalCenter; text: qsTr("Based on %1 WPM typing").arg(controller.typingWpm); color: root.tertiaryText; font.pixelSize: 10 } }
                        }
                        Rectangle { Layout.fillWidth: true; height: 120; radius: 16; color: root.panel; border.color: root.hairline
                            Column { anchors.centerIn: parent; spacing: 6; Text { anchors.horizontalCenter: parent.horizontalCenter; text: controller.dictatedWordCount; color: root.primaryText; font.pixelSize: 30; font.weight: Font.Bold } Text { anchors.horizontalCenter: parent.horizontalCenter; text: qsTr("Total words"); color: root.secondaryText; font.pixelSize: 13 } Text { anchors.horizontalCenter: parent.horizontalCenter; text: qsTr("+%1 today").arg(root.todayHistoryWords()); color: root.tertiaryText; font.pixelSize: 10 } }
                        }
                    }
                    RowLayout {
                        Layout.fillWidth: true; spacing: 12
                        Rectangle { Layout.fillWidth: true; height: 110; radius: 16; color: root.panel; border.color: root.hairline
                            Column { anchors.centerIn: parent; spacing: 6; Text { anchors.horizontalCenter: parent.horizontalCenter; text: root.currentStreak(); color: root.primaryText; font.pixelSize: 28; font.weight: Font.Bold } Text { anchors.horizontalCenter: parent.horizontalCenter; text: qsTr("Current streak"); color: root.secondaryText; font.pixelSize: 12 } Text { anchors.horizontalCenter: parent.horizontalCenter; text: qsTr("Best: %1 days").arg(root.bestStreak()); color: root.tertiaryText; font.pixelSize: 10 } }
                        }
                        Rectangle { Layout.fillWidth: true; height: 110; radius: 16; color: root.panel; border.color: root.hairline
                            Column { anchors.centerIn: parent; spacing: 6; Text { anchors.horizontalCenter: parent.horizontalCenter; text: controller.transcriptCount; color: root.primaryText; font.pixelSize: 28; font.weight: Font.Bold } Text { anchors.horizontalCenter: parent.horizontalCenter; text: qsTr("Transcriptions"); color: root.secondaryText; font.pixelSize: 12 } Text { anchors.horizontalCenter: parent.horizontalCenter; text: qsTr("Avg: %1 words each").arg(controller.transcriptCount > 0 ? Math.floor(controller.dictatedWordCount / controller.transcriptCount) : 0); color: root.tertiaryText; font.pixelSize: 10 } }
                        }
                    }
                    Rectangle {
                        Layout.fillWidth: true; implicitHeight: statsPreferences.implicitHeight + 28; radius: 16; color: root.panel; border.color: root.hairline
                        ColumnLayout { id: statsPreferences; anchors.fill: parent; anchors.margins: 14; spacing: 10
                            Text { text: qsTr("Statistics preferences"); color: root.primaryText; font.pixelSize: 13; font.weight: Font.Medium }
                            Text {
                                Layout.fillWidth: true
                                text: qsTr("Time saved compares your typing speed with an estimated 150 WPM dictation rate.")
                                color: root.secondaryText
                                font.pixelSize: 11
                                wrapMode: Text.Wrap
                            }
                            RowLayout { Layout.fillWidth: true; spacing: 24
                                RowLayout { Layout.fillWidth: true; spacing: 12
                                    Text { text: qsTr("Typing speed"); color: root.secondaryText; font.pixelSize: 11 }
                                    Item { Layout.fillWidth: true }
                                    SpinBox { id: typingWpmInput; from: 10; to: 250; value: controller.typingWpm; editable: true; onValueModified: controller.updateStatsPreferences(value, controller.skipWeekends) }
                                    Text { text: qsTr("WPM"); color: root.tertiaryText; font.pixelSize: 10 }
                                }
                                RowLayout { Layout.fillWidth: true; spacing: 12
                                    Text { Layout.fillWidth: true; text: qsTr("Weekends don't break streaks"); color: root.secondaryText; font.pixelSize: 11; horizontalAlignment: Text.AlignRight }
                                    Switch { checked: controller.skipWeekends; onToggled: controller.updateStatsPreferences(controller.typingWpm, checked) }
                                }
                            }
                        }
                    }
                    Text { text: qsTr("Last 30 days"); color: root.primaryText; font.pixelSize: 17; font.weight: Font.DemiBold; Layout.topMargin: 4 }
                    RowLayout {
                        Layout.fillWidth: true; spacing: 12
                        Rectangle { Layout.fillWidth: true; height: 108; radius: 16; color: root.panel; border.color: root.hairline
                            Column { anchors.centerIn: parent; spacing: 5
                                Text { anchors.horizontalCenter: parent.horizontalCenter; text: root.activityStats(30).activeDayCount; color: root.primaryText; font.pixelSize: 27; font.weight: Font.Bold }
                                Text { anchors.horizontalCenter: parent.horizontalCenter; text: qsTr("Active days"); color: root.secondaryText; font.pixelSize: 12 }
                                Text { anchors.horizontalCenter: parent.horizontalCenter; text: qsTr("%1 sessions").arg(root.activityStats(30).sessions); color: root.tertiaryText; font.pixelSize: 10 }
                            }
                        }
                        Rectangle { Layout.fillWidth: true; height: 108; radius: 16; color: root.panel; border.color: root.hairline
                            Column { anchors.centerIn: parent; spacing: 5
                                Text { anchors.horizontalCenter: parent.horizontalCenter; text: root.activityStats(30).dictations + " / " + root.activityStats(30).files; color: root.primaryText; font.pixelSize: 27; font.weight: Font.Bold }
                                Text { anchors.horizontalCenter: parent.horizontalCenter; text: qsTr("Dictation / files"); color: root.secondaryText; font.pixelSize: 12 }
                                Text { anchors.horizontalCenter: parent.horizontalCenter; text: qsTr("Source split"); color: root.tertiaryText; font.pixelSize: 10 }
                            }
                        }
                        Rectangle { Layout.fillWidth: true; height: 108; radius: 16; color: root.panel; border.color: root.hairline
                            Column { anchors.centerIn: parent; spacing: 5
                                Text { anchors.horizontalCenter: parent.horizontalCenter; text: root.activityStats(30).longest; color: root.primaryText; font.pixelSize: 27; font.weight: Font.Bold }
                                Text { anchors.horizontalCenter: parent.horizontalCenter; text: qsTr("Longest session"); color: root.secondaryText; font.pixelSize: 12 }
                                Text { anchors.horizontalCenter: parent.horizontalCenter; text: qsTr("%1 average words").arg(root.activityStats(30).average); color: root.tertiaryText; font.pixelSize: 10 }
                            }
                        }
                    }
                    Text { text: qsTr("AI enhancement"); color: root.primaryText; font.pixelSize: 17; font.weight: Font.DemiBold; Layout.topMargin: 4 }
                    Text { Layout.fillWidth: true; text: qsTr("Measured from saved raw and final transcripts. Enhancement rate reports usage, not objective writing accuracy."); color: root.secondaryText; font.pixelSize: 11; wrapMode: Text.Wrap }
                    RowLayout {
                        Layout.fillWidth: true; spacing: 12
                        Rectangle { Layout.fillWidth: true; height: 116; radius: 16; color: root.panel; border.color: root.hairline
                            Column { anchors.centerIn: parent; spacing: 5
                                Text { anchors.horizontalCenter: parent.horizontalCenter; text: root.cachedAiHistoryStats.total > 0 ? Math.round(root.cachedAiHistoryStats.enhanced * 100 / root.cachedAiHistoryStats.total) + "%" : "0%"; color: root.accent; font.pixelSize: 28; font.weight: Font.Bold }
                                Text { anchors.horizontalCenter: parent.horizontalCenter; text: qsTr("AI enhanced"); color: root.secondaryText; font.pixelSize: 12 }
                                Text { anchors.horizontalCenter: parent.horizontalCenter; text: qsTr("%1 of %2 dictations").arg(root.cachedAiHistoryStats.enhanced).arg(root.cachedAiHistoryStats.total); color: root.tertiaryText; font.pixelSize: 10 }
                            }
                        }
                        Rectangle { Layout.fillWidth: true; height: 116; radius: 16; color: root.panel; border.color: root.hairline
                            Column { anchors.centerIn: parent; spacing: 5
                                Text { anchors.horizontalCenter: parent.horizontalCenter; text: root.cachedAiHistoryStats.attempts > 0 ? Math.round(root.cachedAiHistoryStats.enhanced * 100 / root.cachedAiHistoryStats.attempts) + "%" : "—"; color: root.primaryText; font.pixelSize: 28; font.weight: Font.Bold }
                                Text { anchors.horizontalCenter: parent.horizontalCenter; text: qsTr("AI success rate"); color: root.secondaryText; font.pixelSize: 12 }
                                Text { anchors.horizontalCenter: parent.horizontalCenter; text: qsTr("%1 successful · %2 fallback").arg(root.cachedAiHistoryStats.enhanced).arg(root.cachedAiHistoryStats.fallback); color: root.tertiaryText; font.pixelSize: 10 }
                            }
                        }
                        Rectangle { Layout.fillWidth: true; height: 116; radius: 16; color: root.panel; border.color: root.hairline
                            Column { anchors.centerIn: parent; spacing: 5
                                Text { anchors.horizontalCenter: parent.horizontalCenter; text: root.cachedAiHistoryStats.latencyCount > 0 ? Math.round(root.cachedAiHistoryStats.latencyTotal / root.cachedAiHistoryStats.latencyCount) + " ms" : "—"; color: root.primaryText; font.pixelSize: 28; font.weight: Font.Bold }
                                Text { anchors.horizontalCenter: parent.horizontalCenter; text: qsTr("Average AI latency"); color: root.secondaryText; font.pixelSize: 12 }
                                Text { anchors.horizontalCenter: parent.horizontalCenter; text: qsTr("Successful and fallback attempts"); color: root.tertiaryText; font.pixelSize: 10 }
                            }
                        }
                    }
                    Rectangle {
                        Layout.fillWidth: true; implicitHeight: providerStats.implicitHeight + 32; radius: 16; color: root.panel; border.color: root.hairline
                        ColumnLayout { id: providerStats; anchors.fill: parent; anchors.margins: 16; spacing: 8
                            Text { text: qsTr("AI PROVIDERS & MODELS"); color: root.secondaryText; font.pixelSize: 11; font.weight: Font.DemiBold }
                            Text { Layout.fillWidth: true; text: root.aiProviderSummary(); color: root.primaryText; font.pixelSize: 12; lineHeight: 1.25; wrapMode: Text.Wrap }
                        }
                    }
                    Rectangle {
                        Layout.fillWidth: true; implicitHeight: editImpactContent.implicitHeight + 32; radius: 16; color: root.panel; border.color: root.hairline
                        ColumnLayout { id: editImpactContent; anchors.fill: parent; anchors.margins: 16; spacing: 8
                            Text { text: qsTr("AI EDITING IMPACT"); color: root.secondaryText; font.pixelSize: 11; font.weight: Font.DemiBold }
                            Text { Layout.fillWidth: true; text: root.aiEditImpact().attempts > 0
                                ? qsTr("%1 of %2 attempts changed the transcript · net %3 words · net %4 characters")
                                      .arg(root.aiEditImpact().changed).arg(root.aiEditImpact().attempts)
                                      .arg(root.aiEditImpact().wordDelta >= 0 ? "+" + root.aiEditImpact().wordDelta : root.aiEditImpact().wordDelta)
                                      .arg(root.aiEditImpact().characterDelta >= 0 ? "+" + root.aiEditImpact().characterDelta : root.aiEditImpact().characterDelta)
                                : qsTr("No AI edit impact has been recorded yet."); color: root.primaryText; font.pixelSize: 12; wrapMode: Text.Wrap }
                            Text { Layout.fillWidth: true; text: qsTr("These are deterministic text deltas, not a claim that the rewritten text is objectively better."); color: root.tertiaryText; font.pixelSize: 10; wrapMode: Text.Wrap }
                        }
                    }
                    Rectangle {
                        Layout.fillWidth: true; height: 170; radius: 16; color: root.panel; border.color: root.hairline
                        ColumnLayout {
                            anchors.fill: parent; anchors.margins: 16; spacing: 10
                            RowLayout { Layout.fillWidth: true
                                Text { Layout.fillWidth: true; text: qsTr("ACTIVITY · LAST %1 DAYS").arg(root.statsChartDays); color: root.secondaryText; font.pixelSize: 11; font.weight: Font.DemiBold }
                                Button { text: qsTr("7 days"); checkable: true; checked: root.statsChartDays === 7; onClicked: root.statsChartDays = 7 }
                                Button { text: qsTr("30 days"); checkable: true; checked: root.statsChartDays === 30; onClicked: root.statsChartDays = 30 }
                            }
                            RowLayout {
                                Layout.fillWidth: true; Layout.fillHeight: true; spacing: 8
                                Repeater {
                                    model: root.statsChartDays
                                    delegate: ColumnLayout {
                                        required property int index; Layout.fillWidth: true; Layout.fillHeight: true; spacing: 4
                                        Item { Layout.fillHeight: true }
                                        Text { visible: root.statsChartDays === 7; Layout.alignment: Qt.AlignHCenter; text: root.wordsOnDay(root.statsChartDays - 1 - index); color: root.secondaryText; font.pixelSize: 9 }
                                        Rectangle { Layout.alignment: Qt.AlignHCenter; width: root.statsChartDays === 7 ? 28 : 8; height: Math.max(3, root.wordsOnDay(root.statsChartDays - 1 - index) / root.maxDailyWords(root.statsChartDays) * 72); radius: 4; color: root.wordsOnDay(root.statsChartDays - 1 - index) > 0 ? root.accent : root.panelRaised }
                                        Text { visible: root.statsChartDays === 7; Layout.alignment: Qt.AlignHCenter; text: Qt.formatDateTime(new Date(new Date().setDate(new Date().getDate() - (root.statsChartDays - 1 - index))), "ddd"); color: root.secondaryText; font.pixelSize: 9 }
                                    }
                                }
                            }
                        }
                    }
                    Rectangle {
                        Layout.fillWidth: true; implicitHeight: milestoneContent.implicitHeight + 32; radius: 16; color: root.panel; border.color: root.hairline
                        ColumnLayout { id: milestoneContent; anchors.fill: parent; anchors.margins: 16; spacing: 10
                            Text { text: qsTr("MILESTONES"); color: root.secondaryText; font.pixelSize: 11; font.weight: Font.DemiBold }
                            Repeater {
                                model: [
                                    { "name": qsTr("Words"), "values": [1000, 10000, 100000], "current": controller.dictatedWordCount },
                                    { "name": qsTr("Transcriptions"), "values": [10, 100, 1000], "current": controller.transcriptCount },
                                    { "name": qsTr("Streak"), "values": [3, 7, 30], "current": root.bestStreak() }
                                ]
                                delegate: RowLayout {
                                    id: milestoneRow
                                    required property var modelData; Layout.fillWidth: true
                                    Text { Layout.preferredWidth: 110; text: modelData.name; color: root.secondaryText; font.pixelSize: 11 }
                                    Repeater { model: modelData.values
                                        delegate: Rectangle { required property int modelData; implicitWidth: milestoneLabel.implicitWidth + 18; implicitHeight: 26; radius: 7; color: milestoneRow.modelData.current >= modelData ? root.selectionSurface : root.panelRaised; border.color: milestoneRow.modelData.current >= modelData ? root.accent : root.hairline
                                            Text { id: milestoneLabel; anchors.centerIn: parent; text: (milestoneRow.modelData.current >= modelData ? "✓ " : "○ ") + modelData; color: milestoneRow.modelData.current >= modelData ? root.accent : root.secondaryText; font.pixelSize: 10 }
                                        }
                                    }
                                    Item { Layout.fillWidth: true }
                                }
                            }
                        }
                    }
                    Rectangle {
                        Layout.fillWidth: true; implicitHeight: recordsContent.implicitHeight + 32; radius: 16; color: root.panel; border.color: root.hairline
                        ColumnLayout { id: recordsContent; anchors.fill: parent; anchors.margins: 16; spacing: 10
                            Text { text: qsTr("INSIGHTS & PERSONAL RECORDS"); color: root.secondaryText; font.pixelSize: 11; font.weight: Font.DemiBold }
                            GridLayout { Layout.fillWidth: true; columns: 4; columnSpacing: 12
                                ColumnLayout { Layout.fillWidth: true; Text { text: root.personalRecords().peakHour; color: root.primaryText; font.pixelSize: 14; font.weight: Font.DemiBold } Text { text: qsTr("Peak time"); color: root.tertiaryText; font.pixelSize: 10 } }
                                ColumnLayout { Layout.fillWidth: true; Text { text: root.personalRecords().longest + " " + qsTr("words"); color: root.primaryText; font.pixelSize: 14; font.weight: Font.DemiBold } Text { text: qsTr("Longest transcription"); color: root.tertiaryText; font.pixelSize: 10 } }
                                ColumnLayout { Layout.fillWidth: true; Text { text: root.personalRecords().mostWords + " " + qsTr("words"); color: root.primaryText; font.pixelSize: 14; font.weight: Font.DemiBold } Text { text: qsTr("Most words in a day"); color: root.tertiaryText; font.pixelSize: 10 } }
                                ColumnLayout { Layout.fillWidth: true; Text { text: root.personalRecords().mostSessions; color: root.primaryText; font.pixelSize: 14; font.weight: Font.DemiBold } Text { text: qsTr("Most sessions in a day"); color: root.tertiaryText; font.pixelSize: 10 } }
                            }
                        }
                    }
                    Button { Layout.alignment: Qt.AlignHCenter; text: qsTr("Reset all statistics"); enabled: controller.transcriptCount > 0; onClicked: resetStatsDialog.open() }
                    Text { text: qsTr("Statistics are derived locally from History and never leave this computer."); color: root.secondaryText; font.pixelSize: 12 }
                }

                ColumnLayout {
                    visible: root.settingsSection === 8
                    spacing: 14
                    Text { text: qsTr("Getting Started"); color: root.primaryText; font.pixelSize: 22; font.weight: Font.Bold }
                    Text { Layout.fillWidth: true; text: qsTr("Set up reliable, private dictation on KDE Plasma in a few minutes."); color: root.secondaryText; font.pixelSize: 14; wrapMode: Text.Wrap }

                    Rectangle {
                        Layout.fillWidth: true; implicitHeight: firstRunContent.implicitHeight + 32; radius: 16; color: root.panel; border.color: root.hairline
                        ColumnLayout {
                            id: firstRunContent; anchors.fill: parent; anchors.margins: 16; spacing: 8
                            Text { text: qsTr("RECOMMENDED FIRST RUN"); color: root.tertiaryText; font.pixelSize: 11; font.weight: Font.Medium }
                            Text { Layout.fillWidth: true; text: qsTr("1. Select and test your microphone.\n2. Download a multilingual Whisper model—Base is a practical starting point for English; other languages may benefit from a larger model.\n3. Choose a fixed language for best short-dictation accuracy, or Automatic for mixed languages.\n4. Keep Automatic (Vulkan) selected for GPU acceleration with CPU fallback.\n5. Hold the global shortcut, speak naturally, then release to transcribe and paste."); color: root.secondaryText; font.pixelSize: 13; lineHeight: 1.25; wrapMode: Text.Wrap }
                            RowLayout {
                                Button { text: qsTr("Open Voice Engine"); onClicked: root.showSettingsSection(1) }
                                Button { text: qsTr("Run welcome guide"); onClicked: { controller.resetOnboarding(); root.onboardingStep = 0; onboardingDialog.open() } }
                            }
                        }
                    }

                    Text { text: qsTr("Setup checklist"); color: root.primaryText; font.pixelSize: 15; font.weight: Font.DemiBold }
                    Text { Layout.fillWidth: true; text: qsTr("Checks reflect saved configuration. Use Test input to verify capture; Plasma may still request shortcut approval."); color: root.secondaryText; font.pixelSize: 12; wrapMode: Text.Wrap }
                    Repeater {
                        model: [
                            { "title": qsTr("Microphone source selected"), "detail": controller.selectedInput >= 0 ? controller.microphoneName : qsTr("Choose and test an input in Voice Engine"), "done": controller.selectedInput >= 0 },
                            { "title": qsTr("Speech model ready"), "detail": controller.modelName, "done": controller.modelStates[controller.selectedModel] === "Downloaded" },
                            { "title": qsTr("Language and compute selected"), "detail": qsTr("%1 · %2").arg(controller.languages[controller.selectedLanguage]).arg(controller.computeBackends[controller.selectedComputeBackend]), "done": controller.selectedLanguage >= 0 && controller.selectedComputeBackend >= 0 },
                            { "title": qsTr("Shortcut preference selected"), "detail": qsTr("Hold %1 while speaking; Plasma controls final approval").arg(controller.shortcuts[controller.selectedShortcut]), "done": controller.selectedShortcut >= 0 },
                            { "title": qsTr("First dictation completed"), "detail": controller.transcriptCount > 0 ? qsTr("%1 transcript(s) saved locally").arg(controller.transcriptCount) : qsTr("Try dictating into a text field"), "done": controller.transcriptCount > 0 }
                        ]
                        delegate: Rectangle {
                            required property var modelData; Layout.fillWidth: true; height: 72; radius: 10; color: root.panel; border.color: root.hairline
                            RowLayout { anchors.fill: parent; anchors.margins: 14; spacing: 12
                                Rectangle { Layout.preferredWidth: 26; Layout.minimumWidth: 26; Layout.maximumWidth: 26; Layout.preferredHeight: 26; radius: 13; color: modelData.done ? "#234b3b" : root.panelRaised; border.color: modelData.done ? "#39755a" : root.hairline; Text { anchors.centerIn: parent; text: modelData.done ? "✓" : "·"; color: modelData.done ? "#70d59b" : root.secondaryText; font.pixelSize: 13 } }
                                ColumnLayout { Layout.fillWidth: true; spacing: 2; Text { Layout.fillWidth: true; text: modelData.title; color: root.primaryText; font.pixelSize: 14; font.weight: Font.Medium } Text { Layout.fillWidth: true; text: modelData.detail; color: root.secondaryText; font.pixelSize: 12; elide: Text.ElideRight } }
                            }
                        }
                    }

                    Rectangle {
                        Layout.fillWidth: true; implicitHeight: usageTips.implicitHeight + 32; radius: 16; color: root.panel; border.color: root.hairline
                        ColumnLayout {
                            id: usageTips; anchors.fill: parent; anchors.margins: 16; spacing: 8
                            Text { text: qsTr("KDE & WAYLAND TIPS"); color: root.tertiaryText; font.pixelSize: 11; font.weight: Font.Medium }
                            Text { Layout.fillWidth: true; text: qsTr("• Closing the settings window keeps FluidVoice available in the Plasma system tray.\n• Plasma owns the global shortcut and may ask for approval the first time.\n• If direct paste is unavailable in an application, the transcript remains recoverable on the clipboard.\n• Automatic compute uses Vulkan when a compatible GPU is available and safely falls back to CPU.\n• Models, history, and dictionary data stay under your standard XDG user-data directory."); color: root.secondaryText; font.pixelSize: 13; lineHeight: 1.25; wrapMode: Text.Wrap }
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
                            Text { text: qsTr("%1 · Intelligent cleanup preview").arg(controller.appVersion); color: root.primaryText; font.pixelSize: 15; font.weight: Font.DemiBold }
                            Text { text: qsTr("NEW"); color: root.accent; font.pixelSize: 11; font.weight: Font.DemiBold }
                            Text { Layout.fillWidth: true; text: qsTr("• Conservative language-aware cleanup for local and cloud providers\n• Deterministic spoken formatting before optional AI\n• Model quality, speed, memory, and privacy guidance\n• History/export metadata for policy, language, provider, latency, and fallback\n• Safer local-provider networking and private-state storage"); color: root.secondaryText; font.pixelSize: 13; lineHeight: 1.25; wrapMode: Text.Wrap }
                            Text { text: qsTr("ENGINEERING"); color: root.accent; font.pixelSize: 11; font.weight: Font.DemiBold; Layout.topMargin: 4 }
                            Text { Layout.fillWidth: true; text: qsTr("• Real controller module boundaries and responsive background I/O\n• Consolidated clipboard, runtime setup, provider, preferences, and WAV policies\n• Versioned settings migration with existing choices preserved\n• Expanded multilingual, streaming-audio, privacy, and packaging tests\n• Clean Clippy, RustSec, QML, AppStream, install, and release-package validation"); color: root.secondaryText; font.pixelSize: 13; lineHeight: 1.25; wrapMode: Text.Wrap }
                            RowLayout { Layout.fillWidth: true
                                Button { text: controller.updateBusy ? qsTr("Checking…") : qsTr("Check for updates"); enabled: !controller.updateBusy; onClicked: controller.checkForUpdates() }
                                Text { Layout.fillWidth: true; text: controller.updateStatus; color: root.secondaryText; font.pixelSize: 11; wrapMode: Text.Wrap }
                            }
                            Rectangle { Layout.fillWidth: true; height: 1; color: root.hairline; Layout.topMargin: 4; Layout.bottomMargin: 4 }
                            Text { text: qsTr("0.1.0 · Foundation preview"); color: root.primaryText; font.pixelSize: 14; font.weight: Font.DemiBold }
                            Text { Layout.fillWidth: true; text: qsTr("Established the Rust/CXX-Qt application shell, PipeWire microphone capture, local whisper.cpp transcription, KDE Wayland global shortcut, clipboard recovery, and the first macOS-inspired recording overlay."); color: root.secondaryText; font.pixelSize: 13; wrapMode: Text.Wrap }
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

                ColumnLayout {
                    visible: root.settingsSection === 11
                    spacing: 14
                    Text { text: qsTr("About & License"); color: root.primaryText; font.pixelSize: 22; font.weight: Font.Bold }
                    Text { Layout.fillWidth: true; text: root.destinationDescriptions[11]; color: root.secondaryText; font.pixelSize: 14; wrapMode: Text.Wrap }
                    Rectangle {
                        Layout.fillWidth: true; implicitHeight: legalContent.implicitHeight + 40; radius: 16; color: root.panel; border.color: root.hairline
                        ColumnLayout {
                            id: legalContent; anchors.fill: parent; anchors.margins: 20; spacing: 12
                            Text { text: qsTr("FLUIDVOICE LINUX %1").arg(controller.appVersion); color: root.tertiaryText; font.pixelSize: 11; font.weight: Font.Medium }
                            Text { Layout.fillWidth: true; text: qsTr("Copyright © 2026 David Bolin. Licensed under the GNU General Public License, version 3 only."); color: root.primaryText; font.pixelSize: 14; wrapMode: Text.Wrap }
                            Text { Layout.fillWidth: true; text: qsTr("This program comes with absolutely no warranty. You may copy, modify, and redistribute it under GPLv3. The complete license and corresponding source are available from the project repository."); color: root.secondaryText; font.pixelSize: 13; lineHeight: 1.2; wrapMode: Text.Wrap }
                            Rectangle { Layout.fillWidth: true; height: 1; color: root.hairline }
                            Text { Layout.fillWidth: true; text: qsTr("This is an independent, unofficial Linux implementation based on the GPLv3 FluidVoice project for macOS by Altic and the upstream contributors. It is not sponsored or endorsed by them. The application and tray artwork are credited to the upstream contributors; exact provenance is recorded in THIRD_PARTY_NOTICES.md."); color: root.secondaryText; font.pixelSize: 13; lineHeight: 1.2; wrapMode: Text.Wrap }
                            RowLayout {
                                Button { text: qsTr("Project source & GPLv3"); onClicked: Qt.openUrlExternally("https://github.com/davidkodar/fluidvoice-linux") }
                                Button { text: qsTr("Upstream FluidVoice"); onClicked: Qt.openUrlExternally("https://github.com/altic-dev/FluidVoice") }
                            }
                        }
                    }
                }
            }
        }
    }

    Window {
        id: overlay
        readonly property int listeningWidth: controller.selectedOverlaySize === 0 ? 300 : controller.selectedOverlaySize === 2 ? 560 : 380
        readonly property int listeningHeight: controller.selectedOverlaySize === 0
                                               ? (controller.overlayShowText ? 120 : 88)
                                               : controller.selectedOverlaySize === 2
                                                 ? (controller.overlayShowText ? 240 : 132)
                                                 : (controller.overlayShowText ? 184 : 116)
        width: controller.overlayResultAvailable ? Math.max(listeningWidth, 420) : listeningWidth
        height: controller.overlayResultAvailable
                ? (controller.overlayShowText ? 174 : 126)
                : listeningHeight
        x: Math.round((Screen.width - width) / 2)
        y: controller.selectedOverlayPosition === 1 ? Screen.height - height - 54 : controller.selectedOverlayPosition === 2 ? Math.round((Screen.height - height) / 2) : 42
        visible: controller.overlayVisible
        color: "transparent"
        opacity: controller.overlayOpacity
        transientParent: null
        modality: Qt.NonModal
        flags: Qt.Tool | Qt.FramelessWindowHint | Qt.WindowStaysOnTopHint
               | (controller.overlayResultAvailable ? 0 : Qt.WindowDoesNotAcceptFocus)
        title: qsTr("FluidVoice Recording")
        property string animatedTranscript: ""
        property string targetTranscript: ""

        HoverHandler { id: overlayHover }

        Timer {
            id: overlayResultTimer
            interval: 15000
            onTriggered: overlayHover.hovered ? restart() : controller.dismissOverlay()
        }

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
            function onOverlayResultAvailableChanged() {
                if (controller.overlayResultAvailable)
                    overlayResultTimer.restart()
                else
                    overlayResultTimer.stop()
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
            radius: 20
            color: "#f51a1b1f"
            border.color: controller.recording ? root.accent : "#3dffffff"
            border.width: 1

            ColumnLayout {
                anchors.fill: parent
                anchors.margins: 16
                spacing: 12

                RowLayout {
                    Layout.fillWidth: true
                    Text {
                        text: controller.recording ? qsTr("Listening") : controller.transcribing ? (controller.aiEnabled ? qsTr("Transcribing and improving") : qsTr("Transcribing")) : qsTr("Dictation ready")
                        color: controller.recording ? root.accent : "#f2f2f2"
                        font.pixelSize: 14
                        font.weight: Font.DemiBold
                    }
                    Item { Layout.fillWidth: true }
                    Text {
                        visible: !controller.overlayResultAvailable
                        text: controller.recording ? qsTr("Release to finish") : controller.transcribing ? qsTr("Processing") : "Ctrl Alt D"
                        color: "#8d8d92"
                        font.pixelSize: 11
                    }
                    Button {
                        visible: controller.overlayResultAvailable
                        flat: true
                        text: "⋯"
                        font.pixelSize: 20
                        Layout.preferredWidth: 30
                        Layout.preferredHeight: 28
                        ToolTip.visible: hovered
                        ToolTip.text: qsTr("Last dictation actions")
                        onClicked: resultActions.open()

                        Menu {
                            id: resultActions
                            y: parent.height
                            MenuItem { text: qsTr("Copy last transcription"); onTriggered: controller.copyLastResult(false) }
                            MenuItem { text: qsTr("Copy raw transcription"); visible: controller.aiEnabled; enabled: controller.lastRawText.length > 0; onTriggered: controller.copyLastResult(true) }
                            MenuSeparator { visible: controller.aiEnabled }
                            MenuItem { text: qsTr("Undo AI on last"); visible: controller.aiEnabled; enabled: controller.lastRawText.length > 0 && controller.lastRawText !== controller.transcriptText; onTriggered: controller.undoLastAi() }
                            MenuItem { text: qsTr("Reprocess last dictation"); visible: controller.aiEnabled; enabled: controller.lastRawText.length > 0 && !controller.transcribing; onTriggered: controller.retryLastAi() }
                            MenuSeparator { }
                            MenuItem { text: qsTr("Dismiss"); onTriggered: controller.dismissOverlay() }
                        }
                    }
                }

                Text {
                    visible: controller.overlayShowText
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    Layout.minimumHeight: controller.overlayResultAvailable ? 42 : 0
                    text: overlay.animatedTranscript.length > 0
                          ? overlay.animatedTranscript
                          : qsTr("Speak naturally — text will appear here")
                    color: overlay.animatedTranscript.length > 0 ? "#eeeeef" : "#77777c"
                    font.pixelSize: controller.overlayResultAvailable ? 15 : 13
                    font.weight: controller.overlayResultAvailable ? Font.Medium : Font.Normal
                    lineHeight: 1.18
                    wrapMode: Text.Wrap
                    elide: Text.ElideRight
                    maximumLineCount: controller.selectedOverlaySize === 2 ? 4 : 3
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
                    Item { Layout.preferredWidth: 22; Layout.preferredHeight: 22 }
                }
            }
        }
    }
}
