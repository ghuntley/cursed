#ifndef CURSED_RT_H
#define CURSED_RT_H

#include <stdint.h>
#include <stddef.h>

// Common ABI types
typedef struct {
    char* ptr;
    int64_t len;
} cursed_str_t;

typedef struct {
    char* data;
    int64_t len;
    int64_t cap;
} cursed_vec_t;

// Math functions
double mathz_add(double a, double b);
double mathz_sub(double a, double b);
double mathz_mul(double a, double b);
double mathz_div(double a, double b);
double mathz_abs_normie(double x);
double mathz_max_normie(double a, double b);
double mathz_min_normie(double a, double b);

// Debug/printing functions
int32_t cursed_dbg_spill_str(cursed_str_t str);
int32_t cursed_dbg_spill_f64(double value);
int32_t cursed_dbg_spill_i64(int64_t value);

// String conversion functions
cursed_str_t cursed_to_string_f64(double value);
cursed_str_t cursed_to_string_i64(int64_t value);

// Collections functions
cursed_vec_t collections_vec_new(void);
cursed_vec_t collections_vec_push(cursed_vec_t vec, int64_t item);
int64_t collections_vec_len(cursed_vec_t vec);
int64_t collections_vec_get(cursed_vec_t vec, int64_t index);

// JSON functions  
int32_t json_validate(cursed_str_t json_str);
cursed_str_t json_stringify(cursed_str_t data);

// Memory functions
int64_t memory_malloc(int64_t size);
void memory_free(int64_t addr);
void memory_memset(int64_t addr, int32_t value, int64_t size);

#endif // CURSED_RT_H
