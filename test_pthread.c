#include <pthread.h>
#include <stdio.h>

void* thread_function(void* arg) {
    printf("Thread running\n");
    return NULL;
}

int main() {
    pthread_t thread;
    pthread_create(&thread, NULL, thread_function, NULL);
    pthread_join(thread, NULL);
    printf("Main thread finished\n");
    return 0;
}
