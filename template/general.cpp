#include <bits/stdc++.h>
using namespace std;

#define int long long

inline int qread() {
    int ans = 0;
    char c = getchar();
    bool f = 0;
    while (c < '0' || c > '9') {
        if (c == '-') {
            f = 1;
        }
        c = getchar();
    }
    while (c >= '0' && c <= '9') {
        ans = ans * 10 + c - '0';
        c = getchar();
    }
    if (f) {
        return -ans;
    } else {
        return ans;
    }
}

const int INF = INT_MAX;
const int N = 0;

signed main() {
    // freopen("out", "w", stdout);

    return 0;
}
