#include <QAction>
#include <QApplication>
#include <QIcon>
#include <QLabel>
#include <QMenu>
#include <QPushButton>
#include <QStyle>
#include <QSystemTrayIcon>
#include <QVBoxLayout>
#include <QWidget>

#include <memory>

int main(int argc, char* argv[])
{
    QApplication application(argc, argv);
    QApplication::setApplicationName(QStringLiteral("FluidVoice Linux"));
    QApplication::setApplicationVersion(QStringLiteral("0.1.0"));
    QApplication::setOrganizationName(QStringLiteral("FluidVoice Linux"));
    application.setQuitOnLastWindowClosed(false);

    QWidget settingsWindow;
    settingsWindow.setWindowTitle(QStringLiteral("FluidVoice Linux"));
    settingsWindow.resize(480, 280);

    auto* layout = new QVBoxLayout(&settingsWindow);
    auto* title = new QLabel(QStringLiteral("FluidVoice Linux"));
    auto titleFont = title->font();
    titleFont.setPointSize(titleFont.pointSize() + 6);
    titleFont.setBold(true);
    title->setFont(titleFont);
    layout->addWidget(title);

    auto* status = new QLabel(QStringLiteral(
        "Project foundation ready. The next milestone connects the KDE global "
        "shortcut, PipeWire capture, and local transcription."));
    status->setWordWrap(true);
    layout->addWidget(status);
    layout->addStretch();

    auto* closeButton = new QPushButton(QStringLiteral("Close"));
    QObject::connect(closeButton, &QPushButton::clicked, &settingsWindow, &QWidget::hide);
    layout->addWidget(closeButton);

    QSystemTrayIcon trayIcon;
    trayIcon.setToolTip(QStringLiteral("FluidVoice Linux — idle"));
    trayIcon.setIcon(application.style()->standardIcon(QStyle::SP_MediaVolume));

    QMenu trayMenu;
    auto* openAction = trayMenu.addAction(QStringLiteral("Open FluidVoice Linux"));
    QObject::connect(openAction, &QAction::triggered, &settingsWindow, [&settingsWindow] {
        settingsWindow.show();
        settingsWindow.raise();
        settingsWindow.activateWindow();
    });
    trayMenu.addSeparator();
    auto* quitAction = trayMenu.addAction(QStringLiteral("Quit"));
    QObject::connect(quitAction, &QAction::triggered, &application, &QApplication::quit);

    trayIcon.setContextMenu(&trayMenu);
    QObject::connect(&trayIcon, &QSystemTrayIcon::activated, &settingsWindow,
        [&settingsWindow](QSystemTrayIcon::ActivationReason reason) {
            if (reason == QSystemTrayIcon::Trigger) {
                settingsWindow.setVisible(!settingsWindow.isVisible());
                if (settingsWindow.isVisible()) {
                    settingsWindow.raise();
                    settingsWindow.activateWindow();
                }
            }
        });
    trayIcon.show();

    if (!QSystemTrayIcon::isSystemTrayAvailable()) {
        settingsWindow.show();
    }

    return application.exec();
}

