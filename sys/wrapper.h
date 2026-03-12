// generated with artificial intelligence
// sounds bad. but whatever the fuck
// so i just add some personal touches in case you hate its format

#if defined(JSC_INCLUDE_PATH)
    //  path provided from -DJSC_INCLUDE_PATH="..."
    #include JSC_INCLUDE_PATH
#else
    // downloaded/built headers next to the wrapper
    #include "JavaScriptCore/JavaScript.h"
#endif

#ifdef __cplusplus
extern "C" {
#endif

void JSLock(JSContextGroupRef group);
void JSUnlock(JSContextGroupRef group);
JSContextGroupRef JSContextGetGroup(JSContextRef ctx);
JSContextGroupRef JSContextGroupRetain(JSContextGroupRef group);
void JSContextGroupRelease(JSContextGroupRef group);

#ifdef __cplusplus
}
#endif
