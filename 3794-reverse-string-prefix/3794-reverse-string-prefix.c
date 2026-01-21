#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char* reversePrefix(const char* s, int k) {
    int n = strlen(s);

    // 결과 문자열 메모리 할당 (+1은 '\0')
    char* res = (char*)malloc(n + 1);

    int idx = 0;

    // 1️⃣ 앞 k개 뒤집기
    for (int i = k - 1; i >= 0; i--) {
        res[idx++] = s[i];
    }

    // 2️⃣ 나머지 그대로 복사
    for (int i = k; i < n; i++) {
        res[idx++] = s[i];
    }

    res[idx] = '\0';
    return res;
}