#pragma once

#include <QApplication>
#include <QLockFile>
#include <QLocalServer>
#include <QMenu>
#include <QSystemTrayIcon>
#include <memory>

namespace spevox {

class SpevoxApplication final : public QApplication {
public:
    SpevoxApplication(int &argc, char **argv);
    bool isPrimaryInstance() const;
    void refreshApplicationIcon();

private:
    void showSettingsWindow();

    std::unique_ptr<QLockFile> instanceLock;
    std::unique_ptr<QLocalServer> activationServer;
    std::unique_ptr<QMenu> trayMenu;
    std::unique_ptr<QSystemTrayIcon> trayIcon;
    bool primaryInstance = false;
};

std::unique_ptr<SpevoxApplication> newApplication();
int execApplication(SpevoxApplication &application);
bool isPrimaryInstance(const SpevoxApplication &application);
void refreshApplicationIcon(SpevoxApplication &application);

} // namespace spevox
