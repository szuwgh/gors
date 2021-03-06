#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

void *new_builder(void);

void add_key(void *arg, const uint8_t *key, uint32_t len);

const uint8_t *get(void *arg);

uint32_t len(void *arg);
