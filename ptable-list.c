// Program to replace the partition table in the usbloader bootloader
// 
// 
#include <stdio.h>
#include <stdint.h>

#ifndef WIN32
//%%%%
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#else
//%%%%
#include <windows.h>
#include "getopt.h"
#include "printf.h"
#endif

#include "parts.h"

  
//############################################################################################################3

void main(int argc, char* argv[]) {
  
  

struct ptable_t ptable;
FILE* in;

if (argc != 2) {    
    printf("\n - Filename with partition table not specified\n");
    return;
}  

in=fopen(argv[optind],"r+b");
if (in == 0) {  
  printf("\n Error opening file %s\n",argv[optind]);
  return;
}

 
// читаем текущую таблицу
fread(&ptable,sizeof(ptable),1,in);

if (strncmp(ptable.head, "pTableHead", 16) != 0) {  
  printf("\n File is not a partition table\n");
  return ;
}
  
show_map(ptable);
}
