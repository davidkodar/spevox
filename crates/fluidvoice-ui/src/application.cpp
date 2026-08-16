#include "fluidvoice-ui/src/application.h"

#include <QDir>
#include <QIcon>
#include <QLocalSocket>
#include <QStandardPaths>
#include <QWindow>

namespace fluidvoice {

FluidVoiceApplication::FluidVoiceApplication(int &argc, char **argv)
    : QApplication(argc, argv) {
    setApplicationName("FluidVoice");
    setApplicationDisplayName("FluidVoice");
    setOrganizationName("FluidVoice Linux");
    setDesktopFileName("io.github.davidkodar.FluidVoiceLinux");
    setWindowIcon(QIcon(":/qt/qml/io/github/davidkodar/FluidVoiceLinux/assets/fluidvoice-app.png"));
    setQuitOnLastWindowClosed(false);
    QString runtimeDirectory = QStandardPaths::writableLocation(QStandardPaths::RuntimeLocation);
    if (runtimeDirectory.isEmpty()) {
        runtimeDirectory = QDir::tempPath();
    }
    instanceLock = std::make_unique<QLockFile>(runtimeDirectory + "/fluidvoice-linux.lock");
    instanceLock->setStaleLockTime(1000);
    primaryInstance = instanceLock->tryLock(0);
    if (!primaryInstance && instanceLock->removeStaleLockFile()) {
        primaryInstance = instanceLock->tryLock(0);
    }
    const QString activationName = "fluidvoice-linux-activation";
    if (primaryInstance) {
        QLocalServer::removeServer(activationName);
        activationServer = std::make_unique<QLocalServer>();
        connect(activationServer.get(), &QLocalServer::newConnection, this, [this]() {
            while (QLocalSocket *socket = activationServer->nextPendingConnection()) {
                connect(socket, &QLocalSocket::disconnected, socket, &QObject::deleteLater);
                socket->disconnectFromServer();
            }
            for (QWindow *window : topLevelWindows()) {
                if (window->title() != "FluidVoice") {
                    continue;
                }
                window->show();
                window->raise();
                window->requestActivate();
            }
        });
        activationServer->listen(activationName);
    } else {
        QLocalSocket socket;
        socket.connectToServer(activationName, QIODevice::WriteOnly);
        if (socket.waitForConnected(500)) {
            socket.write("activate");
            socket.flush();
            socket.waitForBytesWritten(500);
            socket.disconnectFromServer();
        }
    }
}

bool FluidVoiceApplication::isPrimaryInstance() const { return primaryInstance; }

std::unique_ptr<FluidVoiceApplication> newApplication() {
    static int argc = 1;
    static char applicationName[] = "fluidvoice-ui";
    static char *argv[] = {applicationName, nullptr};

    return std::make_unique<FluidVoiceApplication>(argc, argv);
}

int execApplication(FluidVoiceApplication &application) {
    return application.exec();
}

bool isPrimaryInstance(const FluidVoiceApplication &application) {
    return application.isPrimaryInstance();
}

} // namespace fluidvoice
