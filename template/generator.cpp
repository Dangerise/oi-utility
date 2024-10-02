#include <bits/stdc++.h>
#include <cassert>
#include <cstdio>
#include <cstdlib>
using namespace std;

#define int long long
// #define double long double
#define il inline
#define siz(x) ((int)x.size())
#define me(a, v) memset(a, v, sizeof(a))
#define cp(a, b) memcpy(b, a, (assert(sizeof(a) == sizeof(b)), sizeof(a)))
#define L(i, l, r) for (int i = l; i <= r; i++)
#define R(i, r, l) for (int i = r; i >= l; i--)
#define ve(a) vector<a>
#define pa(a, b) pair<a, b>
#define x first
#define y second
typedef double flt;
typedef string str;

il char gc() { return getchar(); }
il void pc(char c) { putchar(c); }
il void el() { pc('\n'); }
il int qread() {
    int ans = 0;
    char c = gc();
    bool f = 0;
    while (c < '0' || c > '9') {
        if (c == '-') {
            f = 1;
        }
        c = gc();
    }
    while (c >= '0' && c <= '9') {
        ans = ans * 10 + c - '0';
        c = gc();
    }
    if (f) {
        return -ans;
    } else {
        return ans;
    }
}

constexpr int N = 10, V = 10;

il int random(int l, int r) { return rand() * rand() % (r - l + 1) + l; }

signed main() {
    freopen("input.txt", "w", stdout);

    srand((int)time(0));

    return 0;
}
