#include <stdio.h>
#include <pthread.h>

void* thread_func(void* arg) {
    printf("Hello from thread!\n");
    return NULL;
}

int main() {
    pthread_t thread;
    pthread_create(&thread, NULL, thread_func, NULL);
    pthread_join(thread, NULL);
    printf("Thread completed!\n");
    return 0;
}
