#include <stdlib.h> // exit,atexit.
#include <termios.h> // struct termios,tcgetattr,tcsetattr,ECHO,TCSAFLUSH,etc.
#include <unistd.h> // read,STDIN_FILENO.
#include <stdio.h> // perror, NULL.
#include <errno.h> // errno macro,EAGAIN
#include <stdatomic.h> // atomic datatypes.
#include <pthread.h>
#include <sys/select.h>
#include <assert.h>
#include <signal.h>
#include <time.h>

#include "unix.h"

#define CHAR_BUF_SIZE (uint64_t)1000
#define TIMEOUT_SECONDS (uint64_t)1

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

static void errorHandler(const char* s);
static void signalHandler(int signal);
static bool isOkToRead(const uint64_t index_read, const uint64_t index_write, const bool index_flip);
static bool isOkToWrite(const uint64_t index_read, const uint64_t index_write, const bool index_flip);
static void* writerFn(void*);

void disablePartialRawMode() {
  puts("\x1B[?1002l");
  puts("\x1B[?1006l");
  puts("\x1B[?25h");

  if (tcsetattr(STDIN_FILENO, TCSAFLUSH, &orig_termios) == -1) {
    errorHandler(NULL);
  }
}

void enablePartialRawMode() {
  tcgetattr(STDIN_FILENO, &orig_termios);
  struct termios raw = orig_termios;
  raw.c_iflag &= ~(ICRNL | IXON);
  raw.c_lflag &= ~(ECHO | ICANON | IEXTEN); // ISIG can be added here.
  // raw.c_oflag &= ~(OPOST); // TODO: Enable?
  raw.c_cc[VMIN] = 1; // Only return from read when at least one character is ready (=1), but don't block (=0).
  raw.c_cc[VTIME] = TIMEOUT_SECONDS * 10; // Time to wait for input in deciseconds (i.e. 1/10:th seconds).
  tcsetattr(STDIN_FILENO, TCSAFLUSH, &raw);

  // Also enable mouse tracking via ANSI escape codes (xterm).
  puts("\x1B[?1002h"); // Track button presses + movement while pressed.
  puts("\x1B[?1006h"); // Enable SGR mouse mode to support large terminals (> char/u8 size coordinates).
  puts("\x1B[?25l"); // Hide cursor
}

uint8_t getChar(char * const buf, const bool blocking) {
  if (!atomic_load(&initialized) || atomic_load(&error)) {
    return 1;
  }

  uint8_t ret = 0;
  struct timespec ts;

  pthread_mutex_lock(&mutex);

  if (!blocking && char_buf_available == 0) {
    pthread_mutex_unlock(&mutex);
    return 2;
  }

  clock_gettime(CLOCK_REALTIME, &ts);
  ts.tv_sec += TIMEOUT_SECONDS;

  while (char_buf_available < 1 && !atomic_load(&error)) {
    pthread_cond_timedwait(&cond, &mutex, &ts);
    clock_gettime(CLOCK_REALTIME, &ts);
    ts.tv_sec += TIMEOUT_SECONDS;
  }

  if (atomic_load(&error)) {
    pthread_mutex_unlock(&mutex);
    return 1;
  }

  if (isOkToRead(char_buf_index_read, char_buf_index_write, char_buf_index_flip)) {
    *buf = char_buf[char_buf_index_read];

    uint64_t new_index = (char_buf_index_read + 1) % CHAR_BUF_SIZE;
    if (new_index < char_buf_index_read) {
      char_buf_index_flip = false;
    }
    char_buf_index_read = new_index;

    char_buf_available -= 1;
    ret = 0;
  } else {
    // Should not be possible(?)
    assert(false);
  }

  pthread_mutex_unlock(&mutex);
  return ret;
}

bool running() {
  return atomic_load(&initialized) && !atomic_load(&error);
}

void initialize() {
  if (!atomic_load(&initialized)) {
    signal(SIGINT, int signalsignalHandler);
    signal(SIGTERM, signalHandler);
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
    pthread_cancel(writer);
    pthread_mutex_destroy(&mutex);
    pthread_cond_destroy(&cond);
  }
}

void errorHandler(const char* s) {
  if (s == NULL) {
    s = DEFAULT_ERROR_MSG;
  }

  atomic_store(&error, true);
  // perror(s); // Caused issues with Rusts locked buffers.
}

void signalHandler(int signal) {
  if (atomic_load(&initialized)) {
    disablePartialRawMode();
    errorHandler("Signal received");
  }
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

void* writerFn(void*) {
  char buffer;
  int64_t len = 0;

  while (atomic_load(&initialized) && !atomic_load(&error)) {
    if ((len = read(STDIN_FILENO, &buffer, 1)) > 0) {
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
    } else if (len == -1) {
      errorHandler("read() error");
    }
  }

  pthread_exit(NULL);
}
