#include "fluidvoice-ui/src/application.h"

#include <QDir>
#include <QIcon>
#include <QStandardPaths>

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
