#include <bits/stdc++.h>
using namespace std;

#define int long long
// #define double long double
#define il inline
#define siz(x) ((int)x.size())
#define me(a, v) memset(a, v, sizeof(a))
#define cp(a, b) memcpy(b, a, (assert(sizeof(a) == sizeof(b)), sizeof(a)))
#define L(i, l, r) for (int i = l; i <= r; i++)
#define R(i, r, l) for (int i = r; i >= l; i--)
#define x first
#define y second

#define ve(a) vector<a>
#define vep(a, b) vector<pair<a, b>>
#define pa(a, b) pair<a, b>
#define map(a, b) map<a, b>
#define set(a, b) set(a, b)
#define mmap(a, b) multimap<a, b>
#define mset(a, b) mset<a, b>
#define hmap(a, b) unordered_map<a, b>
#define hset(a, b) unordered_set<a, b>

typedef double flt;
typedef string str;

il char gc() { return getchar(); }
il void pc(char c) { putchar(c); }
il void el() { pc('\n'); }
il void bp() { puts("Fuck ccf !"); }
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

il int random(int l, int r) { return (int)rand() * rand() % (r - l + 1) + l; }

signed main() {
    freopen(".in", "w", stdout);

    srand((int)time(0));

    return 0;
}
