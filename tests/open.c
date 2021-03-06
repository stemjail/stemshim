#include <fcntl.h>
#define __USE_LARGEFILE64
#include <stdio.h>
#include <sys/stat.h>
#include <dirent.h>

extern int __open(const char *pathname, int flags);

int main(int argc, char *argv[])
{
	printf("\nread:\n");
	open(argv[0], 0);
	printf("\nread:\n");
	open(argv[0], O_RDONLY);
	printf("\nwrite:\n");
	open(argv[0], O_WRONLY);
	printf("\nwrite:\n");
	open(argv[0], O_RDWR);
	umask(0022);
	printf("\nwrite:\n");
	open(argv[0], O_CREAT, 0220);
	printf("\nwrite:\n");
	open(argv[0], O_CREAT);
	printf("\nread:\n");
	__open(argv[0], 0);
	printf("\nread:\n");
	fopen(argv[0], "r");
	printf("\nwrite:\n");
	fopen(argv[0], "w");
	printf("\nwrite:\n");
	fopen(argv[0], "rw");
	printf("\nread:\n");
	fopen64(argv[0], "r");
	struct stat s;
	printf("\nread:\n");
	stat(".", &s);
	printf("\nread:\n");
	opendir(".");

	/* Bypasse -Werror=unused-parameter */
	argc = argc;

	return 0;
}
