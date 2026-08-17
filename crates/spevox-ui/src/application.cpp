#include "spevox-ui/src/application.h"

#include <QDir>
#include <QDebug>
#include <QFile>
#include <QIcon>
#include <QLocalSocket>
#include <QStandardPaths>
#include <QWindow>
#include <vector>

namespace spevox {

SpevoxApplication::SpevoxApplication(int &argc, char **argv)
    : QApplication(argc, argv) {
    setApplicationName("Spevox");
    setApplicationDisplayName("Spevox");
    setOrganizationName("Spevox");
    setDesktopFileName("io.github.davidkodar.Spevox");
    QIcon applicationIcon = QIcon::fromTheme("spevox-app");
    if (applicationIcon.isNull()) {
        applicationIcon = QIcon(":/qt/qml/io/github/davidkodar/Spevox/assets/spevox-app.png");
    }
    setWindowIcon(applicationIcon);
    setQuitOnLastWindowClosed(false);
    QString runtimeDirectory = QStandardPaths::writableLocation(QStandardPaths::RuntimeLocation);
    if (runtimeDirectory.isEmpty()) {
        runtimeDirectory = QStandardPaths::writableLocation(QStandardPaths::GenericCacheLocation)
                           + "/spevox/runtime";
        QDir().mkpath(runtimeDirectory);
        QFile::setPermissions(runtimeDirectory,
                              QFileDevice::ReadOwner | QFileDevice::WriteOwner |
                                  QFileDevice::ExeOwner);
    }
    instanceLock = std::make_unique<QLockFile>(runtimeDirectory + "/spevox.lock");
    instanceLock->setStaleLockTime(1000);
    primaryInstance = instanceLock->tryLock(0);
    if (!primaryInstance && instanceLock->removeStaleLockFile()) {
        primaryInstance = instanceLock->tryLock(0);
    }
    const QString activationName = runtimeDirectory + "/spevox-activation";
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
            qWarning() << "Spevox activation socket unavailable:"
                       << activationServer->errorString();
        }

        trayMenu = std::make_unique<QMenu>();
        QAction *openAction = trayMenu->addAction("Open Spevox");
        connect(openAction, &QAction::triggered, this, &SpevoxApplication::showSettingsWindow);
        trayMenu->addSeparator();
        QAction *quitAction = trayMenu->addAction("Quit");
        connect(quitAction, &QAction::triggered, this, &QApplication::quit);

        trayIcon = std::make_unique<QSystemTrayIcon>(windowIcon());
        trayIcon->setToolTip("Spevox");
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

void SpevoxApplication::showSettingsWindow() {
    for (QWindow *window : topLevelWindows()) {
        if (window->title() != "Spevox") {
            continue;
        }
        window->show();
        window->raise();
        window->requestActivate();
    }
}

void SpevoxApplication::refreshApplicationIcon() {
    const QIcon icon(":/qt/qml/io/github/davidkodar/Spevox/assets/spevox-app.png");
    if (icon.isNull()) {
        return;
    }
    setWindowIcon(icon);
    if (trayIcon) {
        trayIcon->setIcon(QIcon(":/qt/qml/io/github/davidkodar/Spevox/assets/spevox-tray.png"));
    }
    for (QWindow *window : topLevelWindows()) {
        window->setIcon(icon);
    }
}

bool SpevoxApplication::isPrimaryInstance() const { return primaryInstance; }

std::unique_ptr<SpevoxApplication> newApplication() {
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
        return std::vector<QByteArray>{QByteArray("spevox")};
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

    return std::make_unique<SpevoxApplication>(argc, arguments.data());
}

int execApplication(SpevoxApplication &application) {
    return application.exec();
}

bool isPrimaryInstance(const SpevoxApplication &application) {
    return application.isPrimaryInstance();
}

void refreshApplicationIcon(SpevoxApplication &application) {
    application.refreshApplicationIcon();
}

} // namespace spevox
