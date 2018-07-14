#include <stdio.h>

int main(void)
{
    FILE *fp = fopen("../2017-01.txt", "r");

    char first = fgetc(fp);
    char current = first, next = -1;
    int part1 = 0;

    while ((next = fgetc(fp)) != EOF)
    {
        if (current == next) 
        {
            part1 += current - '0';
        }
        current = next;
    }
    if (current == first)
    {
        part1 += current - '0';
    }

    fclose(fp);

    printf("Part 1: %d", part1);

    return 0;
}