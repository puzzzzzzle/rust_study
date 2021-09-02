//
// Created by 23591 on 2021/9/3.
//
#include <sys/time.h>
#include <stdio.h>
#include <unistd.h>


int test()
{

    struct timeval tv;
    gettimeofday(&tv, NULL);

    printf("second: %ld\n", tv.tv_sec); // 秒
    printf("millisecond: %ld\n", tv.tv_sec * 1000 + tv.tv_usec / 1000); // 毫秒
    printf("microsecond: %ld\n", tv.tv_sec * 1000000 + tv.tv_usec); // 徽秒

    sleep(3); // 让程序休眠3秒
    printf("---------------------sleep 3 second-------------------\n");

    gettimeofday(&tv, NULL);

    printf("second: %ld\n", tv.tv_sec); // 秒
    printf("millisecond: %ld\n", tv.tv_sec * 1000 + tv.tv_usec / 1000); // 毫秒
    printf("microsecond: %ld\n", tv.tv_sec * 1000000 + tv.tv_usec); // 徽秒

    return 0;
}

int c_zero() {
    return test();
}

extern int rs_zero();

int c_get_rs_value() {
    return rs_zero();
}