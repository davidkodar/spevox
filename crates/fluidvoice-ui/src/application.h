#pragma once

#include <QApplication>
#include <QLockFile>
#include <QLocalServer>
#include <QMenu>
#include <QSystemTrayIcon>
#include <memory>

namespace fluidvoice {

class FluidVoiceApplication final : public QApplication {
public:
    FluidVoiceApplication(int &argc, char **argv);
    bool isPrimaryInstance() const;

private:
    void showSettingsWindow();

    std::unique_ptr<QLockFile> instanceLock;
    std::unique_ptr<QLocalServer> activationServer;
    std::unique_ptr<QMenu> trayMenu;
    std::unique_ptr<QSystemTrayIcon> trayIcon;
    bool primaryInstance = false;
};

std::unique_ptr<FluidVoiceApplication> newApplication();
int execApplication(FluidVoiceApplication &application);
bool isPrimaryInstance(const FluidVoiceApplication &application);

} // namespace fluidvoice
