//
// Created by 23591 on 2021/9/3.
//

int c_zero() {
    return 42;
}

extern int rs_zero();

int c_get_rs_value() {
    return rs_zero();
}