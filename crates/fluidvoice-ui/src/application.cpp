#include "fluidvoice-ui/src/application.h"

namespace fluidvoice {

FluidVoiceApplication::FluidVoiceApplication(int &argc, char **argv)
    : QApplication(argc, argv) {}

std::unique_ptr<FluidVoiceApplication> newApplication() {
    static int argc = 1;
    static char applicationName[] = "fluidvoice-ui";
    static char *argv[] = {applicationName, nullptr};

    return std::make_unique<FluidVoiceApplication>(argc, argv);
}

int execApplication(FluidVoiceApplication &application) {
    return application.exec();
}

} // namespace fluidvoice
