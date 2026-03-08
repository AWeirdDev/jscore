// generated with artificial intelligence
// sounds bad. but whatever the fuck
// so i just add some personal touches in case you hate its format

#if defined(__APPLE__)
    // apple platforms always have JSC as a system framework
    #include <JavaScriptCore/JavaScript.h>
#elif defined(JSC_INCLUDE_PATH)
    //  path provided from -DJSC_INCLUDE_PATH="..."
    #include JSC_INCLUDE_PATH
#else
    // downloaded/built headers next to the wrapper
    #include "JavaScriptCore/JavaScript.h"
#endif
