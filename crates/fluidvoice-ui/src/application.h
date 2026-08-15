#pragma once

#include <QApplication>
#include <memory>

namespace fluidvoice {

class FluidVoiceApplication final : public QApplication {
public:
    FluidVoiceApplication(int &argc, char **argv);
};

std::unique_ptr<FluidVoiceApplication> newApplication();
int execApplication(FluidVoiceApplication &application);

} // namespace fluidvoice
