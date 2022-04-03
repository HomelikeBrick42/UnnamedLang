#pragma once

#include "SourceLocation.hpp"

#include <exception>
#include <string>

namespace Langite {

    struct CompileError {
        SourceLocation Location;
        std::string Message;
    };

}
