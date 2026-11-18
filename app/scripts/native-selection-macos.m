#import <ApplicationServices/ApplicationServices.h>
#import <stdlib.h>
#import <unistd.h>

// Select a fixture passage with actual macOS key events after the harness places the caret. Requires Accessibility permission and a focused test window.
int main(int argc, char **argv) {
    if (argc != 3 || !AXIsProcessTrusted()) return 2;
    pid_t pid = atoi(argv[1]);
    int length = atoi(argv[2]);
    if (pid <= 0 || length < 1 || length > 1000) return 2;
    for (int i = 0; i < length; i++) {
        CGEventRef down = CGEventCreateKeyboardEvent(NULL, 124, true);
        CGEventRef up = CGEventCreateKeyboardEvent(NULL, 124, false);
        CGEventSetFlags(down, kCGEventFlagMaskShift);
        CGEventSetFlags(up, kCGEventFlagMaskShift);
        CGEventPostToPid(pid, down);
        usleep(10000);
        CGEventPostToPid(pid, up);
        CFRelease(down);
        CFRelease(up);
        usleep(10000);
    }
    return 0;
}
