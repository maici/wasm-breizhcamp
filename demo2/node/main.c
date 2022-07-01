#include <stdio.h>
#include <stdlib.h>

#define BLOCK_COUNT 10

#ifdef EMSCRIPTEN
#include <emscripten.h>
#endif

#include <markdownCLib.h>

int main(int argc, char *argv[])
{

#ifdef EMSCRIPTEN
#ifndef NODERAWFS
  // mount the current folder as a NODEFS instance
  // inside of emscripten
  EM_ASM(
      FS.mkdir('/working');
      FS.mount(NODEFS, {root : '.'}, '/working'););
#endif
#endif

#ifdef EMSCRIPTEN
  FILE *file = fopen("/working/hello_world_file.md", "rb");
#else
  FILE *file = fopen("./hello_world_file.md", "rb");
#endif

  // Get size of file
  fseek(file, 0L, SEEK_END);
  int sz = ftell(file);

  rewind(file);

  if (!file)
  {
    printf("cannot open file\n");
    return 1;
  }

  char buffer[sz + 1];
  if (1 != fread(buffer, sz, 1, file))
  {
    fprintf(stderr, "Cannot read blocks in file\n");
  }
  buffer[sz] = '\0';

  fclose(file);

  const char *test = render_markdown(buffer);

  printf("%s ", (test));

  return 0;
}
