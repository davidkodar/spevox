#include "fluidvoice-ui/src/application.h"

#include <QDir>
#include <QDebug>
#include <QFile>
#include <QIcon>
#include <QLocalSocket>
#include <QStandardPaths>
#include <QWindow>
#include <vector>

namespace fluidvoice {

FluidVoiceApplication::FluidVoiceApplication(int &argc, char **argv)
    : QApplication(argc, argv) {
    setApplicationName("FluidVoice");
    setApplicationDisplayName("FluidVoice");
    setOrganizationName("FluidVoice Linux");
    setDesktopFileName("io.github.davidkodar.FluidVoiceLinux");
    QIcon applicationIcon = QIcon::fromTheme("io.github.davidkodar.FluidVoiceLinux");
    if (applicationIcon.isNull()) {
        applicationIcon = QIcon(":/qt/qml/io/github/davidkodar/FluidVoiceLinux/assets/fluidvoice-app.png");
    }
    setWindowIcon(applicationIcon);
    setQuitOnLastWindowClosed(false);
    QString runtimeDirectory = QStandardPaths::writableLocation(QStandardPaths::RuntimeLocation);
    if (runtimeDirectory.isEmpty()) {
        runtimeDirectory = QStandardPaths::writableLocation(QStandardPaths::GenericCacheLocation)
                           + "/fluidvoice/runtime";
        QDir().mkpath(runtimeDirectory);
        QFile::setPermissions(runtimeDirectory,
                              QFileDevice::ReadOwner | QFileDevice::WriteOwner |
                                  QFileDevice::ExeOwner);
    }
    instanceLock = std::make_unique<QLockFile>(runtimeDirectory + "/fluidvoice-linux.lock");
    instanceLock->setStaleLockTime(1000);
    primaryInstance = instanceLock->tryLock(0);
    if (!primaryInstance && instanceLock->removeStaleLockFile()) {
        primaryInstance = instanceLock->tryLock(0);
    }
    const QString activationName = runtimeDirectory + "/fluidvoice-linux-activation";
    if (primaryInstance) {
        QLocalServer::removeServer(activationName);
        activationServer = std::make_unique<QLocalServer>();
        activationServer->setSocketOptions(QLocalServer::UserAccessOption);
        connect(activationServer.get(), &QLocalServer::newConnection, this, [this]() {
            while (QLocalSocket *socket = activationServer->nextPendingConnection()) {
                connect(socket, &QLocalSocket::disconnected, socket, &QObject::deleteLater);
                socket->disconnectFromServer();
            }
            showSettingsWindow();
        });
        if (!activationServer->listen(activationName)) {
            qWarning() << "FluidVoice activation socket unavailable:"
                       << activationServer->errorString();
        }

        trayMenu = std::make_unique<QMenu>();
        QAction *openAction = trayMenu->addAction("Open FluidVoice");
        connect(openAction, &QAction::triggered, this, &FluidVoiceApplication::showSettingsWindow);
        trayMenu->addSeparator();
        QAction *quitAction = trayMenu->addAction("Quit");
        connect(quitAction, &QAction::triggered, this, &QApplication::quit);

        trayIcon = std::make_unique<QSystemTrayIcon>(windowIcon());
        trayIcon->setToolTip("FluidVoice");
        trayIcon->setContextMenu(trayMenu.get());
        connect(trayIcon.get(), &QSystemTrayIcon::activated, this,
                [this](QSystemTrayIcon::ActivationReason reason) {
                    if (reason == QSystemTrayIcon::Trigger || reason == QSystemTrayIcon::DoubleClick) {
                        showSettingsWindow();
                    }
                });
        trayIcon->show();
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

void FluidVoiceApplication::showSettingsWindow() {
    for (QWindow *window : topLevelWindows()) {
        if (window->title() != "FluidVoice") {
            continue;
        }
        window->show();
        window->raise();
        window->requestActivate();
    }
}

void FluidVoiceApplication::refreshApplicationIcon() {
    const QIcon icon(":/qt/qml/io/github/davidkodar/FluidVoiceLinux/assets/fluidvoice-app.png");
    if (icon.isNull()) {
        return;
    }
    setWindowIcon(icon);
    if (trayIcon) {
        trayIcon->setIcon(icon);
    }
    for (QWindow *window : topLevelWindows()) {
        window->setIcon(icon);
    }
}

bool FluidVoiceApplication::isPrimaryInstance() const { return primaryInstance; }

std::unique_ptr<FluidVoiceApplication> newApplication() {
    static std::vector<QByteArray> argumentStorage = [] {
        QFile commandLineFile("/proc/self/cmdline");
        if (commandLineFile.open(QIODevice::ReadOnly)) {
            QList<QByteArray> arguments = commandLineFile.readAll().split('\0');
            while (!arguments.isEmpty() && arguments.last().isEmpty()) {
                arguments.removeLast();
            }
            if (!arguments.isEmpty()) {
                return std::vector<QByteArray>(arguments.cbegin(), arguments.cend());
            }
        }
        return std::vector<QByteArray>{QByteArray("fluidvoice-ui")};
    }();
    static std::vector<char *> arguments = [] {
        std::vector<char *> result;
        result.reserve(argumentStorage.size() + 1);
        for (QByteArray &argument : argumentStorage) {
            result.push_back(argument.data());
        }
        result.push_back(nullptr);
        return result;
    }();
    static int argc = static_cast<int>(argumentStorage.size());

    return std::make_unique<FluidVoiceApplication>(argc, arguments.data());
}

int execApplication(FluidVoiceApplication &application) {
    return application.exec();
}

bool isPrimaryInstance(const FluidVoiceApplication &application) {
    return application.isPrimaryInstance();
}

void refreshApplicationIcon(FluidVoiceApplication &application) {
    application.refreshApplicationIcon();
}

} // namespace fluidvoice
