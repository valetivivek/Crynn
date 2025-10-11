#ifndef GECKO_WRAPPER_H
#define GECKO_WRAPPER_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

// GeckoView initialization
int gecko_init(void);
void gecko_shutdown(void);

// Window management
void* gecko_create_window(int width, int height, const char* title);
void gecko_destroy_window(void* window);
void gecko_resize_window(void* window, int width, int height);

// Navigation
int gecko_navigate_to(void* window, const char* url);
int gecko_go_back(void* window);
int gecko_go_forward(void* window);
int gecko_reload(void* window);
int gecko_stop(void* window);

// Tab management
void* gecko_create_tab(void* window);
void gecko_close_tab(void* window, void* tab);
int gecko_switch_to_tab(void* window, void* tab);

// Event handling
typedef void (*gecko_event_callback)(const char* event_type, const char* data, void* user_data);
void gecko_set_event_callback(void* window, gecko_event_callback callback, void* user_data);

// Memory management
size_t gecko_get_memory_usage(void* window);
void gecko_garbage_collect(void* window);

#ifdef __cplusplus
}
#endif

#endif // GECKO_WRAPPER_H
