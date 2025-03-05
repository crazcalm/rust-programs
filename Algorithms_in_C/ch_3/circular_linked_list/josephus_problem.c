#include <stdio.h>
#include <stdlib.h>

struct node {
  int key;
  struct node *next;
}

main() {
  int i, N, M;
  struct node *t, *x;

  // Capture user input
  scanf("%d %d", &N, &M);

  // Creating the linked list of number
  t = (struct node *)malloc(sizeof *t);
  t->key = 1;
  x = t;
  for (i = 2; i <= N; i++) {
    t->next = (struct node *)malloc(sizeof *t);
    t = t->next;
    t->key = i;
  }

  // Pointing the last node to the first node.
  // Makes the linked list circular
  t->next = x;

  // Josephus problem
  while (t != t->next) {
    for (i = 1; i < M; i++)
      t = t->next;
    printf("%d ", t->next->key);
    x = t->next;
    t->next = t->next->next;
    free(x);
  }
  printf("%d\n", t->key);
}
