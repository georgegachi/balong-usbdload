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

#include "patcher.h"


//#######################################################################################################
void main(int argc, char* argv[]) {
  
FILE* in;
FILE* out;
uint8_t* buf;
uint32_t fsize;
int opt;
uint8_t outfilename[100];
int oflag=0,bflag=0;
uint32_t res;


// Разбор командной строки

while ((opt = getopt(argc, argv, "o:bh")) != -1) {
  switch (opt) {
   case 'h': 
printf("\n Program for automatic patching of Balong V7 platform loaders\n\n\
%s [options] <usbloader file>\n\n\
  Valid keys are:\n\n\
-o file - output file name. By default, only the possibility of a patch is checked\n\
-b - add a patch that disables bad block checking\n\
\n",argv[0]);

    return;

   case 'o':
    strcpy(outfilename,optarg);
    oflag=1;
    break;

   case 'b':
     bflag=1;
     break;
     
   case '?':
   case ':':  
     return;
    
  }
}  

printf("\n Программа автоматической модификации загрузчиков Balong V7, (c) forth32");

 if (optind>=argc) {
    printf("\n - Не указано имя файла для загрузки\n - Для подсказки укажите ключ -h\n");
    return;
}  
    
in=fopen(argv[optind],"rb");
if (in == 0) {
  printf("\n Ошибка открытия файла %s",argv[optind]);
  return;
}

// определяем размер файла
fseek(in,0,SEEK_END);
fsize=ftell(in);
rewind(in);

// выделяем буфер и читаем туда весь файл
buf=malloc(fsize);
fread(buf,1,fsize,in);
fclose(in);

//==================================================================================

res=pv7r1(buf, fsize);
if (res != 0)  {
  printf("\n* Найдена сигнатура типа V7R1 по смещению %08x",res);
  goto endpatch;
}  

res=pv7r2(buf, fsize);
if (res != 0)  {
  printf("\n* Найдена сигнатура типа V7R2 по смещению %08x",res);
  goto endpatch;
}  

res=pv7r11(buf, fsize);
if (res != 0)  {
  printf("\n* Найдена сигнатура типа V7R11 по смещению %08x",res);
  goto endpatch;
}   

res=pv7r22(buf, fsize);
if (res != 0)  {
  printf("\n* Найдена сигнатура типа V7R22 по смещению %08x",res);
  goto endpatch;
}  

res=pv7r22_2(buf, fsize);
if (res != 0)  {
  printf("\n* Найдена сигнатура типа V7R22_2 по смещению %08x",res);
  goto endpatch;
}

res=pv7r22_3(buf, fsize);
if (res != 0)  {
  printf("\n* Найдена сигнатура типа V7R22_3 по смещению %08x",res);
  goto endpatch;
}

printf("\n! Сигнатура eraseall-патча не найдена");

//==================================================================================
endpatch:

if (bflag) {
   res=perasebad(buf, fsize);
   if (res != 0) printf("\n* Найдена сигнатура isbad по смещению %08x",res);  
   else  printf("\n! Сигнатура isbad не найдена");  
}

if (oflag) {
  out=fopen(outfilename,"wb");
  if (out != 0) {
    fwrite(buf,1,fsize,out);
    fclose(out);
  }
  else printf("\n Ошибка открытия выходного файла %s",outfilename);
}
free(buf);
printf("\n");
}

   