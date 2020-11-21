#include <stdio.h>

#define FILE_SIZE 2030

int main(void)
{
    FILE *fp = fopen("../2017-01.txt", "r");

    // Part 1
    int part1 = 0;
    char first = fgetc(fp);
    char current = first, next = -1;
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

    // Part 1
    int part2 = 0;
    fpos_t currentPos = 0;

    rewind(fp);
    for (int i = 0; i < FILE_SIZE / 2; i++) {
        current = fgetc(fp);
        fseek(fp, FILE_SIZE / 2 - 1, SEEK_CUR);
        next = fgetc(fp);
        fseek(fp, - FILE_SIZE / 2, SEEK_CUR);

        if (current == next) {
            part2 += (current - '0') * 2;
        }
    }

    // Output
    printf("Part1: %d\n", part1);
    printf("Part2: %d\n", part2);

    fclose(fp);

    return 0;
}