#if defined(__APPLE__)
    // Apple platforms always have JSC as a system framework
    #include <JavaScriptCore/JavaScript.h>
#elif defined(JSC_INCLUDE_PATH)
    // Explicit path provided via -DJSC_INCLUDE_PATH="..." clang arg
    #include JSC_INCLUDE_PATH
#else
    // Fallback: downloaded/built headers next to this wrapper
    #include "JavaScriptCore/JavaScript.h"
#endif
