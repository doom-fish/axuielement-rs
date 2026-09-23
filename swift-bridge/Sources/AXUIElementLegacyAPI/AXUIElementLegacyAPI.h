#pragma once

#include <ApplicationServices/ApplicationServices.h>

extern Boolean AXLegacyAPIEnabled(void) __asm("_AXAPIEnabled");

extern AXError AXLegacyMakeProcessTrusted(CFStringRef executablePath) __asm("_AXMakeProcessTrusted");

extern AXError AXLegacyUIElementPostKeyboardEvent(
    AXUIElementRef application,
    CGCharCode keyChar,
    CGKeyCode virtualKey,
    Boolean keyDown) __asm("_AXUIElementPostKeyboardEvent");
