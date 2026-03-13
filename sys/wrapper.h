// no more ai fuck yeah

#if defined(JSC_INCLUDE_PATH) && defined(RI_INCLUDE_PATH)
    //  path provided from -DJSC_INCLUDE_PATH="..."
    #include JSC_INCLUDE_PATH
    #include RI_INCLUDE_PATH
#else
    #error couldn't find JavaScriptCore.h and JSRemoteInspector.h, see logs
#endif
