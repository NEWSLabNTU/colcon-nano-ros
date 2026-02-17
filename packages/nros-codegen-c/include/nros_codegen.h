/**
 * @file nros_codegen.h
 * @brief C interface for nros code generation.
 *
 * This header declares the function provided by libnros_codegen_c.a.
 * The CMake build system uses this to compile a thin wrapper executable
 * that drives C code generation at configure time.
 */

#ifndef NROS_CODEGEN_H
#define NROS_CODEGEN_H

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Generate C bindings from a JSON arguments file.
 *
 * @param args_file  Path to the JSON arguments file (null-terminated).
 * @param verbose    Non-zero for verbose output.
 * @return 0 on success, 1 on error (details printed to stderr).
 */
int nros_codegen_generate_c(const char *args_file, int verbose);

#ifdef __cplusplus
}
#endif

#endif  /* NROS_CODEGEN_H */
