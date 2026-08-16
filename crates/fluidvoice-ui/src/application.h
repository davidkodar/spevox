#pragma once

#include <QApplication>
#include <QLockFile>
#include <memory>

namespace fluidvoice {

class FluidVoiceApplication final : public QApplication {
public:
    FluidVoiceApplication(int &argc, char **argv);
    bool isPrimaryInstance() const;

private:
    std::unique_ptr<QLockFile> instanceLock;
    bool primaryInstance = false;
};

std::unique_ptr<FluidVoiceApplication> newApplication();
int execApplication(FluidVoiceApplication &application);
bool isPrimaryInstance(const FluidVoiceApplication &application);

} // namespace fluidvoice
