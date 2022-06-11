#include <stdio.h>
#include <stdint.h>
#include <setjmp.h>

int32_t current(void);
void increment(void);
void *call_with_counter(void *f(void *), void *data);

void *call_increment(void *ptr) {
    increment();
    return NULL;
}

void *call_longjmp(void *ptr) {
    longjmp(g_env, 1);
    return NULL;
}

int main(int argc, char **argv) {
    printf("N = %d\n", current());
    increment();
    printf("N = %d\n", current());

    call_with_counter(&call_increment, NULL);
    printf("N = %d\n", current());

    if (setjmp(g_env) == 0) {
        call_with_counter(&call_longjmp, NULL);
        printf("longjmp in't called\n");
    } else {
        printf("longjmp is called\n");
    }
    printf("N = %d\n", current());
}