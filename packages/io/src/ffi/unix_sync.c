#include <stdlib.h> // exit,atexit.
#include <termios.h> // struct termios,tcgetattr,tcsetattr,ECHO,TCSAFLUSH,etc.
#include <unistd.h> // read,STDIN_FILENO.
#include <stdio.h> // perror, NULL.
#include <errno.h> // errno macro,EAGAIN
#include <stdatomic.h> // atomic datatypes.
#include <pthread.h>
#include <sys/select.h>
#include <assert.h>

#include "unix.h"

#define CHAR_BUF_SIZE (uint64_t)1000

// Terminal stuff
static const char* DEFAULT_ERROR_MSG = "Terminal I/O Failed";
static struct termios orig_termios;

// Threading
static char char_buf[CHAR_BUF_SIZE];
static uint64_t char_buf_index_read = 0;
static uint64_t char_buf_index_write = 0;
static uint64_t char_buf_available = 0;
static bool char_buf_index_flip = false;
static atomic_bool initialized = false;
static atomic_bool error = false;
static pthread_t writer;
static pthread_mutex_t mutex;
static pthread_cond_t cond;

static void errorHandler();
static void exitHandler();
static bool isOkToRead(const uint64_t index_read, const uint64_t index_write, const bool index_flip);
static bool isOkToWrite(const uint64_t index_read, const uint64_t index_write, const bool index_flip);
static void* writerFn(void* _);

void disablePartialRawMode() {
  puts("\x1B[?1002l");
  puts("\x1B[?1006l");

  if (tcsetattr(STDIN_FILENO, TCSAFLUSH, &orig_termios) == -1) {
    errorHandler(NULL);
  }
}

void enablePartialRawMode() {
  tcgetattr(STDIN_FILENO, &orig_termios);
  struct termios raw = orig_termios;
  raw.c_iflag &= ~(ICRNL | IXON);
  raw.c_lflag &= ~(ECHO | ICANON | IEXTEN | ISIG );
  // raw.c_oflag &= ~(OPOST); // TODO: Enable?
  raw.c_cc[VMIN] = 0; // Only return from read when at least one character is ready (=1), but don't block (=0).
  raw.c_cc[VTIME] = 0; // Time to wait for input in deciseconds (i.e. 1/10:th seconds). In our case do not wait. Handle waiting on input elsewhere.
  tcsetattr(STDIN_FILENO, TCSAFLUSH, &raw);

  // Also enable mouse tracking via ANSI escape codes (xterm).
  puts("\x1B[?1002h"); // Track button presses + movement while pressed.
  puts("\x1B[?1006h"); // Enable SGR mouse mode to support large terminals (> char/u8 size coordinates).
}

void setExitHandler() {
  atexit(exitHandler);
}

bool getChar(char * const buf) {
  if (!atomic_load(&initialized) || atomic_load(&error)) {
    return false;
  }

  bool ret = false;

  pthread_mutex_lock(&mutex);

  while (char_buf_available < 1) {
    pthread_cond_wait(&cond, &mutex);
  }

  if (isOkToRead(char_buf_index_read, char_buf_index_write, char_buf_index_flip)) {
    *buf = char_buf[char_buf_index_read];

    uint64_t new_index = (char_buf_index_read + 1) % CHAR_BUF_SIZE;
    if (new_index < char_buf_index_read) {
      char_buf_index_flip = false;
    }
    char_buf_index_read = new_index;

    char_buf_available -= 1;
    ret = true;
  } else {
    // Should not be possible(?)
    assert(false);
  }

  pthread_mutex_unlock(&mutex);

  return ret;
}

void initialize() {
  if (!atomic_load(&initialized)) {
    pthread_mutex_init(&mutex, NULL);
    pthread_cond_init(&cond, NULL);
    enablePartialRawMode();
    char_buf_index_read = 0;
    char_buf_index_write = 0;
    char_buf_available = 0;
    char_buf_index_flip = false;
    pthread_create(&writer, NULL, writerFn, NULL);
    atomic_store(&error, false);
    atomic_store(&initialized, true);
  }
}

void terminate() {
  if (atomic_load(&initialized)) {
    atomic_store(&initialized, false);
    disablePartialRawMode();
    pthread_join(writer, NULL);
    pthread_mutex_destroy(&mutex);
    pthread_cond_destroy(&cond);
  }
}

void errorHandler(const char* s) {
  if (s == NULL) {
    s = DEFAULT_ERROR_MSG;
  }

  atomic_store(&error, true);
  perror(s);
}

void exitHandler() {
  disablePartialRawMode();
  terminate();
}

bool isOkToRead(const uint64_t index_read, const uint64_t index_write, const bool index_flip) {
  bool non_flip = index_read < index_write && !index_flip;
  bool flip = index_read > index_write && index_flip;
  return non_flip || flip;
}

bool isOkToWrite(const uint64_t index_read, const uint64_t index_write, const bool index_flip) {
  bool flip_guard = !(index_write == CHAR_BUF_SIZE - 1 && index_read == 0);
  bool non_flip = index_write >= index_read && !index_flip;
  bool flip = index_write < (index_read - 1) && index_flip;
  return flip_guard && (non_flip || flip);
}

void* writerFn(void* _) {
  fd_set readfds;
  struct timeval timeout;
  char buffer;

  timeout.tv_sec = 0;
  timeout.tv_usec = 1000; // 1 millisecond (1000Hz polling rate).

  while (atomic_load(&initialized)) {
    FD_ZERO(&readfds);
    FD_SET(STDIN_FILENO, &readfds);

    // Wait for input using select()
    int retval = select(STDIN_FILENO + 1, &readfds, NULL, NULL, &timeout);

    if (retval == -1) {
      errorHandler("select() error");
    } else if (retval != 0) {
      // Input is available, read it
      if (read(STDIN_FILENO, &buffer, 1) > 0) {
        pthread_mutex_lock(&mutex);

        if (isOkToWrite(char_buf_index_read, char_buf_index_write, char_buf_index_flip)) {
          char_buf[char_buf_index_write] = buffer;

          uint64_t new_index = (char_buf_index_write + 1) % CHAR_BUF_SIZE;
          if (new_index < char_buf_index_write) {
            char_buf_index_flip = true;
          }
          char_buf_index_write = new_index;
          char_buf_available += 1;
        }

        pthread_cond_signal(&cond);
        pthread_mutex_unlock(&mutex);
      } else {
        errorHandler("read() error");
      }
    }
  }

  pthread_exit(NULL);
}