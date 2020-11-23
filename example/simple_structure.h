//
// Created by jack on 11/21/20.
//

#ifndef F_STACK_SIMPLE_STRUCTURE_H
#define F_STACK_SIMPLE_STRUCTURE_H

#include <semaphore.h>
#include <stdlib.h>
#include <memory.h>
struct mutex_stack_t {
    int size;
    int head;
    sem_t mutex;
    void **data;
};

void mutex_stack_init(struct mutex_stack_t *st, int size, int size_per_element) {
    st->size = size;
    st->head = 0;
    sem_init(&st->mutex, 0, 1);
    st->data = malloc(size * size_per_element);
    bzero(st->data, size * size_per_element);
}

void mutex_stack_push(struct mutex_stack_t *st, void *element) {
    sem_wait(&st->mutex);
    st->data[st->head++] = element;
    sem_post(&st->mutex);
}

void *mutex_stack_pop(struct mutex_stack_t *st) {
    sem_wait(&st->mutex);
    st->head -= 1;
    void *data = st->data[st->head];
    st->data[st->head] = NULL;
    sem_post(&st->mutex);
    return data;
}

int mutex_stack_size(struct mutex_stack_t *st) {
    sem_wait(&st->mutex);
    int size = st->head;
    sem_post(&st->mutex);
    return size;
}



#endif //F_STACK_SIMPLE_STRUCTURE_H
