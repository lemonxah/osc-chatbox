import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import Qt.labs.settings 1.0
import osc.chatbox 1.0

ApplicationWindow {
    id: root
    visible: true
    width: settings.windowWidth
    height: settings.windowHeight
    x: settings.windowX
    y: settings.windowY
    title: "OSC Chatbox"
    color: "#1a1a2e"

    onWidthChanged: settings.windowWidth = width
    onHeightChanged: settings.windowHeight = height
    onXChanged: settings.windowX = x
    onYChanged: settings.windowY = y

    Settings {
        id: settings
        category: "osc_chatbox"
        property int windowWidth: 480
        property int windowHeight: 720
        property int windowX: 100
        property int windowY: 100

        property string oscAddress: "127.0.0.1:9000"
        property bool statusEnabled: false
        property string statusLine1: ""
        property string statusLine2: ""
        property string statusLine3: ""
        property string statusLine4: ""
        property string statusLine5: ""
        property string statusLine6: ""
        property bool timeEnabled: false
        property string timeFormat: "%H:%M"
        property bool statsEnabled: false
        property bool statsShowCpu: true
        property bool statsShowRam: true
        property bool networkEnabled: false
        property bool mediaEnabled: false
        property bool afkEnabled: false
        property int afkTimeoutSecs: 300
        property string afkText: "AFK"
        property bool systemDetailsEnabled: false
        property bool heartrateEnabled: false
        property string heartrateToken: ""
    }

    property bool loading: true

    ChatboxController {
        id: controller
    }

    function loadSettings() {
        loading = true
        controller.osc_address = settings.oscAddress
        controller.status_enabled = settings.statusEnabled
        controller.status_line1 = settings.statusLine1
        controller.status_line2 = settings.statusLine2
        controller.status_line3 = settings.statusLine3
        controller.status_line4 = settings.statusLine4
        controller.status_line5 = settings.statusLine5
        controller.status_line6 = settings.statusLine6
        controller.time_enabled = settings.timeEnabled
        controller.time_format = settings.timeFormat
        controller.stats_enabled = settings.statsEnabled
        controller.stats_show_cpu = settings.statsShowCpu
        controller.stats_show_ram = settings.statsShowRam
        controller.network_enabled = settings.networkEnabled
        controller.media_enabled = settings.mediaEnabled
        controller.afk_enabled = settings.afkEnabled
        controller.afk_timeout_secs = settings.afkTimeoutSecs
        controller.afk_text = settings.afkText
        controller.system_details_enabled = settings.systemDetailsEnabled
        controller.heartrate_enabled = settings.heartrateEnabled
        controller.heartrate_token = settings.heartrateToken
        loading = false
    }

    Component.onCompleted: {
        loadSettings()
    }

    function saveSettings() {
        if (loading) return
        settings.oscAddress = controller.osc_address
        settings.statusEnabled = controller.status_enabled
        settings.statusLine1 = controller.status_line1
        settings.statusLine2 = controller.status_line2
        settings.statusLine3 = controller.status_line3
        settings.statusLine4 = controller.status_line4
        settings.statusLine5 = controller.status_line5
        settings.statusLine6 = controller.status_line6
        settings.timeEnabled = controller.time_enabled
        settings.timeFormat = controller.time_format
        settings.statsEnabled = controller.stats_enabled
        settings.statsShowCpu = controller.stats_show_cpu
        settings.statsShowRam = controller.stats_show_ram
        settings.networkEnabled = controller.network_enabled
        settings.mediaEnabled = controller.media_enabled
        settings.afkEnabled = controller.afk_enabled
        settings.afkTimeoutSecs = controller.afk_timeout_secs
        settings.afkText = controller.afk_text
        settings.systemDetailsEnabled = controller.system_details_enabled
        settings.heartrateEnabled = controller.heartrate_enabled
        settings.heartrateToken = controller.heartrate_token
    }

    Timer {
        id: tickTimer
        interval: 1500
        repeat: true
        running: controller.running
        onTriggered: controller.tick()
    }

    ScrollView {
        anchors.fill: parent
        anchors.margins: 12
        contentWidth: availableWidth

        ColumnLayout {
            width: parent.width
            spacing: 10

            Label {
                text: "OSC Chatbox"
                font.pixelSize: 22
                font.bold: true
                color: "#e0e0e0"
                Layout.alignment: Qt.AlignHCenter
            }

            GroupBox {
                title: "Connection"
                Layout.fillWidth: true
                background: Rectangle { color: "transparent" }
                label: Label { text: parent.title; color: "#e94560"; font.bold: true; padding: 4 }

                ColumnLayout {
                    width: parent.width
                    spacing: 6

                    RowLayout {
                        Label { text: "OSC Address:"; color: "#c0c0c0"; Layout.preferredWidth: 100 }
                        TextField {
                            id: oscAddr
                            text: controller.osc_address
                            placeholderText: "127.0.0.1:9000"
                            Layout.fillWidth: true
                            color: "#e0e0e0"
                            background: Rectangle { color: "#0f3460"; radius: 4 }
                            onTextChanged: { controller.osc_address = text; saveSettings() }
                        }
                    }

                    RowLayout {
                        spacing: 8
                        Button {
                            text: controller.running ? "Stop" : "Start"
                            highlighted: controller.running
                            onClicked: {
                                if (controller.running) {
                                    controller.stop()
                                } else {
                                    loadSettings()
                                    controller.start()
                                }
                            }
                        }
                        Rectangle {
                            width: 12; height: 12; radius: 6
                            color: controller.running ? "#00ff88" : "#ff4444"
                        }
                        Label {
                            text: controller.running ? "Connected" : "Disconnected"
                            color: "#c0c0c0"
                        }
                    }
                }
            }

            GroupBox {
                title: "Chat"
                Layout.fillWidth: true
                background: Rectangle { color: "transparent" }
                label: Label { text: parent.title; color: "#e94560"; font.bold: true; padding: 4 }

                ColumnLayout {
                    width: parent.width
                    spacing: 6

                    RowLayout {
                        TextField {
                            id: chatInput
                            placeholderText: "Type a message..."
                            Layout.fillWidth: true
                            color: "#e0e0e0"
                            background: Rectangle { color: "#0f3460"; radius: 4 }
                            onTextChanged: {
                                if (controller.running) {
                                    controller.setTyping(text.length > 0)
                                }
                            }
                            Keys.onReturnPressed: sendBtn.clicked()
                        }
                        Button {
                            id: sendBtn
                            text: "Send"
                            onClicked: {
                                if (chatInput.text.length > 0) {
                                    controller.sendMessage(chatInput.text)
                                    chatInput.text = ""
                                }
                            }
                        }
                    }
                }
            }

            GroupBox {
                title: "Personal Status"
                Layout.fillWidth: true
                background: Rectangle { color: "transparent" }
                label: Label { text: parent.title; color: "#e94560"; font.bold: true; padding: 4 }

                ColumnLayout {
                    width: parent.width
                    spacing: 6

                    Switch {
                        text: "Enabled"
                        checked: controller.status_enabled
                        onCheckedChanged: { controller.status_enabled = checked; controller.applySettings(); saveSettings() }
                        palette.text: "#c0c0c0"
                    }
                    TextField {
                        placeholderText: "Status line 1..."
                        text: controller.status_line1
                        Layout.fillWidth: true
                        color: "#e0e0e0"
                        background: Rectangle { color: "#0f3460"; radius: 4 }
                        onTextChanged: { controller.status_line1 = text; controller.applySettings(); saveSettings() }
                    }
                    TextField {
                        placeholderText: "Status line 2..."
                        text: controller.status_line2
                        Layout.fillWidth: true
                        color: "#e0e0e0"
                        background: Rectangle { color: "#0f3460"; radius: 4 }
                        onTextChanged: { controller.status_line2 = text; controller.applySettings(); saveSettings() }
                    }
                    TextField {
                        placeholderText: "Status line 3..."
                        text: controller.status_line3
                        Layout.fillWidth: true
                        color: "#e0e0e0"
                        background: Rectangle { color: "#0f3460"; radius: 4 }
                        onTextChanged: { controller.status_line3 = text; controller.applySettings(); saveSettings() }
                    }
                    TextField {
                        placeholderText: "Status line 4..."
                        text: controller.status_line4
                        Layout.fillWidth: true
                        color: "#e0e0e0"
                        background: Rectangle { color: "#0f3460"; radius: 4 }
                        onTextChanged: { controller.status_line4 = text; controller.applySettings(); saveSettings() }
                    }
                    TextField {
                        placeholderText: "Status line 5..."
                        text: controller.status_line5
                        Layout.fillWidth: true
                        color: "#e0e0e0"
                        background: Rectangle { color: "#0f3460"; radius: 4 }
                        onTextChanged: { controller.status_line5 = text; controller.applySettings(); saveSettings() }
                    }
                    TextField {
                        placeholderText: "Status line 6..."
                        text: controller.status_line6
                        Layout.fillWidth: true
                        color: "#e0e0e0"
                        background: Rectangle { color: "#0f3460"; radius: 4 }
                        onTextChanged: { controller.status_line6 = text; controller.applySettings(); saveSettings() }
                    }
                }
            }

            GroupBox {
                title: "Time"
                Layout.fillWidth: true
                background: Rectangle { color: "transparent" }
                label: Label { text: parent.title; color: "#e94560"; font.bold: true; padding: 4 }

                ColumnLayout {
                    width: parent.width
                    spacing: 6

                    Switch {
                        text: "Enabled"
                        checked: controller.time_enabled
                        onCheckedChanged: { controller.time_enabled = checked; controller.applySettings(); saveSettings() }
                        palette.text: "#c0c0c0"
                    }
                    RowLayout {
                        Label { text: "Format:"; color: "#c0c0c0"; Layout.preferredWidth: 100 }
                        TextField {
                            text: controller.time_format
                            Layout.fillWidth: true
                            color: "#e0e0e0"
                            background: Rectangle { color: "#0f3460"; radius: 4 }
                            onTextChanged: { controller.time_format = text; controller.applySettings(); saveSettings() }
                        }
                    }
                }
            }

            GroupBox {
                title: "System Stats"
                Layout.fillWidth: true
                background: Rectangle { color: "transparent" }
                label: Label { text: parent.title; color: "#e94560"; font.bold: true; padding: 4 }

                ColumnLayout {
                    width: parent.width
                    spacing: 6

                    Switch {
                        text: "Enabled"
                        checked: controller.stats_enabled
                        onCheckedChanged: { controller.stats_enabled = checked; controller.applySettings(); saveSettings() }
                        palette.text: "#c0c0c0"
                    }
                    RowLayout {
                        CheckBox {
                            text: "CPU"
                            checked: controller.stats_show_cpu
                            onCheckedChanged: { controller.stats_show_cpu = checked; controller.applySettings(); saveSettings() }
                            palette.text: "#c0c0c0"
                        }
                        CheckBox {
                            text: "RAM"
                            checked: controller.stats_show_ram
                            onCheckedChanged: { controller.stats_show_ram = checked; controller.applySettings(); saveSettings() }
                            palette.text: "#c0c0c0"
                        }
                    }
                }
            }

            GroupBox {
                title: "Network Stats"
                Layout.fillWidth: true
                background: Rectangle { color: "transparent" }
                label: Label { text: parent.title; color: "#e94560"; font.bold: true; padding: 4 }

                Switch {
                    text: "Enabled"
                    checked: controller.network_enabled
                    onCheckedChanged: { controller.network_enabled = checked; controller.applySettings(); saveSettings() }
                    palette.text: "#c0c0c0"
                }
            }

            GroupBox {
                title: "System Details"
                Layout.fillWidth: true
                background: Rectangle { color: "transparent" }
                label: Label { text: parent.title; color: "#e94560"; font.bold: true; padding: 4 }

                Switch {
                    text: "Enabled"
                    checked: controller.system_details_enabled
                    onCheckedChanged: { controller.system_details_enabled = checked; controller.applySettings(); saveSettings() }
                    palette.text: "#c0c0c0"
                }
            }

            GroupBox {
                title: "Media / Music"
                Layout.fillWidth: true
                background: Rectangle { color: "transparent" }
                label: Label { text: parent.title; color: "#e94560"; font.bold: true; padding: 4 }

                Switch {
                    text: "Enabled"
                    checked: controller.media_enabled
                    onCheckedChanged: { controller.media_enabled = checked; controller.applySettings(); saveSettings() }
                    palette.text: "#c0c0c0"
                }
            }

            GroupBox {
                title: "AFK Detection"
                Layout.fillWidth: true
                background: Rectangle { color: "transparent" }
                label: Label { text: parent.title; color: "#e94560"; font.bold: true; padding: 4 }

                ColumnLayout {
                    width: parent.width
                    spacing: 6

                    Switch {
                        text: "Enabled"
                        checked: controller.afk_enabled
                        onCheckedChanged: { controller.afk_enabled = checked; controller.applySettings(); saveSettings() }
                        palette.text: "#c0c0c0"
                    }
                    RowLayout {
                        Label { text: "Timeout (s):"; color: "#c0c0c0"; Layout.preferredWidth: 100 }
                        SpinBox {
                            value: controller.afk_timeout_secs
                            from: 30
                            to: 3600
                            stepSize: 30
                            onValueChanged: { controller.afk_timeout_secs = value; controller.applySettings(); saveSettings() }
                        }
                    }
                    RowLayout {
                        Label { text: "AFK Text:"; color: "#c0c0c0"; Layout.preferredWidth: 100 }
                        TextField {
                            text: controller.afk_text
                            Layout.fillWidth: true
                            color: "#e0e0e0"
                            background: Rectangle { color: "#0f3460"; radius: 4 }
                            onTextChanged: { controller.afk_text = text; controller.applySettings(); saveSettings() }
                        }
                    }
                }
            }

            GroupBox {
                title: "Heart Rate (Pulsoid)"
                Layout.fillWidth: true
                background: Rectangle { color: "transparent" }
                label: Label { text: parent.title; color: "#e94560"; font.bold: true; padding: 4 }

                ColumnLayout {
                    width: parent.width
                    spacing: 6

                    Switch {
                        text: "Enabled"
                        checked: controller.heartrate_enabled
                        onCheckedChanged: { controller.heartrate_enabled = checked; controller.applySettings(); saveSettings() }
                        palette.text: "#c0c0c0"
                    }
                    RowLayout {
                        Label { text: "Token:"; color: "#c0c0c0"; Layout.preferredWidth: 100 }
                        TextField {
                            text: controller.heartrate_token
                            echoMode: TextInput.Password
                            Layout.fillWidth: true
                            color: "#e0e0e0"
                            background: Rectangle { color: "#0f3460"; radius: 4 }
                            onTextChanged: { controller.heartrate_token = text; controller.applySettings(); saveSettings() }
                        }
                    }
                }
            }

            Item { Layout.fillHeight: true }
        }
    }
}
