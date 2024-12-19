// C wrapper for the FASTER C++ submodule.
//
// bindgen does not support inlined functions, template functions, or methods of template classes, all of which are used
// by FASTER. This wrapper exposes a limited C interface that does not use these features. In older versions of FASTER
// the library included a C wrapper (formerly at `src/core/faster-c.h`). This file exposes a similar interface.

#ifndef FASTER_C_H_
#define FASTER_C_H_

#ifdef __cplusplus
extern "C" {
#endif

#include <stddef.h>
#include <stdint.h>
#include <stdbool.h>

  typedef struct faster_t faster_t;

  typedef struct faster_result faster_result;
  typedef void (*faster_callback)(faster_result);

  enum faster_status {
      Ok,
      Pending,
      NotFound,
      OutOfMemory,
      IOError,
      Corrupted,
      Aborted
  };
  typedef enum faster_status faster_status;

  typedef void (*read_callback)(void*, const uint8_t*, uint64_t, faster_status);
  typedef uint64_t (*rmw_callback)(const uint8_t*, uint64_t, uint8_t*, uint64_t, uint8_t*);

  typedef struct faster_checkpoint_result faster_checkpoint_result;
  struct faster_checkpoint_result {
    bool checked;
    char* token;
  };

  typedef struct faster_recover_result faster_recover_result;
  struct faster_recover_result {
    uint8_t status;
    uint32_t version;
    int session_ids_count;
    char* session_ids;
  };

  // Thread-related operations
  const char* faster_start_session(faster_t* faster_t);
  uint64_t faster_continue_session(faster_t* faster_t, const char* token);
  void faster_stop_session(faster_t* faster_t);
  void faster_refresh_session(faster_t* faster_t);
  void faster_complete_pending(faster_t* faster_t, bool b);

  // Checkpoint/Recover
  faster_checkpoint_result* faster_checkpoint(faster_t* faster_t);
  faster_checkpoint_result* faster_checkpoint_index(faster_t* faster_t);
  faster_checkpoint_result* faster_checkpoint_hybrid_log(faster_t* faster_t);
  faster_recover_result* faster_recover(faster_t* faster_t, const char* index_token, const char* hybrid_log_token);

  // Operations
  faster_t* faster_open(const uint64_t table_size, const uint64_t log_size, bool pre_allocate_log);
  faster_t* faster_open_with_disk(const uint64_t table_size, const uint64_t log_size, const char* storage,
                                  double log_mutable_fraction, bool pre_allocate_log);
  uint8_t faster_upsert(faster_t* faster_t, const uint8_t* key, const uint64_t key_length,
                        uint8_t* value, uint64_t value_length, const uint64_t monotonic_serial_number);
  uint8_t faster_rmw(faster_t* faster_t, const uint8_t* key, const uint64_t key_length, uint8_t* modification,
                     const uint64_t length, const uint64_t monotonic_serial_number, rmw_callback cb);
  uint8_t faster_read(faster_t* faster_t, const uint8_t* key, const uint64_t key_length,
                       const uint64_t monotonic_serial_number, read_callback cb, void* target);
  uint8_t faster_delete(faster_t* faster_t, const uint8_t* key, const uint64_t key_length,
                        const uint64_t monotonic_serial_number);
  void faster_destroy(faster_t* faster_t);
  bool faster_grow_index(faster_t* faster_t);

  // Statistics
  uint64_t faster_size(faster_t* faster_t);
  void faster_dump_distribution(faster_t* faster_t);

#ifdef __cplusplus
}  // extern "C"
#endif

#endif  /* FASTER_C_H_ */




