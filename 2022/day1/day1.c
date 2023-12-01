#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <limits.h>

int get_empty_lines_amount(FILE* fp);
void sort_cals(int *cals, int size);
int* parse_cals(FILE* fp, int size);
int sum_top_three_cals(int* cals);

int main (int argc, char *argv[]) {
  FILE * fp;
  fp = fopen("cals.txt", "r");

  if (fp == NULL) {
    printf("error opening file\n");
    return 1;
  }
  int elves = get_empty_lines_amount(fp) + 1;
  int* elves_calories = parse_cals(fp, elves);
  sort_cals(elves_calories, elves);
  printf("max_cals: %i\n", elves_calories[0]);
  printf("sum of top three cals: %i\n", 
      sum_top_three_cals(elves_calories));

  fclose(fp);
  free(elves_calories);
  return 0;
}

int sum_top_three_cals(int* cals) {
  int cals_sum = 0;
  for(int i = 0; i < 3; i++) {
    cals_sum += cals[i];
  }
  return cals_sum;
}


int* parse_cals(FILE* fp, int size) {
  char * line = NULL;
  size_t len = 0;
  ssize_t read;
  int* elves_calories = malloc(size * sizeof(int));
  int cals_sum = 0;
  int curr_cals = 0;
  int elve_index = 0;
  while ((read = getline(&line, &len, fp)) != -1) {
    if(strcmp(line, "\n") == 0) {
      elves_calories[elve_index] = cals_sum;
      cals_sum = 0;
      elve_index++;
    }
    curr_cals = strtol(line, NULL, 10);
    cals_sum += curr_cals;
  }
  return elves_calories;
}

void sort_cals(int *cals, int size) {
  int tmp_cals = 0;
  for(int n = size; n > 1; n--) {
    for(int i = 0; i < n-1; i++) {
      if(cals[i] < cals[i+1]) {
        tmp_cals = cals[i];
        cals[i] = cals[i+1];
        cals[i+1] = tmp_cals;
      }
    }
  }
}

int get_empty_lines_amount(FILE* fp) {
  char * line = NULL;
  size_t len = 0;
  ssize_t read;
  int count = 0;
  while ((read = getline(&line, &len, fp)) != -1) {
    if(strcmp(line, "\n") == 0)
      count++;
  }
  rewind(fp);
  return count;
}
